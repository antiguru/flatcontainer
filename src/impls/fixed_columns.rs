//! A region to contain a variable number of columns.

use std::fmt::Debug;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::impls::offsets::OffsetContainer;
use crate::{CopyIter, IntoOwned, Push, Region};

/// A region that can store a fixed number of elements per row.
///
/// The region is backed by a number of columns, where the number depends on
/// the length of the first row encountered. For pushed row, the region
/// remembers the indices into each column that populated. All rows must have the
/// same length.
///
/// All columns have the same type `R`, indexes into `R` are stored in an `O`: [`OffsetContainer`].
///
/// # Examples
///
/// Copy a table-like structure:
/// ```
/// # use flatcontainer::impls::deduplicate::ConsecutiveOffsetPairs;
/// # use flatcontainer::{ColumnsRegion, Push, Region, StringRegion};
/// # use flatcontainer::impls::fixed_columns::FixedColumnsRegion;
/// # use flatcontainer::impls::offsets::OffsetOptimized;
/// let data = [
///     vec!["1", "2", "3"],
///     vec!["4", "5", "6"],
///     vec!["7", "8", "9"],
/// ];
///
/// let mut r = <FixedColumnsRegion<ConsecutiveOffsetPairs<StringRegion>, OffsetOptimized>>::default();
///
/// let mut indices = Vec::with_capacity(data.len());
///
/// for row in &data {
///     let index = r.push(row);
///     indices.push(index);
/// }
///
/// # for (&index, row) in indices.iter().zip(&data) {
/// #     assert!(row.iter().copied().eq(r.index(index).iter()));
/// # }
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FixedColumnsRegion<R, O> {
    /// Offsets into individual columns.
    offsets: Vec<O>,
    /// Storage for columns.
    inner: Vec<R>,
}

impl<R: Default, O: Default> FixedColumnsRegion<R, O> {
    /// Ensures that the region has a width of exactly `columns`, either
    /// by creating sufficient columns, or by interrupting the program.
    ///
    /// # Panics
    ///
    /// Panics if the region has a different number of columns.
    #[inline]
    fn ensure_columns(&mut self, columns: usize) {
        // Ensure all required regions exist.
        assert!(
            self.inner.is_empty() || columns == self.inner.len(),
            "All rows in a fixed columns region must have equal length, expected {} but is {}.",
            columns,
            self.inner.len()
        );
        if self.inner.is_empty() {
            self.populate_columns(columns);
        }
    }

    /// Add `columns` columns to the region. Only called if the region is empty.
    #[cold]
    fn populate_columns(&mut self, columns: usize) {
        while self.inner.len() < columns {
            self.inner.push(R::default());
            self.offsets.push(O::default());
        }
    }
}

impl<R, O> Region for FixedColumnsRegion<R, O>
where
    R: Region,
    O: OffsetContainer<R::Index>,
{
    type Owned = Vec<R::Owned>;
    type ReadItem<'a> = ReadColumns<'a, R, O> where Self: 'a;
    type Index = usize;

    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        let cols = regions.clone().map(|r| r.inner.len()).max().unwrap_or(0);
        let len_iter = regions.clone().map(|r| r.inner.len()).filter(|&l| l > 0);
        debug_assert_eq!(len_iter.clone().min(), len_iter.max());

        let mut inner = Vec::with_capacity(cols);
        let mut offsets = Vec::with_capacity(cols);
        for col in 0..cols {
            inner.push(R::merge_regions(
                regions.clone().flat_map(|r| r.inner.get(col)),
            ));
            offsets.push(O::default());
        }

        Self { inner, offsets }
    }

    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        ReadColumns(Ok(ReadColumnsInner {
            columns: self,
            index,
        }))
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
        for offset in &mut self.offsets {
            offset.clear();
        }
    }

    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        for inner in &self.inner {
            inner.heap_size(&mut callback);
        }
        for offset in &self.offsets {
            offset.heap_size(&mut callback);
        }
    }

    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item
    }
}

impl<R, O> Default for FixedColumnsRegion<R, O>
where
    R: Region,
    O: OffsetContainer<R::Index>,
{
    fn default() -> Self {
        Self {
            inner: Vec::default(),
            offsets: Vec::default(),
        }
    }
}

/// Read the values of a row.
pub struct ReadColumns<'a, R, O>(Result<ReadColumnsInner<'a, R, O>, &'a [R::Owned]>)
where
    R: Region;

/// Read the values of a row.
pub struct ReadColumnsInner<'a, R, O> {
    /// Storage for columns.
    columns: &'a FixedColumnsRegion<R, O>,
    /// Row index.
    index: usize,
}

impl<'a, R, O> Clone for ReadColumns<'a, R, O>
where
    R: Region,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, R, O> Clone for ReadColumnsInner<'a, R, O> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, R, O> Copy for ReadColumns<'a, R, O> where R: Region {}
impl<'a, R, O> Copy for ReadColumnsInner<'a, R, O> {}

