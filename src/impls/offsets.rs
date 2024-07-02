//! Types to represent offsets.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::impls::storage::Storage;

/// A container to store offsets.
pub trait OffsetContainer<T>: Storage<T> {
    /// Iterator over the elements.
    type Iter<'a>: Iterator<Item = T> + Clone
    where
        Self: 'a;

    /// Lookup an index. May panic for invalid indexes.
    fn index(&self, index: usize) -> T;

    /// Accepts a newly pushed element.
    fn push(&mut self, item: T);

    /// Extend from iterator. Must be [`ExactSizeIterator`] to efficiently
    /// pre-allocate.
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)
    where
        I::IntoIter: ExactSizeIterator;

    /// Returns an iterator over the elements.
    fn iter(&self) -> Self::Iter<'_>;
}

/// A container for offsets that can represent strides of offsets.
///
/// Does not implement `OffsetContainer` because it cannot accept arbitrary pushes. Instead,
/// its `push` method returns a boolean to indicate whether the push was successful or not.
///
/// This type can absorb sequences of the form `0, stride, 2 * stride, 3 * stride, ...` and
/// saturates in a repeated last element.
#[derive(Eq, PartialEq, Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OffsetStride {
    /// No push has occurred.
    #[default]
    Empty,
    /// Pushed a single 0.
    Zero,
    /// `Striding(stride, count)`: `count` many steps of stride `stride` have been pushed.
    Striding(usize, usize),
    /// `Saturated(stride, count, reps)`: `count` many steps of stride `stride`, followed by
    /// `reps` repetitions of the last element have been pushed.
    Saturated(usize, usize, usize),
}

impl OffsetStride {
    /// Accepts or rejects a newly pushed element.
    #[must_use]
    #[inline]
    pub fn push(&mut self, item: usize) -> bool {
        match self {
            OffsetStride::Empty => {
                if item == 0 {
                    *self = OffsetStride::Zero;
                    true
                } else {
                    false
                }
            }
            OffsetStride::Zero => {
                *self = OffsetStride::Striding(item, 2);
                true
            }
            OffsetStride::Striding(stride, count) => {
                if item == *stride * *count {
                    *count += 1;
                    true
                } else if item == *stride * (*count - 1) {
                    *self = OffsetStride::Saturated(*stride, *count, 1);
                    true
                } else {
                    false
                }
            }
            OffsetStride::Saturated(stride, count, reps) => {
                if item == *stride * (*count - 1) {
                    *reps += 1;
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Lookup the element at `index`.
    ///
    /// # Panics
    ///
    /// Panics for out-of-bounds accesses, i.e., if `index` greater or equal to
    /// [`len`][OffsetStride::len].
    #[must_use]
    #[inline]
    pub fn index(&self, index: usize) -> usize {
        match self {
            OffsetStride::Empty => {
                panic!("Empty OffsetStride")
            }
            OffsetStride::Zero => 0,
            OffsetStride::Striding(stride, _steps) => *stride * index,
            OffsetStride::Saturated(stride, steps, _reps) => {
                if index < *steps {
                    *stride * index
                } else {
                    *stride * (*steps - 1)
                }
            }
        }
    }

    /// Returns the number of elements.
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            OffsetStride::Empty => 0,
            OffsetStride::Zero => 1,
            OffsetStride::Striding(_stride, steps) => *steps,
            OffsetStride::Saturated(_stride, steps, reps) => *steps + *reps,
        }
    }

    /// Returns `true` if empty.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        matches!(self, OffsetStride::Empty)
    }

    /// Removes all elements.
    #[inline]
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Return an iterator over the elements.
    #[must_use]
    #[inline]
    pub fn iter(&self) -> OffsetStrideIter {
        OffsetStrideIter {
            strided: *self,
            index: 0,
        }
    }
}

/// An iterator over the elements of an [`OffsetStride`].
#[derive(Clone, Copy)]
pub struct OffsetStrideIter {
    strided: OffsetStride,
    index: usize,
}

impl Iterator for OffsetStrideIter {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.strided.len() {
            let item = self.strided.index(self.index);
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

/// A list of unsigned integers that uses `u32` elements as long as they are small enough, and switches to `u64` once they are not.
#[derive(Eq, PartialEq, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OffsetList<S, L> {
    /// Offsets that fit within a `u32`.
    pub smol: S,
    /// Offsets that either do not fit in a `u32`, or are inserted after some offset that did not fit.
    pub chonk: L,
}

impl<S, L> OffsetList<S, L>
where
    S: OffsetContainer<u32>,
    L: OffsetContainer<u64>,
{
    /// Allocate a new list with a specified capacity.
    #[must_use]
    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            smol: S::with_capacity(cap),
            chonk: L::default(),
        }
    }

