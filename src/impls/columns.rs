//! A region to contain a variable number of columns.

use std::fmt::Debug;
use std::iter::Zip;
use std::slice::Iter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Clear, HeapSize, Index, IntoOwned, Len, PushIter, PushSlice};
use crate::{Push, Region};

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
/// # use flatcontainer::{ColumnsRegion, Index, OwnedRegion, Push, Region, StringRegion};
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
/// let mut r = <ColumnsRegion<StringRegion, OwnedRegion<usize>>>::default();
///
/// for row in &data {
///     r.push(row);
/// }
///
/// for (index, row) in data.iter().enumerate() {
///     assert!(row.iter().copied().eq(r.index(index).iter()));
/// }
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ColumnsRegion<R, O> {
    /// Indices to address rows in `inner`. For each row, we remember
    /// an index for each column.
    indices: O,
    /// Storage for columns.
    inner: Vec<R>,
}

impl<R: Clone, O: Clone> Clone for ColumnsRegion<R, O> {
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

impl<R: Region + Default, O: Region> Region for ColumnsRegion<R, O> {
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
            indices: O::merge_regions(regions.map(|r| &r.indices)),
            inner,
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
            inner.reserve_regions(regions.clone().filter_map(|r| r.inner.get(index)));
        }

        self.indices.reserve_regions(regions.map(|r| &r.indices));
    }
}

impl<R: Default, O: Default> Default for ColumnsRegion<R, O> {
    fn default() -> Self {
        Self {
            indices: O::default(),
            inner: Vec::default(),
        }
    }
}

impl<R: HeapSize, O: HeapSize> HeapSize for ColumnsRegion<R, O> {
    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        self.inner.heap_size(&mut callback);
        for inner in &self.inner {
            inner.heap_size(&mut callback);
        }
        self.indices.heap_size(callback);
    }
}

impl<R: Clear, O: Clear> Clear for ColumnsRegion<R, O> {
    fn clear(&mut self) {
        for inner in &mut self.inner {
            inner.clear();
        }
        self.indices.clear();
    }
}

impl<R, O: Len> Len for ColumnsRegion<R, O> {
    fn len(&self) -> usize {
        self.indices.len()
    }

    fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }
}

impl<R, O> Index for ColumnsRegion<R, O>
where
    R: Index,
    for<'a> O: Index<ReadItem<'a> = &'a [usize]> + 'a,
{
    type Owned = Vec<R::Owned>;
    type ReadItem<'a> = ReadColumns<'a, R, R::Owned> where Self: 'a;

    fn index(&self, index: usize) -> Self::ReadItem<'_> {
        ReadColumns(Ok(ReadColumnsInner {
            columns: &self.inner,
            index: self.indices.index(index),
        }))
    }

    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item
    }
}

/// Read the values of a row.
pub struct ReadColumns<'a, R, O>(Result<ReadColumnsInner<'a, R>, &'a [O]>);

struct ReadColumnsInner<'a, R> {
    /// Storage for columns.
    columns: &'a [R],
    /// Indices to retrieve values from columns.
    index: &'a [usize],
}

impl<'a, R, O> Clone for ReadColumns<'a, R, O> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, R> Clone for ReadColumnsInner<'a, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, R, O> Copy for ReadColumns<'a, R, O> {}
impl<'a, R> Copy for ReadColumnsInner<'a, R> {}

impl<'a, R, O> Debug for ReadColumns<'a, R, O>
where
    R: Index<Owned = O>,
    R::ReadItem<'a>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<'a, R, O> ReadColumns<'a, R, O>
where
    R: Index<Owned = O>,
{
    /// Iterate the individual values of a row.
    #[must_use]
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
}
impl<'a, R, O> Len for ReadColumns<'a, R, O> {
    /// Returns the length of this row.
    fn len(&self) -> usize {
        match &self.0 {
            Ok(inner) => inner.len(),
            Err(slice) => slice.len(),
        }
    }

    /// Returns `true` if this row is empty.
    fn is_empty(&self) -> bool {
        match &self.0 {
            Ok(inner) => inner.is_empty(),
            Err(slice) => slice.is_empty(),
        }
    }
}
impl<'a, R> ReadColumnsInner<'a, R>
where
    R: Index,
{
    /// Get the element at `offset`.
    #[must_use]
    pub fn get(&self, offset: usize) -> R::ReadItem<'a> {
        self.columns[offset].index(self.index[offset])
    }
}
impl<'a, R> Len for ReadColumnsInner<'a, R> {
    fn len(&self) -> usize {
        self.index.len()
    }

    fn is_empty(&self) -> bool {
        self.index.is_empty()
    }
}