impl<'a, R, O> Debug for ReadColumns<'a, R, O>
where
    R: Region,
    R::ReadItem<'a>: Debug,
    O: OffsetContainer<R::Index>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<'a, R, O> ReadColumns<'a, R, O>
where
    R: Region,
    O: OffsetContainer<R::Index>,
{
    /// Iterate the individual values of a row.
    pub fn iter(&'a self) -> ReadColumnsIter<'a, R, O> {
        self.into_iter()
    }

    /// Get the element at `offset`.
    #[must_use]
    pub fn get(&self, offset: usize) -> R::ReadItem<'a> {
        match &self.0 {
            Ok(inner) => inner.get(offset),
            Err(slice) => IntoOwned::borrow_as(&slice[offset]),
        }
    }

    /// Returns the length of this row.
    #[must_use]
    pub fn len(&self) -> usize {
        match &self.0 {
            Ok(inner) => inner.len(),
            Err(slice) => slice.len(),
        }
    }

    /// Returns `true` if this row is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match &self.0 {
            Ok(inner) => inner.is_empty(),
            Err(slice) => slice.is_empty(),
        }
    }
}

impl<'a, R, O> ReadColumnsInner<'a, R, O>
where
    R: Region,
    O: OffsetContainer<R::Index>,
{
    /// Get the element at `offset`.
    #[must_use]
    pub fn get(&self, offset: usize) -> R::ReadItem<'a> {
        self.columns.inner[offset].index(self.columns.offsets[offset].index(self.index))
    }

    /// Returns the length of this row.
    #[must_use]
    pub fn len(&self) -> usize {
        self.columns.inner.len()
    }

    /// Returns `true` if this row is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.columns.inner.is_empty()
    }
}

impl<'a, R, O> IntoOwned<'a> for ReadColumns<'a, R, O>
where
    R: Region,
    O: OffsetContainer<R::Index>,
{
    type Owned = Vec<R::Owned>;

    #[inline]
    fn into_owned(self) -> Self::Owned {
        self.iter().map(IntoOwned::into_owned).collect()
    }

    fn clone_onto(self, other: &mut Self::Owned) {
        let r = std::cmp::min(self.len(), other.len());
        for (item, target) in self.iter().zip(other.iter_mut()) {
            item.clone_onto(target);
        }
        other.extend(self.iter().skip(r).map(IntoOwned::into_owned));
        other.truncate(self.len());
    }

    fn borrow_as(owned: &'a Self::Owned) -> Self {
        Self(Err(owned.as_slice()))
    }
}

impl<'a, R, O> IntoIterator for &ReadColumns<'a, R, O>
where
    R: Region,
    O: OffsetContainer<R::Index>,
{
    type Item = R::ReadItem<'a>;
    type IntoIter = ReadColumnsIter<'a, R, O>;

    fn into_iter(self) -> Self::IntoIter {
        match &self.0 {
            Ok(inner) => ReadColumnsIter(Ok(ReadColumnsIterInner {
                iter: inner.columns.inner.iter().zip(inner.columns.offsets.iter()),
                index: inner.index,
            })),
            Err(slice) => ReadColumnsIter(Err(slice.iter())),
        }
    }
}

/// An iterator over the elements of a row.
pub struct ReadColumnsIter<'a, R: Region, O>(
    Result<ReadColumnsIterInner<'a, R, O>, std::slice::Iter<'a, R::Owned>>,
);

/// An iterator over the elements of a row.
pub struct ReadColumnsIterInner<'a, R, O> {
    iter: std::iter::Zip<std::slice::Iter<'a, R>, std::slice::Iter<'a, O>>,
    index: usize,
}

impl<'a, R, O> Iterator for ReadColumnsIter<'a, R, O>
where
    R: Region,
    O: OffsetContainer<R::Index>,
{
    type Item = R::ReadItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            Ok(inner) => inner.next(),
            Err(slice) => slice.next().map(IntoOwned::borrow_as),
        }
    }
}

impl<'a, R, O> Iterator for ReadColumnsIterInner<'a, R, O>
where
    R: Region,
    O: OffsetContainer<R::Index>,
{
    type Item = R::ReadItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(region, offsets)| region.index(offsets.index(self.index)))
    }
}

impl<R, O> Push<ReadColumns<'_, R, O>> for FixedColumnsRegion<R, O>
where
    R: Region + for<'a> Push<<R as Region>::ReadItem<'a>>,
    O: OffsetContainer<R::Index>,
{
    fn push(&mut self, item: ReadColumns<'_, R, O>) -> Self::Index {
        self.ensure_columns(item.len());
        for ((item, region), offsets) in item
            .iter()
            .zip(self.inner.iter_mut())
            .zip(self.offsets.iter_mut())
        {
            let index = region.push(item);
            offsets.push(index);
        }
        self.offsets.first().map(|o| o.len() - 1).unwrap_or(0)
    }
}

