//! A region to contain a variable number of columns.

use std::fmt::Debug;

use crate::impls::deduplicate::ConsecutiveOffsetPairs;
use crate::impls::offsets::OffsetOptimized;
use crate::CopyIter;
use crate::{CopyOnto, CopyRegion, Index, Region};

/// A region that can store a variable number of elements per row.
///
/// The region is backed by a number of columns, where the number depends on
/// the length of the longest row encountered. For pushed row, the region
/// remembers the indices into each column that populated. Rows can have different
/// lengths, which means that only the first columns will contain a value.
///
/// All columns have the same type `R`.
///
/// # Examples
///
/// Copy a table-like structure:
/// ```
/// # use flatcontainer::impls::deduplicate::ConsecutiveOffsetPairs;
/// # use flatcontainer::{ColumnsRegion, CopyOnto, Region, StringRegion};
/// let data = [
///     vec![],
///     vec!["1"],
///     vec!["2", "3"],
///     vec!["4", "5", "6"],
///     vec!["7", "8"],
///     vec!["9"],
///     vec![],
/// ];
///
/// let mut r = <ColumnsRegion<ConsecutiveOffsetPairs<StringRegion>, _>>::default();
///
/// let mut indices = Vec::with_capacity(data.len());
///
/// for row in &data {
///     let index = row.copy_onto(&mut r);
///     indices.push(index);
/// }
///
/// for (&index, row) in indices.iter().zip(&data) {
///     assert!(row.iter().copied().eq(r.index(index).iter()));
/// }
/// ```
#[derive(Debug)]
pub struct ColumnsRegion<R, Idx>
where
    R: Region<Index = Idx>,
    Idx: Index,
{
    /// Indices to address rows in `inner`. For each row, we remeber
    /// an index for each column.
    indices: ConsecutiveOffsetPairs<CopyRegion<Idx>, OffsetOptimized>,
    /// Storage for columns.
    inner: Vec<R>,
}

impl<R, Idx> Region for ColumnsRegion<R, Idx>
where
    R: Region<Index = Idx>,
    Idx: Index,
{
    type ReadItem<'a> = ReadColumns<'a, R, Idx> where Self: 'a;
    type Index = usize;

    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        let cols = regions.clone().map(|r| r.inner.len()).max().unwrap_or(0);

        let mut inner = Vec::with_capacity(cols);
        for col in 0..cols {
            inner.push(R::merge_regions(
                regions.clone().flat_map(|r| r.inner.get(col)),
            ));
        }

        Self {
            indices: ConsecutiveOffsetPairs::merge_regions(regions.map(|r| &r.indices)),
            inner,
        }
    }

    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        ReadColumns {
            columns: &self.inner,
            index: self.indices.index(index),
        }
    }

    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        for region in regions.clone() {
            while self.inner.len() < region.inner.len() {
                self.inner.push(R::default());
            }
        }
        for (index, inner) in self.inner.iter_mut().enumerate() {
            inner.reserve_regions(regions.clone().flat_map(|r| r.inner.get(index)))
        }
    }

    fn clear(&mut self) {
        for inner in &mut self.inner {
            inner.clear();
        }
        self.indices.clear();
    }
}

impl<R, Idx> Default for ColumnsRegion<R, Idx>
where
    R: Region<Index = Idx>,
    Idx: Index,
{
    fn default() -> Self {
        Self {
            indices: Default::default(),
            inner: Vec::default(),
        }
    }
}

/// Read the values of a row.
pub struct ReadColumns<'a, R, Idx>
where
    R: Region<Index = Idx>,
    Idx: Index,
{
    /// Storage for columns.
    columns: &'a [R],
    /// Indices to retrieve values from columns.
    index: &'a [Idx],
}

impl<'a, R, Idx> Clone for ReadColumns<'a, R, Idx>
where
    R: Region<Index = Idx>,
    Idx: Index,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, R, Idx> Copy for ReadColumns<'a, R, Idx>
where
    R: Region<Index = Idx>,
    Idx: Index,
{
}

impl<'a, R, Idx> Debug for ReadColumns<'a, R, Idx>
where
    R: Region<Index = Idx>,
    R::ReadItem<'a>: Debug,
    Idx: Index,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<'a, R, Idx> ReadColumns<'a, R, Idx>
