# Unstable/Incomplete Rust PIC Template

This is a PoC targeted at x64 linux and has numerous issues, it is based on the following previous work:
- https://bruteratel.com/research/feature-update/2021/01/30/OBJEXEC/
- https://5pider.net/blog/2024/01/27/modern-shellcode-implant-design/
- https://github.com/wumb0/rust_bof
- https://kerkour.com/rust-position-independent-shellcode

Following is the current output of `cargo make run`.

```
[+] Hello Stardust
[1]    37573 segmentation fault (core dumped)  ./target/x86_64-unknown-linux-gnu/debug/runner
```

Following is the desired output and current output of `cargo make run-nopic`

```
[+] Hello Stardust
[*] Stardust Start Address:     0x21e7e0
[*] Stardust End Address:       0x21e815
[*] Stardust Length:            53
```

Using `alloc` appears to work but functionality that requires `compilier_builtins`, e.g. the following functions:
- `memcpy`
- `memmove`
- `memset`
- `memcmp`
- `bcmp`

Will result in a seg fault, an example of this is the `format!` macro. This seg fault appears to be the result of a failed `test rdx, rdx` within `core::fmt::write::hbd7fc918960f6ce7` resulting in a call to the `_gcc_except_table` which has been removed by [linker.ld](./stardust/linker.ld).


Offending dissassembly in `radare2` see `0x00001525` for seg fault:

![seg fault in `core::fmt::write::hbd7fc918960f6ce7`](./docs/segfault-in-core-fmt.png)

Seg fault as observed in `GDB`:

![GDB seg fault](./docs/gdb-debug-segfault.png)

If I debug the elf without the linker script you can see that this is a result of a failed null ptr check within `if !piece.is_empty()`:

![failed null ptr check](./docs/piece-is-empty.raw.png)