    /// Inserts the offset, as a `u32` if that is still on the table.
    ///
    /// # Panics
    ///
    /// Panics if `usize` does not fit in `u64`.
    #[inline]
    pub fn push(&mut self, offset: usize) {
        if self.chonk.is_empty() {
            if let Ok(smol) = offset.try_into() {
                self.smol.push(smol);
            } else {
                self.chonk.push(offset.try_into().unwrap());
            }
        } else {
            self.chonk.push(offset.try_into().unwrap());
        }
    }

    /// Like [`std::ops::Index`], which we cannot implement as it must return a `&usize`.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds, i.e., it is larger or equal to the length.
    #[must_use]
    #[inline]
    pub fn index(&self, index: usize) -> usize {
        if index < self.smol.len() {
            self.smol.index(index).try_into().unwrap()
        } else {
            let index = index - self.smol.len();
            self.chonk.index(index).try_into().unwrap()
        }
    }
    /// The number of offsets in the list.
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.smol.len() + self.chonk.len()
    }

    /// Returns `true` if this list contains no elements.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.smol.is_empty() && self.chonk.is_empty()
    }

    /// Reserve space for `additional` elements.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.smol.reserve(additional);
    }

    /// Remove all elements.
    #[inline]
    pub fn clear(&mut self) {
        self.smol.clear();
        self.chonk.clear();
    }

    #[inline]
    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        self.smol.heap_size(&mut callback);
        self.chonk.heap_size(callback);
    }
}

impl<S, L> Storage<usize> for OffsetList<S, L>
where
    S: OffsetContainer<u32>,
    L: OffsetContainer<u64>,
{
    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        self.reserve(additional)
    }

    #[inline]
    fn clear(&mut self) {
        self.clear()
    }

    #[inline]
    fn heap_size<F: FnMut(usize, usize)>(&self, callback: F) {
        self.heap_size(callback)
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

impl<S, L> OffsetContainer<usize> for OffsetList<S, L>
where
    S: OffsetContainer<u32>,
    L: OffsetContainer<u64>,
{
    type Iter<'a> = OffsetListIter<S::Iter<'a>, L::Iter<'a>> where Self: 'a;

    #[inline]
    fn index(&self, index: usize) -> usize {
        self.index(index)
    }

    #[inline]
    fn push(&mut self, item: usize) {
        self.push(item)
    }

    #[inline]
    fn extend<I: IntoIterator<Item = usize>>(&mut self, iter: I)
    where
        I::IntoIter: ExactSizeIterator,
    {
        for item in iter {
            self.push(item);
        }
    }

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        OffsetListIter {
            smol: self.smol.iter(),
            chonk: self.chonk.iter(),
        }
    }
}

/// An iterator over the elements of an [`OffsetList`].
#[derive(Clone, Copy)]
pub struct OffsetListIter<S, L> {
    smol: S,
    chonk: L,
}

impl<S, L> Iterator for OffsetListIter<S, L>
where
    S: Iterator<Item = u32>,
    L: Iterator<Item = u64>,
{
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.smol
            .next()
            .map(|x| x as usize)
            .or_else(|| self.chonk.next().map(|x| x as usize))
    }
}

/// An offset container implementation that first tries to recognize strides, and then spilles into
/// a regular offset list.
#[derive(Eq, PartialEq, Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OffsetOptimized<S = Vec<u32>, L = Vec<u64>> {
    strided: OffsetStride,
    spilled: OffsetList<S, L>,
}

impl<S, L> Storage<usize> for OffsetOptimized<S, L>
where
    S: OffsetContainer<u32>,
    L: OffsetContainer<u64>,
{
    #[inline]
    fn with_capacity(_capacity: usize) -> Self {
        // `self.strided` doesn't have any capacity, and we don't know the structure of the data.
        Self::default()
    }

    #[inline]
    fn clear(&mut self) {
        self.spilled.clear();
        self.strided = OffsetStride::default();
    }

    #[inline]
    fn len(&self) -> usize {
        self.strided.len() + self.spilled.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.strided.is_empty() && self.spilled.is_empty()
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        if !self.spilled.is_empty() {
            self.spilled.reserve(additional);
        }
    }

    #[inline]
    fn heap_size<F: FnMut(usize, usize)>(&self, callback: F) {
        self.spilled.heap_size(callback);
    }
}

