A lightweight standard library for Rust with freestanding support. It provides
a baseline level of functionality without any external dependencies, and an
extended set of features in a traditional hosted environment.

The `core` library is currently designed to be used as a module, but as Rust's
support for static linking and link-time optimization matures it will move
towards the standard crate model.

# Configuration

* `--cfg libc` to enable features depending on a C standard library implementation
* `--cfg debug` to enable debugging features (assertions)

# Building

Currently, building to bytecode with `--emit-llvm` and then compiling/linking
with `clang` is required because the Rust compiler cannot build code without
the requirement for segmented stack support from the runtime. There is also no
way to avoid position independent code and linking against the runtime without
making use of `clang`.

```
rustc -O --emit-llvm foo.rs
clang -O2 -flto -lm -lpthread -o foo foo.bc
```

As an additional problem, the Rust compiler assumes unwinding is used. Until
there is a way to [disable unwinding](https://github.com/mozilla/rust/issues/10780)
this will be extremely problematic. Rust will output code for running
destructors during table-based unwinding with a dependency on the runtime for
segmented stack support. LLVM can optimize most of this away thanks to
link-time optimization, but Rust provides no way to mark external functions as
not throwing. As soon as calls are made to external functions not hard-wired
into LLVM as `nounwind`, the ability to use Rust without the runtime breaks
down.

# Freestanding usage

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

# Stack safety

Ideally, stack safety is provided with one or more guard pages and compiler
support for inserting checks based on awareness of the guard pages. GCC has
this as [-fcheck-stack](http://gcc.gnu.org/onlinedocs/gccint/Stack-Checking.html)
but LLVM is missing the feature.

At the moment, `core` only has OS-provided guard pages without the necessary
checks on frames larger than the guard size.

Rust's standard library provides stack safety via LLVM's segmented stack
support, but this has a negative performance and code size impact. It's also
unavailable without the Rust runtime.

# Allocators

Allocators are not yet working due to [issue #4252](https://github.com/mozilla/rust/issues/4252).

The `core::mem::Allocator` trait defines the allocator interface. A generic
container takes an allocator type parameter, with `core::heap::Heap` as the
default allocator for a container constructed with the `new` static method.

A container can be constructed using `Container::with_alloc(allocator)`, and
will store the allocator instance internally. Since Rust has zero-size types,
this has no overhead for allocators with no instance state.

Sharing stateful allocator instances between containers can be done with
`core::rc` or `core::arc . Containers are already expensive to clone, so
a reference count on container copies shouldn't be an issue.