where
    R: Region<Index = Idx>,
    Idx: Index,
{
    /// Iterate the individual values of a row.
    pub fn iter(&'a self) -> ReadColumnsIter<'a, R, Idx> {
        self.into_iter()
    }

    /// Get the element at `offset`.
    pub fn get(&self, offset: usize) -> R::ReadItem<'a> {
        self.columns[offset].index(self.index[offset])
    }

    /// Returns the length of this row.
    pub fn len(&self) -> usize {
        self.index.len()
    }

    /// Returns `true` if this row is empty.
    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }
}

impl<'a, R, Idx> IntoIterator for &ReadColumns<'a, R, Idx>
where
    R: Region<Index = Idx>,
    Idx: Index,
{
    type Item = R::ReadItem<'a>;
    type IntoIter = ReadColumnsIter<'a, R, Idx>;

    fn into_iter(self) -> Self::IntoIter {
        ReadColumnsIter {
            iter: self.index.iter().zip(self.columns.iter()),
        }
    }
}

/// An iterator over the elements of a row.
pub struct ReadColumnsIter<'a, R, Idx> {
    iter: std::iter::Zip<std::slice::Iter<'a, Idx>, std::slice::Iter<'a, R>>,
}

impl<'a, R, Idx> Iterator for ReadColumnsIter<'a, R, Idx>
where
    R: Region<Index = Idx>,
    Idx: Index,
{
    type Item = R::ReadItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(&i, r)| r.index(i))
    }
}

impl<'a, R, Idx> CopyOnto<ColumnsRegion<R, Idx>> for ReadColumns<'a, R, Idx>
where
    R: Region<Index = Idx>,
    Idx: Index,
{
    fn copy_onto(
        self,
        target: &mut ColumnsRegion<R, Idx>,
    ) -> <ColumnsRegion<R, Idx> as Region>::Index {
        // Ensure all required regions exist.
        while target.inner.len() < self.len() {
            target.inner.push(R::default());
        }

        let iter = self
            .iter()
            .zip(&mut target.inner)
            .map(|(value, region)| value.copy_onto(region));
        CopyIter(iter).copy_onto(&mut target.indices)
    }
}

impl<'a, R, Idx, T> CopyOnto<ColumnsRegion<R, Idx>> for &'a [T]
where
    R: Region<Index = Idx>,
    Idx: Index,
    &'a T: CopyOnto<R>,
{
    fn copy_onto(
        self,
        target: &mut ColumnsRegion<R, Idx>,
    ) -> <ColumnsRegion<R, Idx> as Region>::Index {
        // Ensure all required regions exist.
        while target.inner.len() < self.len() {
            target.inner.push(R::default());
        }

        let iter = self
            .iter()
            .zip(&mut target.inner)
            .map(|(value, region)| value.copy_onto(region));
        CopyIter(iter).copy_onto(&mut target.indices)
    }
}

impl<R, Idx, T> CopyOnto<ColumnsRegion<R, Idx>> for Vec<T>
where
    R: Region<Index = Idx>,
    Idx: Index,
    T: CopyOnto<R>,
{
    fn copy_onto(
        self,
        target: &mut ColumnsRegion<R, Idx>,
    ) -> <ColumnsRegion<R, Idx> as Region>::Index {
        // Ensure all required regions exist.
        while target.inner.len() < self.len() {
            target.inner.push(R::default());
        }

        let iter = self
            .into_iter()
            .zip(&mut target.inner)
            .map(|(value, region)| value.copy_onto(region));
        CopyIter(iter).copy_onto(&mut target.indices)
    }
}

impl<'a, R, Idx, T> CopyOnto<ColumnsRegion<R, Idx>> for &'a Vec<T>
where
    R: Region<Index = Idx>,
    Idx: Index,
    &'a T: CopyOnto<R>,
{
    fn copy_onto(
        self,
        target: &mut ColumnsRegion<R, Idx>,
    ) -> <ColumnsRegion<R, Idx> as Region>::Index {
        // Ensure all required regions exist.
        while target.inner.len() < self.len() {
            target.inner.push(R::default());
        }

        let iter = self
            .iter()
            .zip(&mut target.inner)
            .map(|(value, region)| value.copy_onto(region));
        CopyIter(iter).copy_onto(&mut target.indices)
    }
}

