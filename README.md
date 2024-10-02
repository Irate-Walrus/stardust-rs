# Unstable/Incomplete Rust PIC Template

> [!warning]
> This is/was an experiment which I may or may not revisit due to other priorities, described below are the issues I ended up facing.
> I'm keen to hear of any possible workarounds for these issues, just open a PR.


This is a PoC targeted at x64 linux and has numerous issues, it is based on the following previous work:
- https://bruteratel.com/research/feature-update/2021/01/30/OBJEXEC/
- https://5pider.net/blog/2024/01/27/modern-shellcode-implant-design/
- https://github.com/wumb0/rust_bof
- https://kerkour.com/rust-position-independent-shellcode

Following is the current output of `cargo make run`.

```
[+] Hello Stardust
[1]    104538 segmentation fault (core dumped)  ./target/x86_64-unknown-linux-gnu/debug/runner
```

Following is the desired output and current output of `cargo make run-nopic`

```
[+] Hello Stardust
[*] Stardust Start Address:     0x10050
[*] Stardust End Address:       0x15278
[*] Stardust Length:            21032B
[*] Stardust Data Offset:       0x16000
```

## Problem #1 - `format!` macro e.g. `&'static &str`

Using the `alloc::fmt::format!` macro will result in a segementation fault due to absolute addresses to reference the `pieces` in `Arguments { pieces, fmt: None, args }`.


This results in the `if !piece.is_empty()` check failting within the following code
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

## Problem #2 - Compiler Builtins

Using `alloc` appears to work but functionality that requires `compilier_builtins`, e.g. the following functions:
- `memcpy`
- `memmove`
- `memset`
- `memcmp`
- `bcmp`

Will result in a segmentation fault due to a `call` made to a bad/absolute hard-coded memory address stored in memory and referenced by a RIP-relative offset.

**Solution**: None

## Problem #3 - Global Offset Table

Calling `extern "C"` functions directly may result in the use of a Global Offset Table placed within the `.got` section.

The Global Offset Table contains absolute/hard-coded memory addresses which when called result in a segmentation fault.

**Solution**: Don't directly call `extern` functions (call them within `asm!` macro ;D)


## Problem #4 `.bss.__rust_no_alloc_shim_is_unstable`

Haven't worked out what exactly this symbol is supposed to do, funnily enough memory allocations appeared to work fine without including it in the shellcode.

**Solution**: Ignore it until it breaks something