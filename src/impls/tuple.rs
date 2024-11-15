//! Regions that stores tuples.

use paste::paste;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Clear, HeapSize, Index, IntoOwned, Len, Push, Region, RegionPreference, ReserveItems};

/// The macro creates the region implementation for tuples
macro_rules! tuple_flatcontainer {
    ($($name:ident)+) => (
        paste! {
            impl<$($name: RegionPreference),*> RegionPreference for ($($name,)*) {
                type Owned = ($($name::Owned,)*);
                type Region = [<Tuple $($name)* Region >]<$($name::Region,)*>;
            }

            /// A region for a tuple.
            #[allow(non_snake_case)]
            #[derive(Default, Debug)]
            #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
            pub struct [<Tuple $($name)* Region >]<$($name),*> {
                $([<container $name>]: $name),*
            }

            #[allow(non_snake_case)]
            impl<$($name: Clone),*> Clone for [<Tuple $($name)* Region>]<$($name),*> {
                fn clone(&self) -> Self {
                    Self {
                        $([<container $name>]: self.[<container $name>].clone(),)*
                    }
                }

                fn clone_from(&mut self, source: &Self) {
                    $(self.[<container $name>].clone_from(&source.[<container $name>]);)*
                }
            }

            #[allow(non_snake_case)]
            impl<$($name: Region),*> Region for [<Tuple $($name)* Region>]<$($name),*> {
                #[inline]
                fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
                where
                    Self: 'a,
                {
                    Self {
                        $([<container $name>]: $name::merge_regions(regions.clone().map(|r| &r.[<container $name>]))),*
                    }
                }
                #[inline(always)]
                fn reserve_regions<'a, It>(&mut self, regions: It)
                where
                    Self: 'a,
                    It: Iterator<Item = &'a Self> + Clone,
                {
                    $(self.[<container $name>].reserve_regions(regions.clone().map(|r| &r.[<container $name>]));)*
                }
            }

            #[allow(non_snake_case)]
            impl<$($name: Index),*> Index for [<Tuple $($name)* Region>]<$($name),*> {
                type Owned = ($($name::Owned,)*);
                type ReadItem<'a> = ($($name::ReadItem<'a>,)*) where Self: 'a;

                #[inline] fn index(&self, index: usize) -> Self::ReadItem<'_> {
                    (
                        $(self.[<container $name>].index(index),)*
                    )
                }

                #[inline]
                fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b> where Self: 'a {
                    let ($($name,)*) = item;
                    (
                        $($name::reborrow($name),)*
                    )
                }
            }

            #[allow(non_snake_case)]
            impl<$($name: Clear),*> Clear for [<Tuple $($name)* Region>]<$($name),*> {
                #[inline(always)]
                fn clear(&mut self) {
                    $(self.[<container $name>].clear();)*
                }
            }

            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            impl<$($name: Len),*> Len for [<Tuple $($name)* Region>]<$($name),*> {
                #[inline(always)]
                fn len(&self) -> usize {
                    $(let len = self.[<container $name>].len();)*
                    len
                }
                #[inline(always)]
                fn is_empty(&self) -> bool {
                    $(let is_empty = self.[<container $name>].is_empty();)*
                    is_empty
                }
            }

            #[allow(non_snake_case)]
            impl<$($name: HeapSize),*> HeapSize for [<Tuple $($name)* Region>]<$($name),*> {
                #[inline]
                fn heap_size<Fn: FnMut(usize, usize)>(&self, mut callback: Fn) {
                    $(self.[<container $name>].heap_size(&mut callback);)*
                }
            }

            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            impl<$($name, [<$name _C>]: Region ),*> Push<($($name,)*)> for [<Tuple $($name)* Region>]<$([<$name _C>]),*>
            where
                $([<$name _C>]: Push<$name>),*
            {
                #[inline]
                fn push(&mut self, item: ($($name,)*)) {
                    let ($($name,)*) = item;
                    $(self.[<container $name>].push($name);)*
                }
            }

            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            impl<'a, $($name, [<$name _C>]),*> Push<&'a ($($name,)*)> for [<Tuple $($name)* Region>]<$([<$name _C>]),*>
            where
                $([<$name _C>]: Push<&'a $name>),*
            {
                #[inline]
                fn push(&mut self, item: &'a ($($name,)*)) {
                    let ($($name,)*) = item;
                    $(self.[<container $name>].push($name);)*
                }
            }

            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            impl<'a, $($name),*> IntoOwned<'a> for ($($name,)*)
            where
                $($name: IntoOwned<'a>),*
            {
                type Owned = ($($name::Owned,)*);

                #[inline]
                fn into_owned(self) -> Self::Owned {
                    let ($($name,)*) = self;
                    (
                        $($name.into_owned(),)*
                    )
                }

                #[inline]
                fn clone_onto(self, other: &mut Self::Owned) {
                    let ($($name,)*) = self;
                    let ($([<$name _other>],)*) = other;
                    $($name.clone_onto([<$name _other>]);)*
                }

                #[inline]
                fn borrow_as(owned: &'a Self::Owned) -> Self {
                    let ($($name,)*) = owned;
                    (
                        $($name::borrow_as($name),)*
                    )
                }
            }

            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            impl<'a, $($name, [<$name _C>]),*> ReserveItems<&'a ($($name,)*)> for [<Tuple $($name)* Region>]<$([<$name _C>]),*>
            where
                $([<$name _C>]: Region + ReserveItems<&'a $name>),*
            {
                #[inline]
                fn reserve_items<It>(&mut self, items: It)
                where
                    It: Iterator<Item = &'a ($($name,)*)> + Clone,
                {
                        tuple_flatcontainer!(reserve_items self items $($name)* @ 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31);
                }
            }

            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            impl<$($name, [<$name _C>]),*> ReserveItems<($($name,)*)> for [<Tuple $($name)* Region>]<$([<$name _C>]),*>
            where
                $([<$name _C>]: Region + ReserveItems<$name>),*
            {
                #[inline]
                fn reserve_items<It>(&mut self, items: It)
                where
                    It: Iterator<Item = ($($name,)*)> + Clone,
                {
                        tuple_flatcontainer!(reserve_items_owned self items $($name)* @ 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31);
                }
            }
        }
    );
    (reserve_items $self:ident $items:ident $name0:ident $($name:ident)* @ $num0:tt $($num:tt)*) => {
        paste! {
            $self.[<container $name0>].reserve_items($items.clone().map(|i| &i.$num0));
            tuple_flatcontainer!(reserve_items $self $items $($name)* @ $($num)*);
        }
    };
    (reserve_items $self:ident $items:ident @ $($num:tt)*) => {};
    (reserve_items_owned $self:ident $items:ident $name0:ident $($name:ident)* @ $num0:tt $($num:tt)*) => {
        paste! {
            $self.[<container $name0>].reserve_items($items.clone().map(|i| i.$num0));
            tuple_flatcontainer!(reserve_items_owned $self $items $($name)* @ $($num)*);
        }
    };
    (reserve_items_owned $self:ident $items:ident @ $($num:tt)*) => {};
}

