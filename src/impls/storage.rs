//! Storage abstractions to represent slices of data.

use crate::CopyIter;

/// Behavior to allocate storage.
///
/// This trait does not express opinions on how to populate itself and how to extract data. Clients
/// should use the [`PushStorage`] trait to insert data into storage, and appropriate
/// [`Index`](std::ops::Index) bounds to extract data.
pub trait Storage<T>: Default {
    /// Allocate storage for at least `capacity` elements.
    #[must_use]
    fn with_capacity(capacity: usize) -> Self;

    /// Allocate storage large enough to absorb `regions`'s contents.
    #[must_use]
    #[inline]
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self::with_capacity(regions.map(Self::len).sum())
    }

    /// Reserve space for `additional` elements.
    fn reserve(&mut self, additional: usize);

    /// Reserve space for `regions`.
    #[inline]
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

    /// Returns the number of elements.
    #[must_use]
    fn len(&self) -> usize;

    /// Returns `true` if empty, i.e., it doesn't contain any elements.
    #[must_use]
    fn is_empty(&self) -> bool;
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

/// Push an item into storage.
pub trait PushStorage<T> {
    /// Push an item into storage.
    fn push_storage(&mut self, item: T);
}

impl<T> PushStorage<&mut Vec<T>> for Vec<T> {
    #[inline]
    fn push_storage(&mut self, item: &mut Vec<T>) {
        self.append(item);
    }
}

impl<T: Clone> PushStorage<&[T]> for Vec<T> {
    #[inline]
    fn push_storage(&mut self, item: &[T]) {
        self.extend_from_slice(item);
    }
}

impl<I: IntoIterator<Item = T>, T> PushStorage<CopyIter<I>> for Vec<T> {
    #[inline]
    fn push_storage(&mut self, item: CopyIter<I>) {
        self.extend(item.0);
    }
}
