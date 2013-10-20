A stub standard library for Rust. It will provide a baseline level of support
for freestanding Rust, and extended functionality based on the availability of
the standard C library, POSIX and OS-specific features.

Rust lacks support for static linking and whole program optimization, so the
`core` library is designed to be directly included as a module within a single
`#[no_std]` crate. When these limitations are fixed, it will become a regular
crate usable in a multi-crate project.

Configuration:

* If the C standard library is available, pass `--cfg libc` to `rustc`
