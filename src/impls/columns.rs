//! A region to contain a variable number of columns.

use std::fmt::Debug;
use std::iter::Zip;
use std::slice::Iter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::impls::deduplicate::ConsecutiveOffsetPairs;
use crate::impls::offsets::{OffsetContainer, OffsetOptimized};
use crate::{IntoOwned, PushIter};
use crate::{OwnedRegion, Push, Region};

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
/// # use flatcontainer::{ColumnsRegion, Push, Region, StringRegion};
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
/// let mut r = <ColumnsRegion<ConsecutiveOffsetPairs<StringRegion>>>::default();
///
/// let mut indices = Vec::with_capacity(data.len());
///
/// for row in &data {
///     let index = r.push(row);
///     indices.push(index);
/// }
///
/// for (&index, row) in indices.iter().zip(&data) {
///     assert!(row.iter().copied().eq(r.index(index).iter()));
/// }
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound = "
            R: Serialize + for<'a> Deserialize<'a>,
            R::Index: Serialize + for<'a> Deserialize<'a>,
            O: Serialize + for<'a> Deserialize<'a>,
            ")
)]
pub struct ColumnsRegion<R, O = OffsetOptimized>
where
    R: Region,
{
    /// Indices to address rows in `inner`. For each row, we remember
    /// an index for each column.
    indices: ConsecutiveOffsetPairs<OwnedRegion<R::Index>, O>,
    /// Storage for columns.
    inner: Vec<R>,
}

impl<R, O> Clone for ColumnsRegion<R, O>
where
    R: Region + Clone,
    O: Clone,
{
    fn clone(&self) -> Self {
        Self {
            indices: self.indices.clone(),
            inner: self.inner.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.indices.clone_from(&source.indices);
        self.inner.clone_from(&source.inner);
    }
}

impl<R, O> Region for ColumnsRegion<R, O>
where
    R: Region,
    O: OffsetContainer<usize>,
{
    type Owned = Vec<R::Owned>;
    type ReadItem<'a> = ReadColumns<'a, R> where Self: 'a;
    type Index = <ConsecutiveOffsetPairs<OwnedRegion<R::Index>, OffsetOptimized> as Region>::Index;

    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        let cols = regions.clone().map(|r| r.inner.len()).max().unwrap_or(0);

        let mut inner = Vec::with_capacity(cols);
        for col in 0..cols {
            inner.push(R::merge_regions(
                regions.clone().filter_map(|r| r.inner.get(col)),
            ));
        }

        Self {
            indices: ConsecutiveOffsetPairs::merge_regions(regions.map(|r| &r.indices)),
            inner,
        }
    }

    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        ReadColumns(Ok(ReadColumnsInner {
            columns: &self.inner,
            index: self.indices.index(index),
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
            inner.reserve_regions(regions.clone().filter_map(|r| r.inner.get(index)));
        }
    }

    fn clear(&mut self) {
        for inner in &mut self.inner {
            inner.clear();
        }
        self.indices.clear();
    }

    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        let size_of_r = std::mem::size_of::<R>();
        callback(
            self.inner.len() * size_of_r,
            self.inner.capacity() * size_of_r,
        );
        for inner in &self.inner {
            inner.heap_size(&mut callback);
        }
        self.indices.heap_size(callback);
    }

    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item
    }
}

impl<R, O> Default for ColumnsRegion<R, O>
where
    R: Region,
    O: OffsetContainer<usize>,
{
    fn default() -> Self {
        Self {
            indices: ConsecutiveOffsetPairs::default(),
            inner: Vec::default(),
        }
    }
}

/// Read the values of a row.
pub struct ReadColumns<'a, R>(Result<ReadColumnsInner<'a, R>, &'a [R::Owned]>)
where
    R: Region;

struct ReadColumnsInner<'a, R>
where
    R: Region,
{
    /// Storage for columns.
    columns: &'a [R],
    /// Indices to retrieve values from columns.
    index: &'a [R::Index],
}

