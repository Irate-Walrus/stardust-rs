# Rust Position Independent Shellcode (PIC) Template for i686 & x86\_64 Linux & Windows

> [!warning]
> This is an experiment and I can personally guarantee it is unsafe. I describe below some of the unobvious (to me) issues I ended up facing.
> I'm keen to hear of any possible workarounds for these issues, just open a PR.

This code is based on the following previous work:
- https://bruteratel.com/research/feature-update/2021/01/30/OBJEXEC/
- https://5pider.net/blog/2024/01/27/modern-shellcode-implant-design/
- https://github.com/wumb0/rust_bof
- https://kerkour.com/rust-position-independent-shellcode
- https://github.com/safedv/Rustic64


Some github and rust-lang issues from the journey, thank you friends!:
- [Compiling `no_std` for `i686-pc-windows-gnu` ignores `panic=abort`](https://github.com/rust-lang/rust/issues/133826)
- [Inclusion of `-lkernel32` and others when compiling `#![no_std]` for i686-pc-windows-gnu](https://users.rust-lang.org/t/inclusion-of-lkernel32-and-others-when-compiling-no-std-for-i686-pc-windows-gnu/121551)
- [`i686-w64-mingw32-gcc` and relative data addressing (PIC)](https://users.rust-lang.org/t/i686-w64-mingw32-gcc-and-relative-data-addressing-pic/122399/9)

The following targets are supported.

| Target | Payload Size |
| --- | --- |
| `i686-linux` | 4141B |
| `x86_64-linux` | 4167B |
| `i686-windows` | 4141B |
| `x86_64-windows` | 4120B |

To build one of these targets use `cargo make -p $target build`

Following is the current output of `cargo make -p x86_64-linux run`:

```
***     [LOADER]        ***
[*] Allocate RW Memory
[*] Copy Shellcode Into RW Memory
[*] Set Memory RX
[*] Allocation Start Address:   0x700000000000
[*] Allocation End Address:     0x700000001047
[*] Allocation Size:            4167B

***     [STARDUST x86_64]       ***
[*] Hello Stardust!
[*] Stardust Start Address:     0x700000000000
[*] Stardust Length:            4167
[*] Stardust Instance:          0x7f785645f000
[*] Hitting Breakpoint!
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

This leads to a call being made to `_gcc_except_table` which has been removed by [linux.ld](./stardust/scripts/linux.ld) resulting in a segmentation fault.

> [!note]
> Patching the GOT appeared to get us a little further along before it crashes. YAY!🥳

**Solution**: None

## (Solved) Problem #2 - Global Offset Table (GOT)

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
let src = alloc::string::String::from("SSECCUS\t\t:ypcmem gnitseT");
let dst: String = src.chars().rev().collect();
info!(&dst);
```

Patching the hardcoded addresses with GDB results in a successful execution as seen below:

![Patching `memcpy` address in GOT with GDB](./docs/patching-memcpy-addr.png)

**Solution**:
- Patch the GOT dynamically during runtime, this appears to allow the use of `compiler_builtins`!
- Don't directly call `extern` functions before patching, call them within `asm!` macro

## (Solved) Problem #3 - i686 Windows and `-fPIC`

You're best off reading this (or maybe you're not, won't get that time back) [`i686-w64-mingw32-gcc` and relative data addressing (PIC)](https://users.rust-lang.org/t/i686-w64-mingw32-gcc-and-relative-data-addressing-pic/122399/9).

**Solution**:
- Compile an i686 elf
- Patch the GOT dynamically during runtime
- Specify `stdcall` where required.
- _"It's all just machine code in the end"_ - Me while justifying this mess