impl<'a, R, O> IntoOwned<'a> for ReadColumns<'a, R, O>
where
    R: Index<Owned = O>,
{
    type Owned = Vec<O>;

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
    R: Index<Owned = O>,
{
    type Item = R::ReadItem<'a>;
    type IntoIter = ReadColumnsIter<'a, R, O>;

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
pub struct ReadColumnsIter<'a, R, O>(Result<ReadColumnsIterInner<'a, R>, Iter<'a, O>>);

/// An iterator over the elements of a row.
pub struct ReadColumnsIterInner<'a, R> {
    iter: Zip<Iter<'a, usize>, Iter<'a, R>>,
}

impl<'a, R, O> Iterator for ReadColumnsIter<'a, R, O>
where
    R: Index<Owned = O>,
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

impl<'a, R, O> ExactSizeIterator for ReadColumnsIter<'a, R, O> where R: Index<Owned = O> {}

impl<'a, R> Iterator for ReadColumnsIterInner<'a, R>
where
    R: Index,
{
    type Item = R::ReadItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(&i, r)| {
            println!("i: {i}");
            r.index(i)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<R, O, Owned> Push<ReadColumns<'_, R, Owned>> for ColumnsRegion<R, O>
where
    R: Default + Index<Owned = Owned> + Len + for<'a> Push<<R as Index>::ReadItem<'a>>,
    O: PushSlice<usize>,
{
    fn push(&mut self, item: ReadColumns<'_, R, Owned>) {
        // Ensure all required regions exist.
        while self.inner.len() < item.len() {
            self.inner.push(R::default());
        }

        for (value, region) in item.iter().zip(&mut self.inner) {
            region.push(value);
        }
        self.indices
            .push_iter(self.inner.iter().take(item.len()).map(|r| r.len() - 1));
    }
}

impl<'a, R, O, T> Push<&'a [T]> for ColumnsRegion<R, O>
where
    R: Default + Len + Push<&'a T>,
    O: PushSlice<usize>,
{
    fn push(&mut self, item: &'a [T]) {
        // Ensure all required regions exist.
        while self.inner.len() < item.len() {
            self.inner.push(R::default());
        }

        for (value, region) in item.iter().zip(&mut self.inner) {
            region.push(value);
        }
        self.indices
            .push_iter(self.inner.iter().take(item.len()).map(|r| r.len() - 1));
    }
}

impl<R, O, T, const N: usize> Push<[T; N]> for ColumnsRegion<R, O>
where
    R: Default + Len + Push<T>,
    O: PushSlice<usize>,
{
    fn push(&mut self, item: [T; N]) {
        // Ensure all required regions exist.
        while self.inner.len() < item.len() {
            self.inner.push(R::default());
        }

        let columns = item.len();
        for (value, region) in item.into_iter().zip(&mut self.inner) {
            region.push(value);
        }
        self.indices
            .push_iter(self.inner.iter().take(columns).map(|r| r.len() - 1));
    }
}

impl<'a, R, O, T, const N: usize> Push<&'a [T; N]> for ColumnsRegion<R, O>
where
    R: Default + Len + Push<&'a T>,
    O: PushSlice<usize>,
{
    fn push(&mut self, item: &'a [T; N]) {
        // Ensure all required regions exist.
        while self.inner.len() < item.len() {
            self.inner.push(R::default());
        }

        for (value, region) in item.into_iter().zip(&mut self.inner) {
            region.push(value);
        }
        self.indices
            .push_iter(self.inner.iter().take(item.len()).map(|r| r.len() - 1));
    }
}

impl<R, O, T> Push<Vec<T>> for ColumnsRegion<R, O>
where
    R: Default + Len + Push<T>,
    O: PushSlice<usize>,
{
    fn push(&mut self, item: Vec<T>) {
        // Ensure all required regions exist.
        while self.inner.len() < item.len() {
            self.inner.push(R::default());
        }

        let columns = item.len();
        for (value, region) in item.into_iter().zip(&mut self.inner) {
            region.push(value);
        }
        self.indices
            .push_iter(self.inner.iter().take(columns).map(|r| r.len() - 1));
    }
}

impl<'a, R, O, T> Push<&'a Vec<T>> for ColumnsRegion<R, O>
where
    R: Default + Len + Push<&'a T>,
    O: PushSlice<usize>,
{
    fn push(&mut self, item: &'a Vec<T>) {
        // Ensure all required regions exist.
        while self.inner.len() < item.len() {
            self.inner.push(R::default());
        }

        for (value, region) in item.into_iter().zip(&mut self.inner) {
            region.push(value);
        }
        self.indices
            .push_iter(self.inner.iter().take(item.len()).map(|r| r.len() - 1));
    }
}

impl<R, O, T, I> Push<PushIter<I>> for ColumnsRegion<R, O>
where
    R: Default + Len + Push<T>,
    I: IntoIterator<Item = T>,
    O: PushSlice<usize>,
{
    #[inline]
    fn push(&mut self, item: PushIter<I>) {
        let mut columns = 0;
        for (index, value) in item.0.into_iter().enumerate() {
            // Ensure all required regions exist.
            if self.inner.len() <= index {
                self.inner.push(R::default());
            }
            self.inner[index].push(value);
            columns += 1;
        }
        self.indices
            .push_iter(self.inner.iter().take(columns).map(|r| r.len() - 1));
    }
}

#[cfg(test)]
mod tests {
    use crate::{OwnedRegion, Push, PushIter, Region, StringRegion};

    use super::*;

    #[test]
    fn test_matrix() {
        let data = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

        let mut r = ColumnsRegion::<Vec<_>, OwnedRegion<usize>>::default();

        for row in &data {
            // r.push(row.as_slice());
            Push::push(&mut r, row);
            // r.push(row.as_slice());
        }

        for (read, row) in r.iter().zip(&data) {
            assert!(row.iter().eq(read.iter()));
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

        let mut r = ColumnsRegion::<Vec<_>, OwnedRegion<usize>>::default();

        for row in &data {
            r.push(*row);
        }

        for (read, row) in r.iter().zip(&data) {
            assert!(row.iter().eq(read.iter()));
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

        let mut r = ColumnsRegion::<StringRegion, OwnedRegion<usize>>::default();

        for row in &data {
            r.push(row);
        }

        for (read, row) in r.iter().zip(&data) {
            assert!(row.iter().eq(read.iter()));
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

        let mut r = ColumnsRegion::<StringRegion, OwnedRegion<usize>>::default();

        for row in &data {
            r.push(row);
        }

        for (read, row) in r.iter().zip(&data) {
            assert!(row.iter().copied().eq(read.iter()));
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

        let mut r = ColumnsRegion::<StringRegion, OwnedRegion<usize>>::default();

        for row in &data {
            r.push(PushIter(row.iter()));
        }

        for (read, row) in r.iter().zip(&data) {
            println!("{read:?} {row:?}");
            assert!(row.iter().copied().eq(read.iter()));
        }

        assert_eq!("1", r.index(1).get(0));
        assert_eq!(1, r.index(1).len());
        assert!(!r.index(1).is_empty());
        assert!(r.index(0).is_empty());

        println!("{r:?}");
    }

    #[test]
    fn read_columns_push() {
        let data = [[[1]; 4]; 4];

        let mut r = <ColumnsRegion<OwnedRegion<u8>, OwnedRegion<usize>>>::default();
        let mut r2 = <ColumnsRegion<OwnedRegion<u8>, OwnedRegion<usize>>>::default();

        for row in &data {
            r.push(row);
            println!("{r:?}");
            r2.push(r.index(r.len() - 1));
            println!("{r2:?}");
            assert!(r
                .index(r.len() - 1)
                .iter()
                .eq(r2.index(r2.len() - 1).iter()));
        }
    }

    #[test]
    #[should_panic]
    fn test_clear() {
        let data = [[[1]; 4]; 4];

        let mut r = <ColumnsRegion<OwnedRegion<u8>, OwnedRegion<usize>>>::default();

        for row in &data {
            r.push(row);
        }

        r.clear();
        let _ = r.index(0);
    }

    #[test]
    fn copy_reserve_regions() {
        let data = [[[1]; 4]; 4];

        let mut r = <ColumnsRegion<OwnedRegion<u8>, OwnedRegion<usize>>>::default();

        for row in &data {
            r.push(row);
        }
        for row in data {
            r.push(row);
        }

        let mut r2 = <ColumnsRegion<OwnedRegion<u8>, OwnedRegion<usize>>>::default();
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

        let mut r = ColumnsRegion::<StringRegion, OwnedRegion<usize>>::default();

        for row in &data {
            r.push(PushIter(row.iter()));
        }

        let (mut siz1, mut cap1) = (0, 0);
        r.heap_size(|s, c| {
            siz1 += s;
            cap1 += c;
        });

        let mut r2 = ColumnsRegion::merge_regions(std::iter::once(&r));
        for row in &data {
            r2.push(PushIter(row.iter()));
        }

        let (mut siz2, mut cap2) = (0, 0);
        r2.heap_size(|s, c| {
            siz2 += s;
            cap2 += c;
        });
        assert!(cap2 <= cap1);
    }
}
