# Rust PIC Template for x86\_64 Linux

> [!warning]
> This is/was an experiment which I may or may not revisit due to other priorities, described below are the issues I ended up facing.
> I'm keen to hear of any possible workarounds for these issues, just open a PR.


This is a PoC targeted at x64 linux and has numerous issues, it is based on the following previous work:
- https://bruteratel.com/research/feature-update/2021/01/30/OBJEXEC/
- https://5pider.net/blog/2024/01/27/modern-shellcode-implant-design/
- https://github.com/wumb0/rust_bof
- https://kerkour.com/rust-position-independent-shellcode
- https://github.com/safedv/Rustic64

The following targets are supported:
- `i686-unknown-linux-gnu`
- `x86_x64-unknown-linux-gnu`

> [!warning]
> At the moment `libc.so` is assumed to be compiled for 64-bit so while the `i686` target is supported it'll probably fail in practice.

Following is the current output of `cargo make run` with `TARGET="i686-unknown-linux-gnu"`.

```
***     [LOADER]        ***
[*] Allocate RW Memory
[*] Copy Shellcode Into RW Memory
[*] Set Memory RX
[*] Allocation Start Address:   0x700000000000
[*] Allocation End Address:     0x70000000310f
[*] Allocation Size:            12559B

***     [STARDUST]      ***
[*] Hello Stardust!
[*] Stardust Start Address:     0x700000000000
[*] Stardust End Address:       0x70000000310f
[*] Stardust Length:            12559
[*] Stardust Data Offset:       0x3000
[*] Stardust Data Address:      0x700000003000
[*] Stardust GOT Offset:        0x30c8
[*] Stardust GOT Address:       0x7000000030c8
[*] Stardust GOT Length:        56
[*] Stardust Instance:          0x7ff709964000
[*] Testing memcpy:             SUCCESS
[*] lib_base_addr:              0x7ff709738000
[*] Hello, world from write!
[*] HITTING BREAKPOINT
```

## Problem #1 - `format!` macro e.g. `&'static &str`

Using the `alloc::fmt::format!` macro will result in a segementation fault due to absolute addresses to reference the `pieces` in `Arguments { pieces, fmt: None, args }`.


This results in the `if !piece.is_empty()` check failing within the following code
@ [https://github.com/rust-lang/rust/blob/master/library/core/src/fmt/mod.rs](https://github.com/rust-lang/rust/blob/150247c338a54cb3d08614d8530d1bb491fa90db/library/core/src/fmt/mod.rs#L1172C1-L1190C10):

```rust
/* core::fmt::write () at core/src/fmt/mod.rs:1179 */
/* 1172 */ match args.fmt {
/* 1173 */     None => {
/* 1174 */         // We can use default formatting parameters for all arguments.
/* 1175 */         for (i, arg) in args.args.iter().enumerate() {
/* 1176 */             // SAFETY: args.args and args.pieces come from the same Arguments,
/* 1177 */             // which guarantees the indexes are always within bounds.
/* 1178 */             let piece = unsafe { args.pieces.get_unchecked(i) };
/* 1179 */             if !piece.is_empty() { // This is the check currently failing
/* 1180 */                 formatter.buf.write_str(*piece)?;
/* 1181 */             }
/* 1182 */
/* 1183 */             // SAFETY: There are no formatting parameters and hence no
/* 1184 */             // count arguments.
/* 1185 */             unsafe {
/* 1186 */                 arg.fmt(&mut formatter)?;
/* 1187 */             }
/* 1188 */             idx += 1;
/* 1189 */         }
/* 1190 */     }

```

This leads to a call being made to `_gcc_except_table` which has been removed by [linker.ld](./stardust/linker.ld) resulting in a segmentation fault.


**Solution**: None

## Problem #2 - Global Offset Table (GOT)

A bunch of stuff uses the GOT including calls to functions with the `compiler_builtins` crate, e.g. the following functions:
- `memcpy`
- `memmove`
- `memset`
- `memcmp`
- `bcmp`

This resulted in a segmentation fault due to a `call` made to a bad/absolute hard-coded memory address stored within the GOT and then referenced by a RIP-relative offset.

Similarly using `extern "C"` functions directly may result in the use the GOT.


The following code was used to ensure `memcpy` was required by the binary:

```rust
let src = alloc::string::String::from("Hello, world!");
let mut dst: String = src.chars().rev().collect();
dst.push('\n');
print(&dst);
```
Patching the hardcoded addresses with GDB results in a successful execution as seen below:

![Patching `memcpy` address in GOT with GDB](./docs/patching-memcpy-addr.png)

**Solution**:
- Patch the GOT dynamically during runtime, this appears to allow the use of `compiler_builtins`!
- Don't directly call `extern` functions before patching, call them within `asm!` macro

## Problem #4 `.bss.__rust_no_alloc_shim_is_unstable`

Haven't worked out what exactly this symbol is supposed to do, funnily enough memory allocations appeared to work fine without fiddling with it.

**Solution**: Ignore it until it breaks something
