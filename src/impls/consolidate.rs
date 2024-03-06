//! Consolidating regions.
//!
//! Consolidation is difficult in flat containers because:
//! * Sorting data is an odd operation. We can only permute the indexes, but
//!   not the region that resolve indexes.
//! * Permuting the index prevents sequential access through regions, which
//!   slows down scanning the data.
//! * It is the user's choice to destroy an existing sequential index or not.
//! * The actual consolidation needs to copy data to release data from regions. Or,
//!   it sits on inaccessible data until it's dropped, which may not be so bad.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::impls::tuple::TupleABRegion;
use crate::{CopyOnto, FlatStack, MirrorRegion, Region};

/// TODO
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        bound = "R: Serialize + for<'a> Deserialize<'a>, R::Index: Serialize + for<'a> Deserialize<'a>"
    )
)]
pub struct Consolidating<R: Region> {
    region: R,
    indices: Vec<R::Index>,
}

impl<R: Region> Default for Consolidating<R> {
    #[inline]
    fn default() -> Self {
        Self {
            indices: Vec::default(),
            region: R::default(),
        }
    }
}

impl<R: Region> From<FlatStack<R>> for Consolidating<R> {
    fn from(stack: FlatStack<R>) -> Self {
        let (region, indices) = stack.into_parts();
        Self { region, indices }
    }
}

impl<R: Region> Consolidating<R> {
    /// Returns the element at the `offset` position.
    #[inline]
    #[must_use]
    pub fn get(&self, offset: usize) -> R::ReadItem<'_> {
        self.region.index(self.indices[offset])
    }

    /// Returns the number of indices in the stack.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.indices.len()
    }

    /// TODO
    pub fn into_flat_stack(self) -> FlatStack<R> {
        FlatStack {
            region: self.region,
            indices: self.indices,
        }
    }

    /// Iterate the items in this stack.
    pub fn iter(&self) -> Iter<'_, R> {
        self.into_iter()
    }
}

impl<'a, R: Region> IntoIterator for &'a Consolidating<R> {
    type Item = R::ReadItem<'a>;
    type IntoIter = Iter<'a, R>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self.indices.iter(),
            region: &self.region,
        }
    }
}

/// An iterator over [`Consolidating`]. The iterator yields [`Region::ReadItem`] elements, which
/// it obtains by looking up indices.
pub struct Iter<'a, R: Region> {
    /// Iterator over indices.
    inner: std::slice::Iter<'a, R::Index>,
    /// Region to map indices to read items.
    region: &'a R,
}

impl<'a, R: Region> Iterator for Iter<'a, R> {
    type Item = R::ReadItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|idx| self.region.index(*idx))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<R: Region> Consolidating<TupleABRegion<R, MirrorRegion<i64>>>
where
    for<'a> R::ReadItem<'a>: Ord + Eq + CopyOnto<R>,
    for<'a, 'b> &'b R::ReadItem<'a>: CopyOnto<R>,
{
    /// TODO
    pub fn sort(&mut self) {
        self.indices
            .sort_by(|x, y| Ord::cmp(&self.region.index(*x).0, &self.region.index(*y).0));
    }

    /// Consolidate a sorted representation.
    pub fn consolidate(&self) -> Self {
        let mut new = Self {
            indices: Vec::new(),
            region: <TupleABRegion<R, MirrorRegion<i64>>>::default(),
        };

        let mut reference = None;

        for index in 0..self.indices.len() {
            if index == 0 {
                reference = Some(self.region.index(self.indices[index]));
            } else if let Some(ref_diff) = reference.as_mut() {
                let (item, d) = self.region.index(self.indices[index]);
                if ref_diff.0 == item {
                    ref_diff.1 += d;
                } else {
                    // emit reference item
                    if ref_diff.1 != 0 {
                        let index = (&*ref_diff).copy_onto(&mut new.region);
                        new.indices.push(index);
                    }
                    reference = Some((item, d));
                }
            }
        }
        if let Some(ref_diff) = reference.take() {
            if ref_diff.1 != 0 {
                let index = ref_diff.copy_onto(&mut new.region);
                new.indices.push(index);
            }
        }
        new
    }

    /// TODO
    pub fn copy<A>(&mut self, item: &(A, i64))
    where
        for<'a> R::ReadItem<'a>: PartialEq<A>,
        for<'a> &'a A: CopyOnto<R>,
    {
        if let Some((region_index, diff)) = self.indices.last_mut() {
            let (last_item, _) = self.region.index((*region_index, *diff));
            if last_item == item.0 {
                *diff += item.1;

                if *diff == 0 {
                    self.indices.pop();
                }
                return;
            }
        }

        let index = item.copy_onto(&mut self.region);
        self.indices.push(index);
    }
}

impl<R: Region> std::fmt::Debug for Consolidating<R>
where
    for<'a> R::ReadItem<'a>: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::impls::tuple::TupleABRegion;
    use crate::MirrorRegion;

    use super::*;

    #[test]
    fn consolidate_t2() {
        let mut c: Consolidating<TupleABRegion<MirrorRegion<u8>, MirrorRegion<i64>>> =
            Consolidating::default();
        c.copy(&(1, 1));
        c.copy(&(1, 1));

        assert_eq!(c.len(), 1);
        assert_eq!(c.get(0), (1, 2));

        c.copy(&(1, -2));

        assert_eq!(c.len(), 0);
    }

    #[test]
    fn consolidate_t2_sort() {
        let mut fs: FlatStack<TupleABRegion<MirrorRegion<u8>, MirrorRegion<i64>>> =
            FlatStack::default();
        fs.copy(&(1, 1));
        fs.copy(&(2, 1));
        fs.copy(&(1, 1));
        fs.copy(&(2, 1));
        fs.copy(&(1, -2));
        fs.copy(&(2, -1));

        let mut c: Consolidating<_> = fs.into();
        c.sort();
        let c = c.consolidate();

        println!("result: {c:?}");

        assert_eq!(c.len(), 1);
        assert_eq!(c.get(0), (2, 1));
    }
}
