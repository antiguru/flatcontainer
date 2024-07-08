flatcontainer
=======

A flat container for Rust.

```toml
[dependencies]
flatcontainer = "0.4"
```

## Example

```rust
use flatcontainer::FlatStack;

let r: Result<_, u16> = Ok("abc");
let mut c = FlatStack::default_impl::<Result<&str, u16>>();
c.copy(&r);
assert_eq!(r, c.get(0));
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
references to said types. A region's write-half should be implemented using the
[`Push`] trait.

Regions permit data access through opaque indexes, which the caller is responsible
for memorizing. An index grants access to the region-specific data representation,
and although it might be inspected, it should not be minted or modified in any
way. As far as a region is concerned, an index is an opaque type with no particular
meaning attached to it, unless specified differently by the region.

Regions roughly follow two patterns: Either they fan out to other regions, or they
behave as terminal nodes and explicitly contain storage. A [`Result`][ResultRegion] region
dispatches to an ok and error region, and uses its index type to distinguish where
to look up a value. A region for slices has storage to remember slices of indexes
where the index can be used to look up the datum's representation in an inner
region.

`flatcontainer` provides the [`FlatStack`] type, an exemplary implementation of how to
implement a collection of items that supports pushing additional elements,
and retrieval by an index to previously pushed elements. It can be used in many
places that simply want to use a flat data representation, but it leaves potential
for optimization behind. Specifically, indexes, although opaque, follow a
simple structure where a more efficient storage can save memory instead of blindly
writing down all values.

All region implementations should be considered examples on how to implement specific
regions that are tailored to the needs of the type, and characteristics of the data
encountered in practice. This crate provides a [`RegionPreference`] trait to let types
express their suggested region, but users can select a different region, as long as
it is compatible with the types to be stored. For example, a vector suggests to use
a slice-based region, but a client might know that the inner type is copy, and hence
better uses a sliced-based region that does not fan out to a region for the individual
elements.

## Safety

This crate is safe to use, and all unsafe code can be explained locally.
At the moment, this is only for assuming that utf-8 data is correct, which is true by
construction.

## Features

The `serde` feature controls whether types implement support for serializing and deserializing
data. Enabled by default.

## Performance and design considerations

A goal of flatcontainer is to store `O(n)` objects in less than `O(n)` allocations,
for example in `O(log n)`, or if pre-sized correctly, in constant allocations. It
can achieve this by laying out data densely in memory. This comes with benefits, and
restrictions. It allows fast sequential access as the CPU can prefetch data, and it
avoids loading data into caches that is not needed. Reducing the number of allocations
limits chatter with the allocator. On the flip side, the regions are append-only, and
copying a value destructs its original form. Reading owned data requires copying or
cloning.

Flatcontainer's region abstraction requires the user to store indexes. Without specific
knowledge about a region, it's likely the index is best stored in a vector. In many cases,
we know more about the data, and can use a better approach to storing indexes.

* Regions storing slices often have an index that looks like `(start, end)`. It is easy
  to observe that the previous element's end is equal to the current element's start,
  so it should only be stored once. This can reduce the size of the index per element from
  16 to 8 bytes.
* For index values that fit in 32 bits, we only need 4 bytes in memory. We can specialize
  a container for indexes that uses `u32` to store values smaller than 2^32, and use `u64`
  otherwise.
* The index into regions storing constant-sized elements often looks like strided numbers,
  e.g., 0, 2, 4, 8, ..., which we can represent using constant memory by remembering the
  stride and length. We can extend this to storing a tail of elements equals to the last
  stride by adding another count. Such an index container uses 0 bits in the limit!
* Consult the [index] module for types specialized to storing indexes using less bits.

Flatcontainer provides some implementations of these concepts. To merge adjacent start-end
pairs, wrap a region in a [`ConsecutiveIndexPairs`] region. It stores indexes and presents
outwards as a dense sequence of 0, 1, 2, ...

A [`CollapseSequence`] region remembers the index of the last element, and if a new element
equals the last, it'll return the previous index again instead of storing the same element
multiple times. This is limited to the direct predecessor, because otherwise storing
indexes and scanning through previous elements would be too expensive.

[index]: impls::index
[`ConsecutiveIndexPairs`]: impls::deduplicate::ConsecutiveIndexPairs
[`CollapseSequence`]: impls::deduplicate::CollapseSequence

## Comparison to columnation

Flatcontainer takes several ideas from [`columnation`](https://github.com/frankmcsherry/columnation).
It uses the concept of regions to abstract dense allocations while still identifying
individual objects. Where columnation returns owned objects that need to be treated as
if they were references, flatcontainer returns an opaque index, which a region can
translate to a lifetimed type. Columnation reads and writes the same type, which forces
it to accept owned data and present look-alike data outwards, forcing the user to be
careful when dropping data. Flatcontainer avoids these issues because indexes do not
contain pointers to owned data.

Flatcontainer offers similar performance to columnation, and is faster in some
situations. For storing strings, flatcontainer provides an index of `(start, end)`
to reconstruct the original input as a reference, while columnation returns an
owned string, which roughly looks like `(pointer, size, capacity)`. Instead of 24 bytes
overhead on a 64-bit CPU, we have 16 byte overhead. We can reduce the 16 byte overhead
to 8 by observing that consecutive entries start at the previous' end, allowing us to
reconstruct the end by looking at the next element's start.

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
