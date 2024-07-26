//! Regions that stores tuples.

use paste::paste;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{IntoOwned, Push, Region, RegionPreference, ReserveItems};

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
            impl<$($name: Region + Clone),*> Clone for [<Tuple $($name)* Region>]<$($name),*>
            where
               $(<$name as Region>::Index: crate::Index),*
            {
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
            impl<$($name: Region),*> Region for [<Tuple $($name)* Region>]<$($name),*>
            where
               $(<$name as Region>::Index: crate::Index),*
            {
                type Owned = ($($name::Owned,)*);
                type ReadItem<'a> = ($($name::ReadItem<'a>,)*) where Self: 'a;

                type Index = ($($name::Index,)*);

                #[inline]
                fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
                where
                    Self: 'a,
                {
                    Self {
                        $([<container $name>]: $name::merge_regions(regions.clone().map(|r| &r.[<container $name>]))),*
                    }
                }

                #[inline] fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
                    let ($($name,)*) = index;
                    (
                        $(self.[<container $name>].index($name),)*
                    )
                }

                #[inline(always)]
                fn reserve_regions<'a, It>(&mut self, regions: It)
                where
                    Self: 'a,
                    It: Iterator<Item = &'a Self> + Clone,
                {
                    $(self.[<container $name>].reserve_regions(regions.clone().map(|r| &r.[<container $name>]));)*
                }

                #[inline(always)]
                fn clear(&mut self) {
                    $(self.[<container $name>].clear();)*
                }

                #[inline]
                fn heap_size<Fn: FnMut(usize, usize)>(&self, mut callback: Fn) {
                    $(self.[<container $name>].heap_size(&mut callback);)*
                }

                #[inline]
                fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b> where Self: 'a {
                    let ($($name,)*) = item;
                    (
                        $($name::reborrow($name),)*
                    )
                }
            }

            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            impl<$($name, [<$name _C>]: Region ),*> Push<($($name,)*)> for [<Tuple $($name)* Region>]<$([<$name _C>]),*>
            where
                $([<$name _C>]: Push<$name>),*
            {
                #[inline]
                fn push(&mut self, item: ($($name,)*))
                    -> <[<Tuple $($name)* Region>]<$([<$name _C>]),*> as Region>::Index {
                    let ($($name,)*) = item;
                    ($(self.[<container $name>].push($name),)*)
                }
            }

            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            impl<'a, $($name, [<$name _C>]),*> Push<&'a ($($name,)*)> for [<Tuple $($name)* Region>]<$([<$name _C>]),*>
            where
                $([<$name _C>]: Region + Push<&'a $name>),*
            {
                #[inline]
                fn push(&mut self, item: &'a ($($name,)*))
                    -> <[<Tuple $($name)* Region>]<$([<$name _C>]),*> as Region>::Index {
                    let ($($name,)*) = item;
                    ($(self.[<container $name>].push($name),)*)
                }
            }

            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            impl<$($name, [<$name _C>]),*> crate::CanPush<($($name,)*)> for [<Tuple $($name)* Region>]<$([<$name _C>]),*>
            where
                $([<$name _C>]: Region + crate::CanPush<$name>),*
            {
                #[inline]
                fn can_push<'a, It>(&self, items: It) -> bool
                where
                    It: Iterator<Item = &'a ($($name,)*)> + Clone,
                {
                    let can_push = true;
                    tuple_flatcontainer!(can_push can_push self items $($name)* @ 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31);
                    can_push
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
            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            impl<$($name),*> crate::Reserve for [<Tuple $($name)* Region>]<$($name),*>
            where
                $($name: crate::Reserve),*
            {
                type Reserve = ($($name::Reserve,)*);

                #[inline]
                fn reserve(&mut self, size: &Self::Reserve) {
                    let ($([<$name _size>],)*) = size;
                    $(self.[<container $name>].reserve([<$name _size>]);)*
                }
            }
        }
    );
    (can_push $var:ident $self:ident $items:ident $name0:ident $($name:ident)* @ $num0:tt $($num:tt)*) => {
        paste! {
            let $var = $var && $self.[<container $name0>].can_push($items.clone().map(|i| &i.$num0));
            tuple_flatcontainer!(can_push $var $self $items $($name)* @ $($num)*);
        }
    };
    (can_push $var:ident $self:ident $items:ident @ $($num:tt)*) => {};
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
    use crate::impls::tuple::TupleABCRegion;
    use crate::{FlatStack, MirrorRegion, Push, Region, StringRegion};

    #[test]
    fn test_tuple() {
        let t = (1, 2, 3);
        let mut r = <TupleABCRegion<MirrorRegion<_>, MirrorRegion<_>, MirrorRegion<_>>>::default();
        let index = r.push(t);
        assert_eq!(t, r.index(index));

        let index = r.push((&1, &2, &3));
        assert_eq!(t, r.index(index));

        let index = r.push((&1, 2, 3));
        assert_eq!(t, r.index(index));

        let index = r.push(&(1, 2, 3));
        assert_eq!(t, r.index(index));

        let index = r.push(&(1, &2, 3));
        assert_eq!(t, r.index(index));
    }

    #[test]
    fn test_nested() {
        let t = ("abc", 2, 3);
        let mut r = <TupleABCRegion<StringRegion, MirrorRegion<_>, MirrorRegion<_>>>::default();
        let index = r.push(t);
        assert_eq!(t, r.index(index));

        let index = r.push((&"abc", &2, &3));
        assert_eq!(t, r.index(index));

        let index = r.push((&"abc", 2, 3));
        assert_eq!(t, r.index(index));

        let index = r.push(&("abc", 2, 3));
        assert_eq!(t, r.index(index));
    }

    #[test]
    fn test_heap_size() {
        let t = ("abc", 2, 3);
        let mut r = <TupleABCRegion<StringRegion, MirrorRegion<_>, MirrorRegion<_>>>::default();

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
        let mut c = FlatStack::default_impl::<(usize, String, Vec<String>)>();
        c.copy((1, format!("Hello"), &["abc"]));

        let mut c2 = FlatStack::default_impl::<(usize, String, Vec<String>)>();
        c2.reserve_items(c.iter());
    }
}
