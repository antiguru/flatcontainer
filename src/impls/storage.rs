//! Storage abstractions to represent slices of data.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// TODO
pub trait Storage<T>: Default {
    /// TODO
    fn with_capacity(capacity: usize) -> Self;
    /// TODO
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a;
    /// TODO
    fn reserve(&mut self, additional: usize);
    /// TODO
    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone;
    /// TODO
    fn clear(&mut self);
    /// TODO
    fn heap_size<F: FnMut(usize, usize)>(&self, callback: F);
    /// TODO
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) -> usize;
    /// TODO
    fn extend_from_slice(&mut self, slice: &[T]) -> usize
    where
        T: Clone;
    /// TODO
    fn index(&self, start: usize, end: usize) -> &[T];
}

impl<T> Storage<T> for Vec<T> {
    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }

    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self::with_capacity(regions.map(Vec::len).sum())
    }

    fn reserve(&mut self, additional: usize) {
        Vec::reserve(self, additional);
    }

    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.reserve(regions.map(Vec::len).sum());
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        let size_of_t = std::mem::size_of::<T>();
        callback(self.len() * size_of_t, self.capacity() * size_of_t);
    }

    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) -> usize {
        Extend::extend(self, iter);
        self.len()
    }

    fn extend_from_slice(&mut self, slice: &[T]) -> usize
    where
        T: Clone,
    {
        self.extend_from_slice(slice);
        self.len()
    }

    fn index(&self, start: usize, end: usize) -> &[T] {
        &self[start..end]
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct Doubling<T> {
    inner: Vec<Vec<T>>,
    offsets: Vec<usize>,
}

impl<T> Default for Doubling<T> {
    fn default() -> Self {
        Self {
            inner: Vec::default(),
            offsets: Vec::default(),
        }
    }
}

impl<T: Clone> Storage<T> for Doubling<T> {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: vec![Vec::with_capacity(capacity)],
            offsets: Vec::default(),
        }
    }

    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self::with_capacity(regions.flat_map(|r| &r.inner).map(Vec::len).sum())
    }

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

    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.reserve(regions.flat_map(|r| &r.inner).map(Vec::len).sum());
    }

    fn clear(&mut self) {
        self.inner.clear();
    }

    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        let size_of_t = std::mem::size_of::<T>();
        for inner in &self.inner {
            callback(inner.len() * size_of_t, inner.capacity() * size_of_t);
        }
    }

    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) -> usize {
        let vec: Vec<T> = iter.into_iter().collect();
        self.extend_from_slice(&vec)
    }

    fn extend_from_slice(&mut self, slice: &[T]) -> usize
    where
        T: Clone,
    {
        self.reserve(slice.len());
        self.inner.last_mut().unwrap().extend_from_slice(slice);
        *self.offsets.last().unwrap_or(&0) + self.inner.last().unwrap().len()
    }

    fn index(&self, start: usize, end: usize) -> &[T] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doubling() {
        let mut d: Doubling<usize> = Doubling::default();
        let mut start = 0;

        for i in 0..1000 {
            let end = d.extend_from_slice(&[i, i + 1, i + 3]);
            assert_eq!(&[i, i + 1, i + 3], d.index(start, end));
            start = end;
        }
    }
}
