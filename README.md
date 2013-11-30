A stub standard library for Rust. It will provide a baseline level of support
for freestanding Rust, and extended functionality based on the availability of
the standard C library, POSIX and OS-specific features.

Rust lacks support for static linking and whole program optimization, so the
`core` library is designed to be directly included as a module within a single
`#[no_std]` crate. When these limitations are fixed, it will become a regular
crate usable in a multi-crate project.

In the short term, this project aims to ease the use of Rust in freestanding
and real-time use cases. Identifying any language issues specific to these
niches is important.

The long-term goal is for the necessary changes to be adopted by the Rust
standard library to ease the maintenance burden. However, exploring the area in
a separate repository is easier.

# Configuration

* `--cfg libc` to enable features depending on a C standard library implementation
* `--cfg debug` to enable debugging features (assertions)

# Building

Currently, building to bytecode with `--emit-llvm` and then compiling/linking
with `clang` is recommended. The Rust compiler is missing switches like
`-ffreestanding`, `-fno-builtin` and a way to avoid linking in support
libraries. Using `clang` also allows whole program optimization across a mixed
Rust and C codebase.

# C standard library

Support for the C11 standard is currently assumed, and workarounds can be done
on a case-by-case basis. Functionality from C will be reused wherever it makes
sense.

# Freestanding

For freestanding use, simply omit the `libc` configuration switch.

LLVM will emit calls to `memcpy`, `memmove` and `memset`. Until [the
fix](https://github.com/mozilla/rust/pull/9945) for symbol visibility lands,
these must be provided by the auxilliary `support.rs` module.

The `support.rs` module must be compiled with `rustc --lib --emit-llvm -passes
inline` and then linked against the bytecode for the main module with `clang
-flto -O2 main.bc support.bc`.

The `inline` pass *must* be run separately to due to
[issue #10116](https://github.com/mozilla/rust/issues/10116) or LLVM will
generate infinitely recursive functions.

# Unwinding and out-of-memory

The library currently makes use of `abort` in out-of-memory conditions like the
Rust standard library. Some errors dealt with using linked failure in the Rust
standard library are also currently dealt with using abort.

Unwinding in these cases can become a configuration flag after threads are
exposed and some very minor work to preserve safety during vector reallocations
is done.

# Stack safety

The operating system is currently relied upon to provide some level of stack
safety. On systems with an MMU, a guard page at the end of each thread's stack
is a viable solution (glibc adds this by default). LLVM support would need to
be implemented to insert checks in functions with large stack frames.

The function prelude stack space check used to provide support for segmented
stacks would work fine on a system with no MMU.

# Allocators

Containers with support for custom allocators will require
[issue #4252](https://github.com/mozilla/rust/issues/4252) to be fixed, but
the initial design is already worked out.

The `core::mem::Allocator` trait defines the allocator interface. A generic
container takes an allocator type parameter, with `core::heap::Heap` as the
default allocator for a container constructed with the `new` static method.

A container can be constructed using `Container::with_alloc(allocator)`, and
will store the allocator instance internally. Since Rust has zero-size types,
this has no overhead for allocators with no instance state.

Sharing stateful allocator instances between containers can be done with
`RcMut` or unsafely with an `*mut` pointer. Containers are already expensive to
clone, so reference counting shouldn't be much of an issue.
