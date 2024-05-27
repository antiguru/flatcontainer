//! A region that encodes its contents.

use crate::{OwnedRegion, Push, Region};

pub use self::misra_gries::MisraGries;
pub use dictionary::DictionaryCodec;

// TODO: Consolidation comes from Differential.

/// Sorts and consolidates `vec`.
///
/// This method will sort `vec` and then consolidate runs of more than one entry with
/// identical first elements by accumulating the second elements of the pairs. Should the final
/// accumulation be zero, the element is discarded.
fn consolidate<T: Ord>(vec: &mut Vec<(T, usize)>) {
    consolidate_from(vec, 0);
}

/// Sorts and consolidate `vec[offset..]`.
///
/// This method will sort `vec[offset..]` and then consolidate runs of more than one entry with
/// identical first elements by accumulating the second elements of the pairs. Should the final
/// accumulation be zero, the element is discarded.
fn consolidate_from<T: Ord>(vec: &mut Vec<(T, usize)>, offset: usize) {
    let length = consolidate_slice(&mut vec[offset..]);
    vec.truncate(offset + length);
}

/// Sorts and consolidates a slice, returning the valid prefix length.
fn consolidate_slice<T: Ord>(slice: &mut [(T, usize)]) -> usize {
    if slice.len() > 1 {
        // We could do an insertion-sort like initial scan which builds up sorted, consolidated runs.
        // In a world where there are not many results, we may never even need to call in to merge sort.
        slice.sort_by(|x, y| x.0.cmp(&y.0));

        // Counts the number of distinct known-non-zero accumulations. Indexes the write location.
        let mut offset = 0;
        let mut accum = slice[offset].1;

        for index in 1..slice.len() {
            if slice[index].0 == slice[index - 1].0 {
                accum += slice[index].1;
            } else {
                if accum != 0 {
                    slice.swap(offset, index - 1);
                    slice[offset].1.clone_from(&accum);
                    offset += 1;
                }
                accum.clone_from(&slice[index].1);
            }
        }
        if accum != 0 {
            slice.swap(offset, slice.len() - 1);
            slice[offset].1 = accum;
            offset += 1;
        }

        offset
    } else {
        slice.iter().filter(|x| x.1 != 0).count()
    }
}

/// A region that encodes its data in a codec `C`.
#[derive(Default, Debug, Clone)]
pub struct CodecRegion<C: Codec, R = OwnedRegion<u8>> {
    inner: R,
    codec: C,
}

impl<C: Codec, R> Region for CodecRegion<C, R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
{
    type ReadItem<'a> = &'a [u8]
    where
        Self: 'a;

    type Index = R::Index;

    /// Construct a region that can absorb the contents of `regions` in the future.
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        let codec = C::new_from(regions.clone().map(|r| &r.codec));
        Self {
            inner: R::merge_regions(regions.map(|r| &r.inner)),
            codec,
        }
    }

    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        self.codec.decode(self.inner.index(index))
    }

    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.inner.reserve_regions(regions.map(|r| &r.inner));
    }

    fn clear(&mut self) {
        self.codec = Default::default();
    }

    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        self.inner.heap_size(&mut callback);
        self.codec.heap_size(callback);
    }

    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        C: 'a,
    {
        item
    }
}

impl<C: Codec, R> Push<&[u8]> for CodecRegion<C, R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + Push<&'a [u8]> + 'a,
{
    fn push(&mut self, item: &[u8]) -> <CodecRegion<C, R> as Region>::Index {
        self.codec.encode(item, &mut self.inner)
    }
}

/// Encode and decode byte strings.
pub trait Codec: Default {
    /// Decodes an input byte slice into a sequence of byte slices.
    fn decode<'a>(&'a self, bytes: &'a [u8]) -> &'a [u8];
    /// Encodes a sequence of byte slices into an output byte slice.
    fn encode<R>(&mut self, bytes: &[u8], output: &mut R) -> R::Index
    where
        for<'a> R: Region + Push<&'a [u8]>;
    /// Constructs a new instance of `Self` from accumulated statistics.
    /// These statistics should cover the data the output expects to see.
    fn new_from<'a, I: Iterator<Item = &'a Self> + Clone>(stats: I) -> Self
    where
        Self: 'a;
    /// Diagnostic information about the state of the codec.
    fn report(&self) {}

    /// Heap size, size - capacity
    fn heap_size<F: FnMut(usize, usize)>(&self, callback: F);
}

mod dictionary {

    use crate::{Push, Region};
    use std::collections::BTreeMap;

    pub use super::{BytesMap, Codec, MisraGries};

    /// A type that can both encode and decode sequences of byte slices.
    #[derive(Default, Debug)]
    pub struct DictionaryCodec {
        encode: BTreeMap<Vec<u8>, u8>,
        decode: BytesMap,
        stats: (MisraGries<Vec<u8>>, [u64; 4]),
        bytes: usize,
        total: usize,
    }

