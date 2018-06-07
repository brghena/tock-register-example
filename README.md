Tock Register Example
=====================

An example project that uses Tock registers externally from Tock. The example
in `main.rs` is meaningless, but does compile.

`mmio_regs.rs` contains an example of extending Tock registers to perform
additional actions. In this case, it provides an abstraction that allows
CPU registers to be handled just like MMIO registers.

To compile, run: `cargo build`

