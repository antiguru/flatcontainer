//! A region that negates on read.
//!
//! We do not provide any [`CopyOnto`][crate::CopyOnto] implementation because
//! it is not clearly defined what it would indicate wrt negating on read.

use crate::{FlatStack, Region};

/// TODO
#[derive(Default)]
pub struct Negate<R>(R);

/// TODO
pub trait Negatable {
    /// TODO
    type N;

    /// TODO
    fn negate(self) -> Self::N;
}

impl<A, B: std::ops::Neg> Negatable for (A, B) {
    type N = (A, B::Output);

    fn negate(self) -> Self::N {
        (self.0, -self.1)
    }
}

impl<A, B, C: std::ops::Neg> Negatable for (A, B, C) {
    type N = (A, B, C::Output);

    fn negate(self) -> Self::N {
        (self.0, self.1, -self.2)
    }
}

impl<R: Region> Region for Negate<R>
where
    for<'a> R::ReadItem<'a>: Negatable,
{
    type ReadItem<'a> = <R::ReadItem<'a> as Negatable>::N
    where
        Self: 'a;

    type Index = R::Index;

    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self(R::merge_regions(regions.map(|r| &r.0)))
    }

    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        self.0.index(index).negate()
    }

    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.0.reserve_regions(regions.map(|r| &r.0));
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn heap_size<F: FnMut(usize, usize)>(&self, callback: F) {
        self.0.heap_size(callback);
    }
}

impl<R: Region> FlatStack<R>
where
    for<'a> R::ReadItem<'a>: Negatable,
{
    /// Negate all contents.
    ///
    /// This consumes the stack and returns one with the inner region wrapped
    /// by [`Negate`], which negates elements on read.
    pub fn negate(self) -> FlatStack<Negate<R>> {
        FlatStack {
            indices: self.indices,
            region: Negate(self.region),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::FlatStack;

    use crate::impls::tuple::TupleABCRegion;

    #[test]
    fn negate() {
        let mut fs = <FlatStack<
            TupleABCRegion<crate::CopyRegion<u8>, crate::CopyRegion<u8>, crate::MirrorRegion<i64>>,
        >>::default();
        fs.copy(([1, 2, 3, 4], [5, 6, 7, 8], 1));

        assert_eq!(
            ([1u8, 2, 3, 4].as_slice(), [5u8, 6, 7, 8].as_slice(), 1_i64),
            fs.get(0)
        );

        let neg = fs.negate();
        assert_eq!(
            ([1, 2, 3, 4].as_slice(), [5, 6, 7, 8].as_slice(), -1),
            neg.get(0)
        );
    }
}