impl<'a, R> Clone for ReadColumns<'a, R>
where
    R: Region,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, R> Clone for ReadColumnsInner<'a, R>
where
    R: Region,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, R> Copy for ReadColumns<'a, R> where R: Region {}
impl<'a, R> Copy for ReadColumnsInner<'a, R> where R: Region {}

impl<'a, R> Debug for ReadColumns<'a, R>
where
    R: Region,
    R::ReadItem<'a>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<'a, R> ReadColumns<'a, R>
where
    R: Region,
{
    /// Iterate the individual values of a row.
    #[must_use]
    pub fn iter(&'a self) -> ReadColumnsIter<'a, R> {
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
impl<'a, R> ReadColumnsInner<'a, R>
where
    R: Region,
{
    /// Get the element at `offset`.
    #[must_use]
    pub fn get(&self, offset: usize) -> R::ReadItem<'a> {
        self.columns[offset].index(self.index[offset])
    }

    /// Returns the length of this row.
    #[must_use]
    pub fn len(&self) -> usize {
        self.index.len()
    }

    /// Returns `true` if this row is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }
}

impl<'a, R> IntoOwned<'a> for ReadColumns<'a, R>
where
    R: Region,
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

impl<'a, R> IntoIterator for &ReadColumns<'a, R>
where
    R: Region,
{
    type Item = R::ReadItem<'a>;
    type IntoIter = ReadColumnsIter<'a, R>;

    fn into_iter(self) -> Self::IntoIter {
        match self.0 {
            Ok(inner) => ReadColumnsIter(Ok(ReadColumnsIterInner {
                iter: inner.index.iter().zip(inner.columns.iter()),
            })),
            Err(slice) => ReadColumnsIter(Err(slice.iter())),
        }
    }
}

/// An iterator over the elements of a row.
pub struct ReadColumnsIter<'a, R: Region>(Result<ReadColumnsIterInner<'a, R>, Iter<'a, R::Owned>>);

/// An iterator over the elements of a row.
pub struct ReadColumnsIterInner<'a, R: Region> {
    iter: Zip<Iter<'a, R::Index>, Iter<'a, R>>,
}

impl<'a, R> Iterator for ReadColumnsIter<'a, R>
where
    R: Region,
{
    type Item = R::ReadItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            Ok(inner) => inner.next(),
            Err(slice) => slice.next().map(IntoOwned::borrow_as),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.0 {
            Ok(inner) => inner.size_hint(),
            Err(slice) => slice.size_hint(),
        }
    }
}

impl<'a, R> ExactSizeIterator for ReadColumnsIter<'a, R> where R: Region {}

impl<'a, R> Iterator for ReadColumnsIterInner<'a, R>
where
    R: Region,
{
    type Item = R::ReadItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(&i, r)| r.index(i))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<R, O> Push<ReadColumns<'_, R>> for ColumnsRegion<R, O>
where
    for<'a> R: Region + Push<<R as Region>::ReadItem<'a>>,
    O: OffsetContainer<usize>,
{
    fn push(&mut self, item: ReadColumns<'_, R>) -> <ColumnsRegion<R, O> as Region>::Index {
        // Ensure all required regions exist.
        while self.inner.len() < item.len() {
            self.inner.push(R::default());
        }

        let iter = item
            .iter()
            .zip(&mut self.inner)
            .map(|(value, region)| region.push(value));
        self.indices.push(PushIter(iter))
    }
}

impl<'a, R, O, T> Push<&'a [T]> for ColumnsRegion<R, O>
where
    R: Region + Push<&'a T>,
    O: OffsetContainer<usize>,
{
    fn push(&mut self, item: &'a [T]) -> <ColumnsRegion<R, O> as Region>::Index {
        // Ensure all required regions exist.
        while self.inner.len() < item.len() {
            self.inner.push(R::default());
        }

        let iter = item
            .iter()
            .zip(&mut self.inner)
            .map(|(value, region)| region.push(value));
        self.indices.push(PushIter(iter))
    }
}