    impl Codec for DictionaryCodec {
        /// Decode a sequence of byte slices.
        fn decode<'a>(&'a self, bytes: &'a [u8]) -> &'a [u8] {
            if let Some(bytes) = self.decode.get(bytes[0].into()) {
                bytes
            } else {
                bytes
            }
        }

        /// Encode a sequence of byte slices.
        ///
        /// Encoding also records statistics about the structure of the input.
        fn encode<R>(&mut self, bytes: &[u8], output: &mut R) -> R::Index
        where
            for<'a> R: Region + Push<&'a [u8]>,
        {
            self.total += bytes.len();
            // If we have an index referencing `bytes`, use the index key.
            let index = if let Some(b) = self.encode.get(bytes) {
                self.bytes += 1;
                output.push([*b].as_slice())
            } else {
                self.bytes += bytes.len();
                output.push(bytes)
            };
            // Stats stuff.
            self.stats.0.insert(bytes.to_owned());
            let tag = bytes[0];
            let tag_idx: usize = (tag % 4).into();
            self.stats.1[tag_idx] |= 1 << (tag >> 2);

            index
        }

        /// Construct a new encoder from supplied statistics.
        fn new_from<'a, I: Iterator<Item = &'a Self> + Clone>(stats: I) -> Self {
            // Collect most popular bytes from combined containers.
            let mut mg = MisraGries::default();
            for (thing, count) in stats.clone().flat_map(|stats| stats.stats.0.clone().done()) {
                mg.update(thing, count);
            }
            let mut mg = mg.done().into_iter();
            // Establish encoding and decoding rules.
            let mut encode = BTreeMap::new();
            let mut decode = BytesMap::default();
            for tag in 0..=255 {
                let tag_idx: usize = (tag % 4).into();
                let shift = tag >> 2;
                let or = stats
                    .clone()
                    .fold(0, |acc, stats| acc | stats.stats.1[tag_idx]);
                if (or >> shift) & 0x01 != 0 {
                    decode.push(None);
                } else if let Some((next_bytes, _count)) = mg.next() {
                    decode.push(Some(&next_bytes[..]));
                    encode.insert(next_bytes, tag);
                }
            }

            Self {
                encode,
                decode,
                stats: (MisraGries::default(), [0u64; 4]),
                bytes: 0,
                total: 0,
            }
        }

        fn report(&self) {
            let mut tags_used = 0;
            tags_used += self.stats.1[0].count_ones();
            tags_used += self.stats.1[1].count_ones();
            tags_used += self.stats.1[2].count_ones();
            tags_used += self.stats.1[3].count_ones();
            let mg = self.stats.0.clone().done();
            let mut bytes = 0;
            for (vec, _count) in &mg {
                bytes += vec.len();
            }
            // if self.total > 10000 && !mg.is_empty() {
            println!(
                "\t{:?}v{:?}: {:?} -> {:?} + {:?} = (x{:?})",
                tags_used,
                mg.len(),
                self.total,
                self.bytes,
                bytes,
                self.total / (self.bytes + bytes),
            );
            // }
        }

        fn heap_size<F: FnMut(usize, usize)>(&self, _callback: F) {
            // Lazy
        }
    }
}

/// A map from `0 .. something` to `Option<&[u8]>`.
///
/// Non-empty slices are pushed in order, and can be retrieved by index.
/// Pushing an empty slice is equivalent to pushing `None`.
#[derive(Debug)]
pub struct BytesMap {
    offsets: Vec<usize>,
    bytes: Vec<u8>,
}
impl Default for BytesMap {
    fn default() -> Self {
        Self {
            offsets: vec![0],
            bytes: Vec::new(),
        }
    }
}
impl BytesMap {
    fn push(&mut self, input: Option<&[u8]>) {
        if let Some(bytes) = input {
            self.bytes.extend(bytes);
        }
        self.offsets.push(self.bytes.len());
    }
    fn get(&self, index: usize) -> Option<&[u8]> {
        if index < self.offsets.len() - 1 {
            let lower = self.offsets[index];
            let upper = self.offsets[index + 1];
            if lower < upper {
                Some(&self.bytes[lower..upper])
            } else {
                None
            }
        } else {
            None
        }
    }
    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.offsets.len() - 1
    }
}

mod misra_gries {

    /// Maintains a summary of "heavy hitters" in a presented collection of items.
    #[derive(Clone, Debug)]
    pub struct MisraGries<T> {
        inner: Vec<(T, usize)>,
    }

    impl<T> Default for MisraGries<T> {
        fn default() -> Self {
            Self {
                inner: Vec::with_capacity(1024),
            }
        }
    }