impl<R, Idx, T, I> CopyOnto<ColumnsRegion<R, Idx>> for CopyIter<I>
where
    R: Region<Index = Idx>,
    Idx: Index,
    T: CopyOnto<R>,
    I: IntoIterator<Item = T>,
{
    #[inline]
    fn copy_onto(
        self,
        target: &mut ColumnsRegion<R, Idx>,
    ) -> <ColumnsRegion<R, Idx> as Region>::Index {
        let iter = self.0.into_iter().enumerate().map(|(index, value)| {
            // Ensure all required regions exist.
            if target.inner.len() <= index {
                target.inner.push(R::default());
            }
            value.copy_onto(&mut target.inner[index])
        });
        CopyIter(iter).copy_onto(&mut target.indices)
    }
}

#[cfg(test)]
mod tests {
    use crate::impls::columns::ColumnsRegion;
    use crate::impls::deduplicate::{CollapseSequence, ConsecutiveOffsetPairs};
    use crate::CopyIter;
    use crate::{CopyOnto, MirrorRegion, Region, StringRegion};

    #[test]
    fn test_matrix() {
        let data = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

        let mut r = ColumnsRegion::<MirrorRegion<_>, _>::default();

        let mut indices = Vec::with_capacity(data.len());

        for row in &data {
            let index = row.as_slice().copy_onto(&mut r);
            indices.push(index);
        }

        for (&index, row) in indices.iter().zip(&data) {
            assert!(row.iter().copied().eq(r.index(index).iter()));
        }
    }

    #[test]
    fn test_ragged() {
        let data = [
            [].as_slice(),
            [1].as_slice(),
            [2, 3].as_slice(),
            [4, 5, 6].as_slice(),
            [7, 8].as_slice(),
            [9].as_slice(),
            [].as_slice(),
        ];

        let mut r = ColumnsRegion::<MirrorRegion<_>, _>::default();

        let mut indices = Vec::with_capacity(data.len());

        for row in &data {
            let index = (*row).copy_onto(&mut r);
            indices.push(index);
        }

        for (&index, row) in indices.iter().zip(&data) {
            assert!(row.iter().copied().eq(r.index(index).iter()));
        }

        println!("{r:?}");
    }

    #[test]
    fn test_ragged_string_vec() {
        let data = vec![
            vec![],
            vec!["1".to_string()],
            vec!["2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
            vec!["7".to_string(), "8".to_string()],
            vec!["9".to_string()],
            vec![],
        ];

        let mut r =
            ColumnsRegion::<CollapseSequence<ConsecutiveOffsetPairs<StringRegion>>, _>::default();

        let mut indices = Vec::with_capacity(data.len());

        for row in &data {
            let index = row.copy_onto(&mut r);
            indices.push(index);
        }

        for (&index, row) in indices.iter().zip(&data) {
            assert!(row.iter().eq(r.index(index).iter()));
        }

        println!("{r:?}");
    }

    #[test]
    fn test_ragged_str_vec() {
        let data = [
            vec![],
            vec!["1"],
            vec!["2", "3"],
            vec!["4", "5", "6"],
            vec!["7", "8"],
            vec!["9"],
            vec![],
        ];

        let mut r = ColumnsRegion::<ConsecutiveOffsetPairs<StringRegion>, _>::default();

        let mut indices = Vec::with_capacity(data.len());

        for row in &data {
            let index = row.copy_onto(&mut r);
            indices.push(index);
        }

        for (&index, row) in indices.iter().zip(&data) {
            assert!(row.iter().copied().eq(r.index(index).iter()));
        }

        println!("{r:?}");
    }

    #[test]
    fn test_ragged_str_iter() {
        let data = [
            vec![],
            vec!["1"],
            vec!["2", "3"],
            vec!["4", "5", "6"],
            vec!["7", "8"],
            vec!["9"],
            vec![],
        ];

        let mut r = ColumnsRegion::<ConsecutiveOffsetPairs<StringRegion>, _>::default();

        let mut indices = Vec::with_capacity(data.len());

        for row in &data {
            let index = CopyIter(row.iter()).copy_onto(&mut r);
            indices.push(index);
        }

        for (&index, row) in indices.iter().zip(&data) {
            assert!(row.iter().copied().eq(r.index(index).iter()));
        }

        println!("{r:?}");
    }
}
