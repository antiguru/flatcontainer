flatcontainer
=======

A flat container for Rust.

```toml
[dependencies]
flatcontainer = "0.1"
```

## Example

```rust
use flatcontainer::{FlatStack, CopyOnto};
fn main() {
  let r: Result<_, u16> = Ok("abc");
  let mut c = FlatStack::default_impl::<Result<&str, u16>>();
  let idx = c.copy(&r);
  assert_eq!(r, c.index(idx));
}
```

## Details

`flatcontainer` is a library that provides abstractions and implementations for
flattening collections of nested data structures into dense allocations. At its
core, a `Region` trait describes how to represent data in memory, with the option
to extract a lifetimed representation of the original data, which can be different
from what the caller had initially.

`flatcontainer` decouples the write-half of a container from its storage and read
interface to permit a multitude of types to be presented to the same region. For
example, a region containing string-like objects and only promising access to
`&str` can accept anything that looks like a string, i.e., `String`, `&str` and
references to said types.

Regions permit data access through opaque indexes, which the caller is responsible
for memorizing. An index grants access to the region-specific data representation,
and although it might be inspected, it should not be minted or modified in any
way. As far as a region is concerned, an index is an opaque type with no particular
meaning attached to it, unless specified differently by the region.

Regions roughly follow two patterns: Either they fan out to other regions, or they
behave as terminal nodes and explicitly contain storage. A `Result` region
dispatches to an ok and error region, and uses its index type to distinguish where
to look up a value. A region for slices has storage to remember slices of indexes
where the index can be used to look up the datum's representation in an inner
region.

`flatcontainer` provides `FlatStack`, an exemplary implementation of how to
implement a collection of items that supports pushing additional elements,
and retrieval by offset of previously pushed elements. It can be used in many
places that simply want to use a flat data representation, but it leaves potential
for optimization behind. Specifically, indexes, although opaque, follow a
simple structure where a more efficient storage can save memory instead of blindly
writing down all values.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