    impl<T: Ord> MisraGries<T> {
        /// Inserts an additional element to the summary.
        pub fn insert(&mut self, element: T) {
            self.update(element, 1);
        }
        /// Inserts multiple copies of an element to the summary.
        pub fn update(&mut self, element: T, count: usize) {
            self.inner.push((element, count));
            if self.inner.len() == self.inner.capacity() {
                self.tidy();
            }
        }
        /// Allocates a Misra-Gries summary which intends to hold up to `k` examples.
        ///
        /// After `n` insertions it will contain only elements that were inserted at least `n/k` times.
        /// The actual memory use is proportional to `2 * k`, so that we can amortize the consolidation.
        #[must_use]
        pub fn with_capacity(k: usize) -> Self {
            Self {
                inner: Vec::with_capacity(2 * k),
            }
        }

        /// Completes the summary, and extracts the items and their counts.
        #[must_use]
        pub fn done(mut self) -> Vec<(T, usize)> {
            use super::consolidate;
            consolidate(&mut self.inner);
            self.inner.sort_by(|x, y| y.1.cmp(&x.1));
            self.inner
        }

        /// Internal method that reduces the summary down to at most `k-1` distinct items, by repeatedly
        /// removing sets of `k` distinct items. The removal is biased towards the lowest counts, so as
        /// to preserve fidelity around the larger counts, for whatever that is worth.
        fn tidy(&mut self) {
            use super::consolidate;
            consolidate(&mut self.inner);
            self.inner.sort_by(|x, y| y.1.cmp(&x.1));
            let k = self.inner.capacity() / 2;
            if self.inner.len() > k {
                let sub_weight = self.inner[k].1 - 1;
                self.inner.truncate(k);
                for (_, weight) in &mut self.inner {
                    *weight -= sub_weight;
                }
                while self.inner.last().map(|x| x.1) == Some(0) {
                    self.inner.pop();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Codec, CodecRegion, DictionaryCodec};
    use crate::*;

    #[test]
    fn test_simple() {
        let mut r = CodecRegion::<DictionaryCodec>::default();

        for _ in 0..1000 {
            let index = r.push("abc".as_bytes());
            assert_eq!("abc".as_bytes(), r.index(index));
        }

        let mut r2 = CodecRegion::default();

        for _ in 0..1000 {
            let index = r2.push("abc".as_bytes());
            assert_eq!("abc".as_bytes(), r2.index(index));
        }

        let mut r3 = CodecRegion::merge_regions([&r, &r2].into_iter());

        for _ in 0..2000 {
            let index = r3.push("abc".as_bytes());
            assert_eq!("abc".as_bytes(), r3.index(index));
        }

        println!("new container after inserts:");
        r.codec.report();
        println!("second new container after inserts:");
        r2.codec.report();
        println!("new container with merged stats after inserts:");
        r3.codec.report();
    }

    #[test]
    fn test_multi() {
        let mut regions = Vec::new();
        for _ in 0..8 {
            regions.push(CodecRegion::<DictionaryCodec>::default());
        }
        for _ in 0..1000 {
            for r in &mut regions {
                r.push("abcdef".as_bytes());
                r.push("defghi".as_bytes());
            }
        }

        let mut merged = CodecRegion::merge_regions(regions.iter());

        for _ in 0..2000 {
            let index = merged.push("abcdef".as_bytes());
            assert_eq!("abcdef".as_bytes(), merged.index(index));
            let index = merged.push("defghi".as_bytes());
            assert_eq!("defghi".as_bytes(), merged.index(index));
        }

        println!("new container with merged stats after inserts:");
        merged.codec.report();
    }

    #[test]
    fn test_heap_size() {
        let mut regions = Vec::new();
        for _ in 0..8 {
            regions.push(CodecRegion::<DictionaryCodec>::default());
        }
        for _ in 0..1000 {
            for r in &mut regions {
                r.push("abcdef".as_bytes());
                r.push("defghi".as_bytes());
            }
        }

        let mut merged = CodecRegion::merge_regions(regions.iter());

        for _ in 0..2000 {
            let index = merged.push("abcdef".as_bytes());
            assert_eq!("abcdef".as_bytes(), merged.index(index));
            let index = merged.push("defghi".as_bytes());
            assert_eq!("defghi".as_bytes(), merged.index(index));
        }

        let (mut size, mut cap, mut cnt) = (0, 0, 0);
        merged.heap_size(|siz, ca| {
            size += siz;
            cap += ca;
            cnt += 1;
        });

        assert!(cnt > 0);

        let mut merged2 = CodecRegion::merge_regions(std::iter::once(&merged));

        for _ in 0..2000 {
            let index = merged2.push("abcdef".as_bytes());
            assert_eq!("abcdef".as_bytes(), merged2.index(index));
            let index = merged2.push("defghi".as_bytes());
            assert_eq!("defghi".as_bytes(), merged2.index(index));
        }

        let (mut size, mut cap, mut cnt) = (0, 0, 0);
        merged2.heap_size(|siz, ca| {
            size += siz;
            cap += ca;
            cnt += 1;
        });
        assert!(cnt > 0);
    }
}