impl<R, O, T, const N: usize> Push<[T; N]> for ColumnsRegion<R, O>
where
    R: Region + Push<T>,
    O: OffsetContainer<usize>,
{
    fn push(&mut self, item: [T; N]) -> <ColumnsRegion<R, O> as Region>::Index {
        // Ensure all required regions exist.
        while self.inner.len() < item.len() {
            self.inner.push(R::default());
        }

        let iter = item
            .into_iter()
            .zip(&mut self.inner)
            .map(|(value, region)| region.push(value));
        self.indices.push(PushIter(iter))
    }
}

impl<'a, R, O, T, const N: usize> Push<&'a [T; N]> for ColumnsRegion<R, O>
where
    R: Region + Push<&'a T>,
    O: OffsetContainer<usize>,
{
    fn push(&mut self, item: &'a [T; N]) -> <ColumnsRegion<R, O> as Region>::Index {
        // Ensure all required regions exist.
        while self.inner.len() < item.len() {
            self.inner.push(R::default());
        }

        let iter = item
            .iter()
            .zip(&mut self.inner)
            .map(|(value, region)| region.push(value));
        self.indices.push(PushIter(iter))
    }
}

impl<R, O, T> Push<Vec<T>> for ColumnsRegion<R, O>
where
    R: Region + Push<T>,
    O: OffsetContainer<usize>,
{
    fn push(&mut self, item: Vec<T>) -> <ColumnsRegion<R, O> as Region>::Index {
        // Ensure all required regions exist.
        while self.inner.len() < item.len() {
            self.inner.push(R::default());
        }

        let iter = item
            .into_iter()
            .zip(&mut self.inner)
            .map(|(value, region)| region.push(value));
        self.indices.push(PushIter(iter))
    }
}

impl<'a, R, O, T> Push<&'a Vec<T>> for ColumnsRegion<R, O>
where
    R: Region + Push<&'a T>,
    O: OffsetContainer<usize>,
{
    fn push(&mut self, item: &'a Vec<T>) -> <ColumnsRegion<R, O> as Region>::Index {
        // Ensure all required regions exist.
        while self.inner.len() < item.len() {
            self.inner.push(R::default());
        }

        let iter = item
            .iter()
            .zip(&mut self.inner)
            .map(|(value, region)| region.push(value));
        self.indices.push(PushIter(iter))
    }
}

impl<R, O, T, I> Push<PushIter<I>> for ColumnsRegion<R, O>
where
    R: Region + Push<T>,
    O: OffsetContainer<usize>,
    I: IntoIterator<Item = T>,
    I::IntoIter: ExactSizeIterator,
{
    #[inline]
    fn push(&mut self, item: PushIter<I>) -> <ColumnsRegion<R, O> as Region>::Index {
        let iter = item.0.into_iter().enumerate().map(|(index, value)| {
            // Ensure all required regions exist.
            if self.inner.len() <= index {
                self.inner.push(R::default());
            }
            self.inner[index].push(value)
        });
        self.indices.push(PushIter(iter))
    }
}

#[cfg(test)]
mod tests {
    use crate::impls::deduplicate::{CollapseSequence, ConsecutiveOffsetPairs};
    use crate::{MirrorRegion, OwnedRegion, Push, PushIter, Region, StringRegion};

    use super::*;

