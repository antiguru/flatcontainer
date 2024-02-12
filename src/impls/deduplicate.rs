//! Simple deduplication of equal consecutive items.

use crate::impls::offsets::{OffsetContainer, OffsetRegion};
use crate::{CopyOnto, Region};
use crate::impls::vec::CopyVector;

/// A region to deduplicate consecutive equal items.
#[derive(Debug, Clone)]
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

impl<R: Region> Region for CollapseSequence<R>
where
    for<'a, 'b> R::ReadItem<'a>: PartialEq<R::ReadItem<'b>>,
{
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
}

impl<R: Region, T: CopyOnto<R>> CopyOnto<CollapseSequence<R>> for T
where
    for<'a> T: PartialEq<R::ReadItem<'a>>,
    for<'a, 'b> R::ReadItem<'a>: PartialEq<R::ReadItem<'b>>,
{
    fn copy_onto(self, target: &mut CollapseSequence<R>) -> <CollapseSequence<R> as Region>::Index {
        if let Some(last_index) = target.last_index {
            if self == target.inner.index(last_index) {
                return last_index;
            }
        }
        let index = self.copy_onto(&mut target.inner);
        target.last_index = Some(index);
        index
    }
}

/// Transform an index of `(usize, usize)` to a sequence of `0..`. Requires the pairs to
/// be dense, i.e., `(i, j)` is followed by `(j, k)`.
///
/// Defers to region `R` for storing items, and uses offset container `O` to
/// rember indices. By default, `O` is `Vec<usize>`.
#[derive(Debug, Clone)]
pub struct ConsecutiveOffsetPairs<R, O = CopyVector<usize>>
where
    R: Region<Index = (usize, usize)>,
    O: OffsetRegion, usize: CopyOnto<O>,
{
    /// Wrapped region
    inner: R,
    /// Storage for offsets. Always stores element 0.
    offsets: O,
    /// The most recent end of the index pair of region `R`.
    last_index: usize,
}

impl<R: Region<Index = (usize, usize)>, O: OffsetRegion> Default
    for ConsecutiveOffsetPairs<R, O>
where usize: CopyOnto<O>,
{
    fn default() -> Self {
        let mut d = Self {
            inner: Default::default(),
            offsets: Default::default(),
            last_index: 0,
        };
        0.copy_onto(&mut d.offsets);
        d
    }
}

impl<R: Region<Index = (usize, usize)>, O: OffsetRegion> Region
    for ConsecutiveOffsetPairs<R, O>
    where usize: CopyOnto<O>,
{
    type ReadItem<'a> = R::ReadItem<'a>
    where
        Self: 'a;

    type Index = usize;

    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        let mut offsets = O::default();
        0.copy_onto(&mut offsets);
        Self {
            inner: R::merge_regions(regions.clone().map(|r| &r.inner)),
            offsets,
            last_index: 0,
        }
    }

    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        self.inner
            .index((self.offsets.index(index), self.offsets.index(index + 1)))
    }

    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.inner.reserve_regions(regions.map(|r| &r.inner));
    }

    fn clear(&mut self) {
        self.last_index = 0;
        self.inner.clear();
        self.offsets.clear();
        0.copy_onto(&mut self.offsets);
    }
}

impl<R: Region<Index = (usize, usize)>, O: OffsetRegion, T: CopyOnto<R>>
    CopyOnto<ConsecutiveOffsetPairs<R, O>> for T
    where usize: CopyOnto<O>,
{
    fn copy_onto(
        self,
        target: &mut ConsecutiveOffsetPairs<R, O>,
    ) -> <ConsecutiveOffsetPairs<R, O> as Region>::Index {
        let index = self.copy_onto(&mut target.inner);
        debug_assert_eq!(index.0, target.last_index);
        target.last_index = index.1;
        index.1.copy_onto(&mut target.offsets);
        target.offsets.len() - 2
    }
}

#[cfg(test)]
mod tests {
    use crate::impls::deduplicate::{CollapseSequence, ConsecutiveOffsetPairs};
    use crate::impls::offsets::OffsetOptimized;
    use crate::{CopyOnto, FlatStack, Region, StringRegion};

    #[test]
    fn test_dedup_flatstack() {
        let mut fs = FlatStack::<CollapseSequence<StringRegion>>::default();

        fs.copy("abc");
        fs.copy("abc");

        println!("{fs:?}");
    }

    #[test]
    fn test_dedup_region() {
        let mut r = CollapseSequence::<StringRegion>::default();

        fn copy<R: Region>(r: &mut R, item: impl CopyOnto<R>) -> R::Index {
            item.copy_onto(r)
        }

        assert_eq!(copy(&mut r, "abc"), copy(&mut r, "abc"));

        println!("{r:?}");
    }

    #[test]
    fn test_offset_optimized() {
        let mut r =
            CollapseSequence::<ConsecutiveOffsetPairs<StringRegion, OffsetOptimized>>::default();

        fn copy<R: Region>(r: &mut R, item: impl CopyOnto<R>) -> R::Index {
            item.copy_onto(r)
        }

        for _ in 0..1000 {
            copy(&mut r, "abc");
        }

        println!("{r:?}");
    }
}
