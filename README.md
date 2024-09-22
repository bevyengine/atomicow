<div class="rustdoc-hidden">

# `atomicow`

</div>

![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/atomicow.svg)](https://crates.io/crates/atomicow)
[![Downloads](https://img.shields.io/crates/d/atomicow.svg)](https://crates.io/crates/atomicow)
[![Docs](https://docs.rs/atomicow/badge.svg)](https://docs.rs/atomicow/latest/atomicow/)

A [`Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html)-like data structure where owned data is stored inside an [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html).
Here's what it looks like:

```rust, ignore
pub enum CowArc<'a, T: ?Sized + 'static> {
    Borrowed(&'a T),
    Static(&'static T),
    Owned(Arc<T>),
}
```

As implied by the `Cow` name, this type allows for cheap immutable reference, but can be converted into a shared owned form via cloning when lifetime extension is required.

This data structure is particularly useful for `str` or other values with a static lifetime,
as might be used in structures such as asset paths.
A `'static str` stored in a `CowArc` can be cloned without allocations or bookkeeping,
while owned values are shared by reference-counting in a thread-safe fashion.

## Comparison to the `cow_arc` crate

The similar [`cow_arc`](https://docs.rs/cow_arc/latest/cow_arc/) crate already exists.
How does `atomicow` differ?

Put simply: `cow_arc`'s data structure is just a wrapper over an `Arc`.
While this is exactly what you need in some use cases,
the enum structure used in `atomicow` is both more transparent and more flexible.

## Contributing

This crate is maintained by the Bevy organization, and is intended to be tiny, stable, zero-dependency, and broadly useful.
[Issues](https://github.com/bevyengine/atomicow/issues) and [pull requests](https://github.com/bevyengine/atomicow/pulls) are genuinely welcome!
