//! Storage abstractions to represent slices of data.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Behavior to allocate storage
pub trait Storage<T>: Default {
    /// Allocate storage for at least `capacity` elements.
    fn with_capacity(capacity: usize) -> Self;

    /// Allocate storage large enough to absorb `regions`'s contents.
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self::with_capacity(regions.map(Self::len).sum())
    }

    /// Reserve space for `additional` elements.
    fn reserve(&mut self, additional: usize);

    /// Reserve space for `regions`.
    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.reserve(regions.map(Self::len).sum());
    }

    /// Clear all contents, possibly retaining some allocations.
    fn clear(&mut self);

    /// Observe the heap size information (size and capacity).
    fn heap_size<F: FnMut(usize, usize)>(&self, callback: F);

    /// Extend from iterator. Must be [`ExactSizeIterator`] to efficiently
    /// pre-allocate.
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)
    where
        I::IntoIter: ExactSizeIterator;

    /// Append the contents of `data`.
    fn append(&mut self, data: &mut Vec<T>);

    /// Extend from the contents of `slice`.
    fn extend_from_slice(&mut self, slice: &[T])
    where
        T: Clone;

    /// Lookup the slice in range `start..end`.
    fn index(&self, index: usize) -> &T;

    /// Returns the number of elements.
    fn len(&self) -> usize;

    /// Returns `true` if empty, i.e., it doesn't contain any elements.
    fn is_empty(&self) -> bool;
}

/// Behavior for slice storage.
pub trait SliceStorage<T>: Storage<T> {
    /// Lookup the slice in range `start..end`.
    fn index_slice(&self, start: usize, end: usize) -> &[T];
}

impl<T> Storage<T> for Vec<T> {
    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        Vec::reserve(self, additional);
    }

    #[inline]
    fn clear(&mut self) {
        self.clear();
    }

    #[inline]
    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        let size_of_t = std::mem::size_of::<T>();
        callback(self.len() * size_of_t, self.capacity() * size_of_t);
    }

    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        Extend::extend(self, iter);
    }

    #[inline]
    fn append(&mut self, data: &mut Vec<T>) {
        self.append(data);
    }

    #[inline]
    fn extend_from_slice(&mut self, slice: &[T])
    where
        T: Clone,
    {
        self.extend_from_slice(slice);
    }

    #[inline]
    #[must_use]
    fn index(&self, index: usize) -> &T {
        &self[index]
    }

    #[inline]
    #[must_use]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    #[must_use]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> SliceStorage<T> for Vec<T> {
    #[inline]
    #[must_use]
    fn index_slice(&self, start: usize, end: usize) -> &[T] {
        &self[start..end]
    }
}

/// TODO
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Doubling<T> {
    inner: Vec<Vec<T>>,
    offsets: Vec<usize>,
    len: usize,
}

impl<T> Default for Doubling<T> {
    fn default() -> Self {
        Self {
            inner: Vec::default(),
            offsets: Vec::default(),
            len: 0,
        }
    }
}

impl<T> Doubling<T> {
    #[inline]
    #[must_use]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: vec![Vec::with_capacity(capacity)],
            offsets: Vec::default(),
            len: 0,
        }
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        let (remaining, last_len) = self
            .inner
            .last()
            .map_or((0, 0), |last| (last.capacity() - last.len(), last.len()));
        if remaining < additional {
            let len = 2 * last_len;
            let len = std::cmp::max(additional, len);
            let len = len.next_power_of_two();
            self.offsets
                .push(last_len + *self.offsets.last().unwrap_or(&0));
            self.inner.push(Vec::with_capacity(len));
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.inner.drain(..self.len().saturating_sub(1));
        if let Some(last) = self.inner.last_mut() {
            last.clear();
        }
    }

    #[inline]
    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        let size_of_usize = std::mem::size_of::<usize>();
        callback(
            self.offsets.len() * size_of_usize,
            self.offsets.capacity() * size_of_usize,
        );
        let size_of_t = std::mem::size_of::<T>();
        for inner in &self.inner {
            callback(inner.len() * size_of_t, inner.capacity() * size_of_t);
        }
    }

    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        let (lo, hi) = iter.size_hint();
        self.reserve(hi.unwrap_or(lo));
        Extend::extend(self.inner.last_mut().unwrap(), iter);
    }

    #[inline]
    #[must_use]
    fn index(&self, index: usize) -> &T {
        let slice_index = self
            .offsets
            .iter()
            .position(|&o| o > index)
            .unwrap_or_else(|| self.offsets.len().saturating_sub(1));
        let index = index - self.offsets[slice_index];
        &self.inner[slice_index][index]
    }

    #[inline]
    fn len(&self) -> usize {
        *self.offsets.last().unwrap_or(&0) + self.inner.last().map_or(0, Vec::len)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.offsets.last().unwrap_or(&0) > &0 || self.inner.last().map_or(false, Vec::is_empty)
    }
}

impl<T> Storage<T> for Doubling<T> {
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    fn clear(&mut self) {
        self.clear()
    }

    fn heap_size<F: FnMut(usize, usize)>(&self, callback: F) {
        self.heap_size(callback);
    }

    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)
    where
        I::IntoIter: ExactSizeIterator,
    {
        self.extend(iter);
    }

    #[inline]
    fn append(&mut self, data: &mut Vec<T>) {
        self.len += data.len();
        self.reserve(data.len());
        self.inner.last_mut().unwrap().append(data);
    }

    #[inline]
    fn extend_from_slice(&mut self, slice: &[T])
    where
        T: Clone,
    {
        self.len += slice.len();
        self.reserve(slice.len());
        self.inner.last_mut().unwrap().extend_from_slice(slice);
    }

    fn index(&self, index: usize) -> &T {
        self.index(index)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> SliceStorage<T> for Doubling<T> {
    #[inline]
    fn index_slice(&self, start: usize, end: usize) -> &[T] {
        let index = self
            .offsets
            .iter()
            .position(|&o| o > start)
            .unwrap_or_else(|| self.offsets.len().saturating_sub(1));
        let start = start - self.offsets[index];
        let end = end - self.offsets[index];
        &self.inner[index][start..end]
    }
}

mod offsetcontainer {
    use crate::impls::offsets::OffsetContainer;
    use crate::impls::storage::Doubling;

    impl<T: Copy> OffsetContainer<T> for Doubling<T> {
        fn with_capacity(capacity: usize) -> Self {
            Self::with_capacity(capacity)
        }

        fn push(&mut self, item: T) {
            self.len += 1;
            self.reserve(1);
            self.inner.last_mut().unwrap().push(item);
        }

        fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)
        where
            I::IntoIter: ExactSizeIterator,
        {
            self.extend(iter);
        }

        fn index(&self, index: usize) -> T {
            *self.index(index)
        }

        fn clear(&mut self) {
            self.clear()
        }

        fn len(&self) -> usize {
            self.len()
        }

        fn reserve(&mut self, additional: usize) {
            self.reserve(additional);
        }

        fn heap_size<F: FnMut(usize, usize)>(&self, callback: F) {
            self.heap_size(callback);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doubling() {
        let mut d: Doubling<usize> = Doubling::default();
        let mut start = 0;

        for i in 0..1000 {
            d.extend_from_slice(&[i, i + 1, i + 3]);
            let end = d.len();
            assert_eq!(&[i, i + 1, i + 3], d.index_slice(start, end));
            start = end;
        }
    }
}
