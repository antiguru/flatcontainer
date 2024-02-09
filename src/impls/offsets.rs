//! Types to represent offsets.

/// TODO
pub trait OffsetContainer<T>: Default + Extend<T> {
    /// Accepts a newly pushed element.
    fn push(&mut self, item: T);

    /// Lookup an index
    fn index(&self, index: usize) -> T;

    /// Clear all contents.
    fn clear(&mut self);

    /// Returns the number of elements.
    fn len(&self) -> usize;

    /// Returns `true` if empty.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Reserve space for `additional` elements.
    fn reserve(&mut self, additional: usize);
}

#[derive(Debug, Default)]
enum OffsetStride {
    #[default]
    Empty,
    Zero,
    Striding(usize, usize),
    Saturated(usize, usize, usize),
}

impl OffsetStride {
    /// Accepts or rejects a newly pushed element.
    fn push(&mut self, item: usize) -> bool {
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

    fn index(&self, index: usize) -> usize {
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

    fn len(&self) -> usize {
        match self {
            OffsetStride::Empty => 0,
            OffsetStride::Zero => 1,
            OffsetStride::Striding(_stride, steps) => *steps,
            OffsetStride::Saturated(_stride, steps, reps) => *steps + *reps,
        }
    }
}

/// A list of unsigned integers that uses `u32` elements as long as they are small enough, and switches to `u64` once they are not.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Default)]
pub struct OffsetList {
    /// Length of a prefix of zero elements.
    pub zero_prefix: usize,
    /// Offsets that fit within a `u32`.
    pub smol: Vec<u32>,
    /// Offsets that either do not fit in a `u32`, or are inserted after some offset that did not fit.
    pub chonk: Vec<u64>,
}

impl OffsetList {
    // TODO
    // /// Allocate a new list with a specified capacity.
    // pub fn with_capacity(cap: usize) -> Self {
    //     Self {
    //         zero_prefix: 0,
    //         smol: Vec::with_capacity(cap),
    //         chonk: Vec::new(),
    //     }
    // }
    /// Inserts the offset, as a `u32` if that is still on the table.
    pub fn push(&mut self, offset: usize) {
        if self.smol.is_empty() && self.chonk.is_empty() && offset == 0 {
            self.zero_prefix += 1;
        } else if self.chonk.is_empty() {
            if let Ok(smol) = offset.try_into() {
                self.smol.push(smol);
            } else {
                self.chonk.push(offset.try_into().unwrap())
            }
        } else {
            self.chonk.push(offset.try_into().unwrap())
        }
    }
    /// Like `std::ops::Index`, which we cannot implement as it must return a `&usize`.
    pub fn index(&self, index: usize) -> usize {
        if index < self.zero_prefix {
            0
        } else if index - self.zero_prefix < self.smol.len() {
            self.smol[index - self.zero_prefix].try_into().unwrap()
        } else {
            self.chonk[index - self.zero_prefix - self.smol.len()]
                .try_into()
                .unwrap()
        }
    }
    /// The number of offsets in the list.
    pub fn len(&self) -> usize {
        self.zero_prefix + self.smol.len() + self.chonk.len()
    }

    /// Returns `true` if this list contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Reserve space for `additional` elements.
    pub fn reserve(&mut self, additional: usize) {
        self.smol.reserve(additional)
    }

    /// Remove all elements.
    pub fn clear(&mut self) {
        self.smol.clear();
        self.chonk.clear();
    }
}

/// TODO
#[derive(Default, Debug)]
pub struct OffsetOptimized {
    strided: OffsetStride,
    spilled: OffsetList,
}

impl OffsetContainer<usize> for OffsetOptimized {
    fn push(&mut self, item: usize) {
        if !self.spilled.is_empty() {
            self.spilled.push(item);
        } else {
            let inserted = self.strided.push(item);
            if !inserted {
                self.spilled.push(item);
            }
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
        self.push(item)
    }

    #[inline]
    fn index(&self, index: usize) -> T {
        self[index]
    }

    #[inline]
    fn clear(&mut self) {
        self.clear()
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        self.reserve(additional)
    }
}

#[cfg(test)]
mod tests {
    use crate::impls::deduplicate::ConsecutiveOffsetPairs;
    use crate::impls::offsets::OffsetOptimized;
    use crate::{CopyOnto, Region, SliceRegion, StringRegion};

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
}