impl<S, L> OffsetContainer<usize> for OffsetOptimized<S, L>
where
    S: OffsetContainer<u32>,
    L: OffsetContainer<u64>,
{
    type Iter<'a> = OffsetOptimizedIter<S::Iter<'a>, L::Iter<'a>> where Self: 'a;

    fn index(&self, index: usize) -> usize {
        if index < self.strided.len() {
            self.strided.index(index)
        } else {
            self.spilled.index(index - self.strided.len())
        }
    }

    fn push(&mut self, item: usize) {
        if self.spilled.is_empty() {
            let inserted = self.strided.push(item);
            if !inserted {
                self.spilled.push(item);
            }
        } else {
            self.spilled.push(item);
        }
    }

    fn extend<I: IntoIterator<Item = usize>>(&mut self, iter: I)
    where
        I::IntoIter: ExactSizeIterator,
    {
        for item in iter {
            self.push(item);
        }
    }

    fn iter(&self) -> Self::Iter<'_> {
        OffsetOptimizedIter {
            strided: self.strided.iter(),
            spilled: self.spilled.iter(),
        }
    }
}

/// An iterator over the elements of an [`OffsetOptimized`].
#[derive(Clone, Copy)]
pub struct OffsetOptimizedIter<S, L> {
    strided: OffsetStrideIter,
    spilled: OffsetListIter<S, L>,
}

impl<S, L> Iterator for OffsetOptimizedIter<S, L>
where
    S: Iterator<Item = u32>,
    L: Iterator<Item = u64>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.strided.next().or_else(|| self.spilled.next())
    }
}

impl<T: Copy> OffsetContainer<T> for Vec<T> {
    type Iter<'a> = std::iter::Copied<std::slice::Iter<'a, T>> where Self: 'a;

    fn index(&self, index: usize) -> T {
        self[index]
    }

    #[inline]
    fn push(&mut self, item: T) {
        self.push(item);
    }

    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)
    where
        I::IntoIter: ExactSizeIterator,
    {
        Extend::extend(self, iter);
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.as_slice().iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::impls::deduplicate::ConsecutiveOffsetPairs;
    use crate::{Push, Region, SliceRegion, StringRegion};

    use super::*;

    #[test]
    fn test_offset_optimized() {
        fn copy<R: Region + Push<T>, T>(r: &mut R, item: T) -> R::Index {
            r.push(item)
        }

        let mut r = SliceRegion::<
            ConsecutiveOffsetPairs<StringRegion, OffsetOptimized>,
            OffsetOptimized,
        >::default();
        let idx = copy(&mut r, ["abc"]);
        assert_eq!("abc", r.index(idx).get(0))
    }

    #[test]
    fn test_offset_optimized_clear() {
        let mut oo = <OffsetOptimized>::default();
        oo.push(0);
        assert_eq!(oo.len(), 1);
        oo.clear();
        assert_eq!(oo.len(), 0);
        assert!(oo.is_empty());
        oo.push(9999999999);
        assert_eq!(oo.len(), 1);
        oo.clear();
        assert_eq!(oo.len(), 0);
    }

    #[test]
    fn test_offset_optimized_reserve() {
        let mut oo = <OffsetOptimized>::default();
        oo.push(9999999999);
        assert_eq!(oo.len(), 1);
        oo.reserve(1);
    }

    #[test]
    fn test_offset_optimized_heap_size() {
        let mut oo = <OffsetOptimized>::default();
        oo.push(9999999999);
        let mut cap = 0;
        oo.heap_size(|_, ca| {
            cap += ca;
        });
        assert!(cap > 0);
    }

    #[test]
    fn test_offset_stride_push() {
        let mut os = OffsetStride::default();
        assert_eq!(os.len(), 0);
        assert!(os.is_empty());
        assert!(os.push(0));
        assert_eq!(os.index(0), 0);
        assert_eq!(os.len(), 1);
        assert!(!os.is_empty());
        assert!(os.push(2));
        assert_eq!(os.len(), 2);
        assert_eq!(os.index(1), 2);
        assert!(os.push(4));
        assert_eq!(os.len(), 3);
        assert_eq!(os.index(2), 4);
        assert!(os.push(4));
        assert_eq!(os.len(), 4);
        assert_eq!(os.index(3), 4);
        assert!(os.push(4));
        assert_eq!(os.len(), 5);
        assert_eq!(os.index(1), 2);
        assert_eq!(os.index(4), 4);
        assert!(!os.push(5));
        os.clear();
        assert!(!os.push(1));
    }

    #[test]
    fn test_chonk() {
        let mut ol = <OffsetList<Vec<_>, Vec<_>>>::default();
        ol.push(usize::MAX);
        assert_eq!(usize::MAX, ol.index(0));
    }

    #[test]
    #[should_panic]
    fn test_offset_stride_index() {
        let os = OffsetStride::default();
        let _ = os.index(0);
    }
}