impl<'a, R, O, T> Push<&'a [T]> for FixedColumnsRegion<R, O>
where
    R: Region + Push<&'a T>,
    O: OffsetContainer<R::Index>,
{
    fn push(&mut self, item: &'a [T]) -> Self::Index {
        self.ensure_columns(item.len());
        for ((item, region), offsets) in item
            .iter()
            .zip(self.inner.iter_mut())
            .zip(self.offsets.iter_mut())
        {
            let index = region.push(item);
            offsets.push(index);
        }
        self.offsets.first().map(|o| o.len() - 1).unwrap_or(0)
    }
}

impl<R, O, T> Push<Vec<T>> for FixedColumnsRegion<R, O>
where
    R: Region + Push<T>,
    O: OffsetContainer<R::Index>,
{
    fn push(&mut self, item: Vec<T>) -> Self::Index {
        self.ensure_columns(item.len());
        for ((item, region), offsets) in item
            .into_iter()
            .zip(self.inner.iter_mut())
            .zip(self.offsets.iter_mut())
        {
            let index = region.push(item);
            offsets.push(index);
        }
        self.offsets.first().map(|o| o.len() - 1).unwrap_or(0)
    }
}

impl<'a, R, O, T> Push<&'a Vec<T>> for FixedColumnsRegion<R, O>
where
    R: Region + Push<&'a T>,
    O: OffsetContainer<R::Index>,
{
    fn push(&mut self, item: &'a Vec<T>) -> Self::Index {
        self.ensure_columns(item.len());
        for (index, offsets) in item
            .iter()
            .zip(self.inner.iter_mut())
            .map(|(item, region)| region.push(item))
            .zip(self.offsets.iter_mut())
        {
            offsets.push(index);
        }
        self.offsets.first().map(|o| o.len() - 1).unwrap_or(0)
    }
}

impl<R, O, T, I> Push<CopyIter<I>> for FixedColumnsRegion<R, O>
where
    R: Region + Push<T>,
    O: OffsetContainer<R::Index>,
    I: IntoIterator<Item = T>,
{
    #[inline]
    fn push(&mut self, item: CopyIter<I>) -> Self::Index {
        if self.inner.is_empty() {
            // Writing the first row, which determines the number of columns.
            for (column, value) in item.0.into_iter().enumerate() {
                self.inner.push(R::default());
                self.offsets.push(O::default());
                let index = self.inner[column].push(value);
                self.offsets[column].push(index);
            }
        } else {
            let mut columns = 0;
            for (column, value) in item.0.into_iter().enumerate() {
                let index = self.inner[column].push(value);
                self.offsets[column].push(index);
                columns += 1;
            }
            self.ensure_columns(columns);
        }
        self.offsets.first().map(|o| o.len() - 1).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::impls::deduplicate::{CollapseSequence, ConsecutiveOffsetPairs};
    use crate::impls::offsets::OffsetOptimized;
    use crate::{CopyIter, MirrorRegion, Push, Region, StringRegion};

    use super::*;

    #[test]
    fn test_matrix() {
        let data = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

        let mut r = FixedColumnsRegion::<MirrorRegion<_>, OffsetOptimized>::default();

        let mut indices = Vec::with_capacity(data.len());

        for row in &data {
            let index = r.push(row.as_slice());
            indices.push(index);
        }

        for (&index, row) in indices.iter().zip(&data) {
            assert!(row.iter().copied().eq(r.index(index).iter()));
        }
    }

    #[test]
    fn test_string_vec() {
        let data = vec![
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
            vec!["7".to_string(), "8".to_string(), "9".to_string()],
        ];

        let mut r = FixedColumnsRegion::<
            CollapseSequence<ConsecutiveOffsetPairs<StringRegion>>,
            OffsetOptimized,
        >::default();

        let mut indices = Vec::with_capacity(data.len());

        for row in &data {
            let index = r.push(row);
            indices.push(index);
        }

        for (&index, row) in indices.iter().zip(&data) {
            assert!(row.iter().eq(r.index(index).iter()));
        }

        println!("{r:?}");
    }

    #[test]
    fn test_str_vec() {
        let data = [
            vec!["1", "2", "3"],
            vec!["4", "5", "6"],
            vec!["7", "8", "9"],
        ];

        let mut r =
            FixedColumnsRegion::<ConsecutiveOffsetPairs<StringRegion>, OffsetOptimized>::default();

        let mut indices = Vec::with_capacity(data.len());

        for row in &data {
            let index = r.push(row);
            indices.push(index);
        }

        for (&index, row) in indices.iter().zip(&data) {
            assert!(row.iter().copied().eq(r.index(index).iter()));
        }

        println!("{r:?}");
    }

    #[test]
    fn test_str_iter() {
        let data = [
            vec!["1", "2", "3"],
            vec!["4", "5", "6"],
            vec!["7", "8", "9"],
        ];

        let mut r =
            FixedColumnsRegion::<ConsecutiveOffsetPairs<StringRegion>, OffsetOptimized>::default();

        let mut indices = Vec::with_capacity(data.len());

        for row in &data {
            let index = r.push(CopyIter(row.iter()));
            indices.push(index);
        }

        for (&index, row) in indices.iter().zip(&data) {
            assert!(row.iter().copied().eq(r.index(index).iter()));
        }

        println!("{r:?}");
    }
}
