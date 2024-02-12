//! Regions that stores tuples.

use paste::paste;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Containerized, CopyOnto, Region, ReserveItems};

/// The macro creates the region implementation for tuples
macro_rules! tuple_flatcontainer {
    ($($name:ident)+) => (
        paste! {
            impl<$($name: Containerized),*> Containerized for ($($name,)*) {
                type Region = [<Tuple $($name)* Region >]<$($name::Region,)*>;
            }

            /// A region for a tuple.
            #[allow(non_snake_case)]
            #[derive(Default, Clone, Debug)]
            #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
            pub struct [<Tuple $($name)* Region >]<$($name),*> {
                $([<container $name>]: $name),*
            }

            #[allow(non_snake_case)]
            impl<$($name: Region),*> Region for [<Tuple $($name)* Region>]<$($name),*>
            where
               $(<$name as Region>::Index: crate::Index),*
            {
                type ReadItem<'a> = ($($name::ReadItem<'a>,)*) where Self: 'a;

                type Index = ($($name::Index,)*);

                fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
                where
                    Self: 'a {
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
                    It: Iterator<Item = &'a Self> + Clone {
                    $(self.[<container $name>].reserve_regions(regions.clone().map(|r| &r.[<container $name>]));)*
                }

                #[inline(always)]
                fn clear(&mut self) {
                    $(self.[<container $name>].clear();)*
                }
            }

            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            impl<$($name, [<$name _C>]: Region ),*>
                CopyOnto<[<Tuple $($name)* Region>]<$([<$name _C>]),*>>
                for ($($name,)*)
                where
                    $($name: CopyOnto<[<$name _C>]>),*
            {
                fn copy_onto(self, target: &mut [<Tuple $($name)* Region>]<$([<$name _C>]),*>)
                    -> <[<Tuple $($name)* Region>]<$([<$name _C>]),*> as Region>::Index {
                    let ($($name,)*) = self;
                    ($($name.copy_onto(&mut target.[<container $name>]),)*)
                }
            }

            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            impl<'a, $($name, [<$name _C>]: Region ),*>
                CopyOnto<[<Tuple $($name)* Region>]<$([<$name _C>]),*>>
                for &'a ($($name,)*)
                where
                    $(&'a $name: CopyOnto<[<$name _C>]>),*
            {
                #[inline(always)]
                fn copy_onto(self, target: &mut [<Tuple $($name)* Region>]<$([<$name _C>]),*>)
                    -> <[<Tuple $($name)* Region>]<$([<$name _C>]),*> as Region>::Index {
                    let ($($name,)*) = self;
                    ($($name.copy_onto(&mut target.[<container $name>]),)*)
                }
            }

            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            impl<'a, $($name, [<$name _C>]: Region ),*>
                ReserveItems<[<Tuple $($name)* Region>]<$([<$name _C>]),*>>
                for &'a ($($name,)*)
                where
                    $(&'a $name: ReserveItems<[<$name _C>]>),*
            {
                fn reserve_items<It>(target: &mut [<Tuple $($name)* Region>]<$([<$name _C>]),*>, items: It)
                where
                    It: Iterator<Item = Self> + Clone {
                        tuple_flatcontainer!(reserve_items target items $($name)* @ 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31);
                }
            }
        }
    );
    (reserve_items $target:ident $items:ident $name0:ident $($name:ident)* @ $num0:tt $($num:tt)*) => {
        paste! {
            ReserveItems::reserve_items(&mut $target.[<container $name0>], $items.clone().map(|i| {
                &i.$num0
            }));
            tuple_flatcontainer!(reserve_items $target $items $($name)* @ $($num)*);
        }
    };
    (reserve_items $target:ident $items:ident @ $($num:tt)*) => {};
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