tuple_flatcontainer!(A);
tuple_flatcontainer!(A B);
tuple_flatcontainer!(A B C);
tuple_flatcontainer!(A B C D);
tuple_flatcontainer!(A B C D E);
tuple_flatcontainer!(A B C D E F);
tuple_flatcontainer!(A B C D E F G);
tuple_flatcontainer!(A B C D E F G H);
tuple_flatcontainer!(A B C D E F G H I);
tuple_flatcontainer!(A B C D E F G H I J);
tuple_flatcontainer!(A B C D E F G H I J K);
tuple_flatcontainer!(A B C D E F G H I J K L);
tuple_flatcontainer!(A B C D E F G H I J K L M);
tuple_flatcontainer!(A B C D E F G H I J K L M N);
tuple_flatcontainer!(A B C D E F G H I J K L M N O);
tuple_flatcontainer!(A B C D E F G H I J K L M N O P);
cfg_if::cfg_if! {
    if #[cfg(not(feature="serde"))] {
        tuple_flatcontainer!(A B C D E F G H I J K L M N O P Q);
        tuple_flatcontainer!(A B C D E F G H I J K L M N O P Q R);
        tuple_flatcontainer!(A B C D E F G H I J K L M N O P Q R S);
        tuple_flatcontainer!(A B C D E F G H I J K L M N O P Q R S T);
        tuple_flatcontainer!(A B C D E F G H I J K L M N O P Q R S T U);
        tuple_flatcontainer!(A B C D E F G H I J K L M N O P Q R S T U V);
        tuple_flatcontainer!(A B C D E F G H I J K L M N O P Q R S T U V W);
        tuple_flatcontainer!(A B C D E F G H I J K L M N O P Q R S T U V W X);
        tuple_flatcontainer!(A B C D E F G H I J K L M N O P Q R S T U V W X Y);
        tuple_flatcontainer!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);
        tuple_flatcontainer!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z AA);
        tuple_flatcontainer!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z AA AB);
        tuple_flatcontainer!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z AA AB AC);
        tuple_flatcontainer!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z AA AB AC AD);
        tuple_flatcontainer!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z AA AB AC AD AE);
        tuple_flatcontainer!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z AA AB AC AD AE AF);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Push, StringRegion};

    use super::*;

    #[test]
    fn test_tuple() {
        let t = (1, 2, 3);
        let mut r = <TupleABCRegion<Vec<_>, Vec<_>, Vec<_>>>::default();
        r.push(t);
        assert_eq!((&1, &2, &3), r.index(0));

        r.push((&1, &2, &4));
        assert_eq!((&1, &2, &4), r.index(1));

        r.push((&1, 2, 5));
        assert_eq!((&1, &2, &5), r.index(2));

        r.push(&(1, 2, 6));
        assert_eq!((&1, &2, &6), r.index(3));

        r.push(&(1, &2, 7));
        assert_eq!((&1, &2, &7), r.index(4));
    }

    #[test]
    fn test_nested() {
        let t = ("abc", 2, 3);
        let mut r = <TupleABCRegion<StringRegion, Vec<_>, Vec<_>>>::default();
        r.push(t);
        assert_eq!(("abc", &2, &3), r.index(0));

        r.push((&"abc", &2, &3));
        assert_eq!(("abc", &2, &3), r.index(1));

        r.push((&"abc", 2, 3));
        assert_eq!(("abc", &2, &3), r.index(2));

        r.push(&("abc", 2, 3));
        assert_eq!(("abc", &2, &3), r.index(3));
    }

    #[test]
    fn test_heap_size() {
        let t = ("abc", 2, 3);
        let mut r = <TupleABCRegion<StringRegion, Vec<_>, Vec<_>>>::default();

        let _ = r.push(t);

        let (mut size, mut cap, mut cnt) = (0, 0, 0);
        r.heap_size(|siz, ca| {
            size += siz;
            cap += ca;
            cnt += 1;
        });

        println!("size {size} cap {cap} cnt {cnt}");

        assert!(size > 0);
        assert!(cap > 0);
        assert!(cnt > 0);
    }
    #[test]
    fn test_reserve_items() {
        let mut c = <(usize, String, Vec<String>) as RegionPreference>::Region::default();
        c.push((1, format!("Hello"), &["abc"]));

        let mut c2 = <(usize, String, Vec<String>) as RegionPreference>::Region::default();
        c2.reserve_items(c.iter());
    }
}
