//! Simple deduplication of equal consecutive items.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::impls::offsets::OffsetContainer;
use crate::{Push, Region};

/// A region to deduplicate consecutive equal items.
///
/// # Examples
///
/// The following example shows that two inserts can result in the same index.
/// ```
/// use flatcontainer::impls::deduplicate::CollapseSequence;
/// use flatcontainer::{Push, StringRegion};
/// let mut r = <CollapseSequence<StringRegion>>::default();
///
/// assert_eq!(r.push("abc"), r.push("abc"));
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CollapseSequence<R: Region> {
    /// Inner region.
    inner: R,
    /// The index of the last pushed item.
    last_index: Option<R::Index>,
}

impl<R: Region> Default for CollapseSequence<R> {
    fn default() -> Self {
        Self {
            inner: R::default(),
            last_index: None,
        }
    }
}

impl<R: Region> Region for CollapseSequence<R> {
    type Owned = R::Owned;
    type ReadItem<'a> = R::ReadItem<'a> where Self: 'a;
    type Index = R::Index;

    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            inner: R::merge_regions(regions.map(|r| &r.inner)),
            last_index: None,
        }
    }

    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        self.inner.index(index)
    }

    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.inner.reserve_regions(regions.map(|r| &r.inner));
    }

    fn clear(&mut self) {
        self.inner.clear();
        self.last_index = None;
    }

    fn heap_size<F: FnMut(usize, usize)>(&self, callback: F) {
        self.inner.heap_size(callback);
    }

    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        R::reborrow(item)
    }
}

impl<R, T> Push<T> for CollapseSequence<R>
where
    R: Region + Push<T>,
    for<'a> T: PartialEq<R::ReadItem<'a>>,
{
    fn push(&mut self, item: T) -> <CollapseSequence<R> as Region>::Index {
        if let Some(last_index) = self.last_index {
            if item == self.inner.index(last_index) {
                return last_index;
            }
        }
        let index = self.inner.push(item);
        self.last_index = Some(index);
        index
    }
}

/// Transform an index of `(usize, usize)` to a sequence of `0..`. Requires the pairs to
/// be dense, i.e., `(i, j)` is followed by `(j, k)`.
///
/// Defers to region `R` for storing items, and uses offset container `O` to
/// remeber indices. By default, `O` is `Vec<usize>`.
///
/// # Examples
///
/// The following example shows that two inserts into a copy region have a collapsible index:
/// ```
/// use flatcontainer::impls::deduplicate::{CollapseSequence, ConsecutiveOffsetPairs};
/// use flatcontainer::{Push, OwnedRegion, Region, StringRegion};
/// let mut r = <ConsecutiveOffsetPairs<OwnedRegion<u8>>>::default();
///
/// let index: usize = r.push(&b"abc");
/// assert_eq!(b"abc", r.index(index));
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConsecutiveOffsetPairs<R, O = Vec<usize>>
where
    R: Region<Index = (usize, usize)>,
    O: OffsetContainer<usize>,
{
    /// Wrapped region
    inner: R,
    /// Storage for offsets. Always stores element 0.
    offsets: O,
    /// The most recent end of the index pair of region `R`.
    last_index: usize,
}

impl<R: Region<Index = (usize, usize)>, O: OffsetContainer<usize>> Default
    for ConsecutiveOffsetPairs<R, O>
{
    #[inline]
    fn default() -> Self {
        let mut d = Self {
            inner: Default::default(),
            offsets: Default::default(),
            last_index: 0,
        };
        d.offsets.push(0);
        d
    }
}

impl<R, O> Region for ConsecutiveOffsetPairs<R, O>
where
    R: Region<Index = (usize, usize)>,
    O: OffsetContainer<usize>,
{
    type Owned = R::Owned;
    type ReadItem<'a> = R::ReadItem<'a>
    where
        Self: 'a;

    type Index = usize;

    #[inline]
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        let mut offsets = O::default();
        offsets.push(0);
        Self {
            inner: R::merge_regions(regions.map(|r| &r.inner)),
            offsets,
            last_index: 0,
        }
    }

    #[inline]
    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        self.inner
            .index((self.offsets.index(index), self.offsets.index(index + 1)))
    }

    #[inline]
    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.inner.reserve_regions(regions.map(|r| &r.inner));
    }

    #[inline]
    fn clear(&mut self) {
        self.last_index = 0;
        self.inner.clear();
        self.offsets.clear();
        self.offsets.push(0);
    }

    #[inline]
    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        self.offsets.heap_size(&mut callback);
        self.inner.heap_size(callback);
    }

    #[inline]
    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        R::reborrow(item)
    }
}

impl<R, O, T> Push<T> for ConsecutiveOffsetPairs<R, O>
where
    R: Region<Index = (usize, usize)> + Push<T>,
    O: OffsetContainer<usize>,
{
    #[inline]
    fn push(&mut self, item: T) -> <ConsecutiveOffsetPairs<R, O> as Region>::Index {
        let index = self.inner.push(item);
        debug_assert_eq!(index.0, self.last_index);
        self.last_index = index.1;
        self.offsets.push(index.1);
        self.offsets.len() - 2
    }
}

#[cfg(test)]
mod tests {
    use crate::impls::deduplicate::{CollapseSequence, ConsecutiveOffsetPairs};
    use crate::impls::offsets::OffsetOptimized;
    use crate::{FlatStack, Push, StringRegion};

    #[test]
    fn test_dedup_flatstack() {
        let mut fs = FlatStack::<CollapseSequence<StringRegion>>::default();

        fs.copy("abc");
        fs.copy("abc");

        assert_eq!(2, fs.len());

        println!("{fs:?}");
    }

    #[test]
    fn test_dedup_region() {
        let mut r = CollapseSequence::<StringRegion>::default();

        assert_eq!(r.push("abc"), r.push("abc"));

        println!("{r:?}");
    }

    #[test]
    fn test_offset_optimized() {
        let mut r =
            CollapseSequence::<ConsecutiveOffsetPairs<StringRegion, OffsetOptimized>>::default();

        for _ in 0..1000 {
            let _ = r.push("abc");
        }

        println!("{r:?}");
    }
}