    #[test]
    fn test_matrix() {
        let data = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

        let mut r = ColumnsRegion::<MirrorRegion<_>>::default();

        let mut indices = Vec::with_capacity(data.len());

        for row in &data {
            let index = r.push(row.as_slice());
            indices.push(index);
        }

        for (index, row) in indices.iter().zip(&data) {
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

        let mut r = ColumnsRegion::<MirrorRegion<_>>::default();

        let mut indices = Vec::with_capacity(data.len());

        for row in &data {
            let index = r.push(*row);
            indices.push(index);
        }

        for (index, row) in indices.iter().zip(&data) {
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
            ColumnsRegion::<CollapseSequence<ConsecutiveOffsetPairs<StringRegion>>>::default();

        let mut indices = Vec::with_capacity(data.len());

        for row in &data {
            let index = r.push(row);
            indices.push(index);
        }

        for (index, row) in indices.iter().zip(&data) {
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

        let mut r = ColumnsRegion::<ConsecutiveOffsetPairs<StringRegion>>::default();

        let mut indices = Vec::with_capacity(data.len());

        for row in &data {
            let index = r.push(row);
            indices.push(index);
        }

        for (index, row) in indices.iter().zip(&data) {
            assert!(row.iter().eq(r.index(index).iter()));
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

        let mut r = ColumnsRegion::<ConsecutiveOffsetPairs<StringRegion>>::default();

        let mut indices = Vec::with_capacity(data.len());

        for row in &data {
            let index = r.push(PushIter(row.iter()));
            indices.push(index);
        }

        for (index, row) in indices.iter().zip(&data) {
            assert!(row.iter().eq(r.index(index).iter()));
        }

        assert_eq!("1", r.index(indices[1]).get(0));
        assert_eq!(1, r.index(indices[1]).len());
        assert!(!r.index(indices[1]).is_empty());
        assert!(r.index(indices[0]).is_empty());

        println!("{r:?}");
    }

    #[test]
    fn read_columns_push() {
        let data = [[[1]; 4]; 4];

        let mut r = <ColumnsRegion<OwnedRegion<u8>>>::default();
        let mut r2 = <ColumnsRegion<OwnedRegion<u8>>>::default();

        for row in &data {
            let idx = r.push(row);
            let idx2 = r2.push(r.index(idx));
            assert!(r.index(idx).iter().eq(r2.index(idx2).iter()));
        }
    }

    #[test]
    #[should_panic]
    fn test_clear() {
        let data = [[[1]; 4]; 4];

        let mut r = <ColumnsRegion<OwnedRegion<u8>>>::default();

        let mut idx = None;
        for row in &data {
            idx = Some(r.push(row));
        }

        r.clear();
        let _ = r.index(idx.unwrap());
    }

    #[test]
    fn copy_reserve_regions() {
        let data = [[[1]; 4]; 4];

        let mut r = <ColumnsRegion<OwnedRegion<u8>>>::default();

        for row in &data {
            let _ = r.push(row);
        }
        for row in data {
            let _ = r.push(row);
        }

        let mut r2 = <ColumnsRegion<OwnedRegion<u8>>>::default();
        r2.reserve_regions(std::iter::once(&r));

        let mut cap = 0;
        r2.heap_size(|_, c| cap += c);
        assert!(cap > 0);
    }

    #[test]
    fn test_merge_regions() {
        let data = [
            vec![],
            vec!["1"],
            vec!["2", "3"],
            vec!["4", "5", "6"],
            vec!["7", "8"],
            vec!["9"],
            vec![],
        ];

        let mut r = ColumnsRegion::<ConsecutiveOffsetPairs<StringRegion>>::default();

        for row in &data {
            let _ = r.push(PushIter(row.iter()));
        }

        let (mut siz1, mut cap1) = (0, 0);
        r.heap_size(|s, c| {
            siz1 += s;
            cap1 += c;
        });

        let mut r2 = ColumnsRegion::merge_regions(std::iter::once(&r));
        for row in &data {
            let _ = r2.push(PushIter(row.iter()));
        }

        let (mut siz2, mut cap2) = (0, 0);
        r2.heap_size(|s, c| {
            siz2 += s;
            cap2 += c;
        });
        assert!(cap2 <= cap1);
    }
}
