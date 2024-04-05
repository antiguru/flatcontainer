//! Types to represent offsets.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A container to store offsets.
pub trait OffsetContainer<T>: Default + Extend<T> {
    /// Accepts a newly pushed element.
    fn push(&mut self, item: T);

    /// Lookup an index. May panic for invalid indexes.
    fn index(&self, index: usize) -> T;

    /// Clear all contents.
    fn clear(&mut self);

    /// Returns the number of elements.
    fn len(&self) -> usize;

    /// Returns `true` if empty.
    #[inline]
    #[must_use]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Reserve space for `additional` elements.
    fn reserve(&mut self, additional: usize);

    /// Heap size, size - capacity
    fn heap_size<F: FnMut(usize, usize)>(&self, callback: F);
}

/// A container for offsets that can represent strides of offsets.
///
/// Does not implement `OffsetContainer` because it cannot accept arbitrary pushes.
#[derive(Debug, Default)]
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
    pub fn is_empty(&self) -> bool {
        matches!(self, OffsetStride::Empty)
    }

    /// Removes all elements.
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

/// A list of unsigned integers that uses `u32` elements as long as they are small enough, and switches to `u64` once they are not.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OffsetList {
    /// Offsets that fit within a `u32`.
    pub smol: Vec<u32>,
    /// Offsets that either do not fit in a `u32`, or are inserted after some offset that did not fit.
    pub chonk: Vec<u64>,
}

impl OffsetList {
    /// Allocate a new list with a specified capacity.
    #[must_use]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            smol: Vec::with_capacity(cap),
            chonk: Vec::new(),
        }
    }

    /// Inserts the offset, as a `u32` if that is still on the table.
    ///
    /// # Panics
    ///
    /// Panics if `usize` does not fit in `u64`.
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
    pub fn index(&self, index: usize) -> usize {
        if index < self.smol.len() {
            self.smol[index].try_into().unwrap()
        } else {
            self.chonk[index - self.smol.len()].try_into().unwrap()
        }
    }
    /// The number of offsets in the list.
    #[must_use]
    pub fn len(&self) -> usize {
        self.smol.len() + self.chonk.len()
    }

    /// Returns `true` if this list contains no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Reserve space for `additional` elements.
    pub fn reserve(&mut self, additional: usize) {
        self.smol.reserve(additional);
    }

    /// Remove all elements.
    pub fn clear(&mut self) {
        self.smol.clear();
        self.chonk.clear();
    }

    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        self.smol.heap_size(&mut callback);
        self.chonk.heap_size(callback);
    }
}

/// An offset container implementation that first tries to recognize strides, and then spilles into
/// a regular offset list.
#[derive(Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OffsetOptimized {
    strided: OffsetStride,
    spilled: OffsetList,
}

impl OffsetContainer<usize> for OffsetOptimized {
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

    fn index(&self, index: usize) -> usize {
        if index < self.strided.len() {
            self.strided.index(index)
        } else {
            self.spilled.index(index - self.strided.len())
        }
    }

    fn clear(&mut self) {
        self.spilled.clear();
        self.strided = OffsetStride::default();
    }

    fn len(&self) -> usize {
        self.strided.len() + self.spilled.len()
    }

    fn reserve(&mut self, additional: usize) {
        if !self.spilled.is_empty() {
            self.spilled.reserve(additional);
        }
    }

    fn heap_size<F: FnMut(usize, usize)>(&self, callback: F) {
        self.spilled.heap_size(callback);
    }
}

impl Extend<usize> for OffsetOptimized {
    fn extend<T: IntoIterator<Item = usize>>(&mut self, iter: T) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<T: Copy> OffsetContainer<T> for Vec<T> {
    #[inline]
    fn push(&mut self, item: T) {
        self.push(item);
    }

    #[inline]
    #[must_use]
    fn index(&self, index: usize) -> T {
        self[index]
    }

    #[inline]
    fn clear(&mut self) {
        self.clear();
    }

    #[inline]
    #[must_use]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        let size_of_t = std::mem::size_of::<T>();
        callback(self.len() * size_of_t, self.capacity() * size_of_t);
    }
}

#[cfg(test)]
mod tests {
    use crate::impls::deduplicate::ConsecutiveOffsetPairs;
    use crate::{CopyOnto, ReadRegion, Region, SliceRegion, StringRegion};

    use super::*;

    #[test]
    fn test_offset_optimized() {
        fn copy<R: Region>(r: &mut R, item: impl CopyOnto<R>) -> R::Index {
            item.copy_onto(r)
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
        let mut oo = OffsetOptimized::default();
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
        let mut oo = OffsetOptimized::default();
        oo.push(9999999999);
        assert_eq!(oo.len(), 1);
        oo.reserve(1);
    }

    #[test]
    fn test_offset_optimized_heap_size() {
        let mut oo = OffsetOptimized::default();
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
        let mut ol = OffsetList::default();
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
