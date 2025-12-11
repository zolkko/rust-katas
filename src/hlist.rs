// marker trait, because there is no true HRTs in Rust.
// Must be private/sealed to prevent third party to implement it,
// because it must be implemented only for HList and HNil.

pub use crate::hlist::macros::hlist;


pub trait IsHList: crate::hlist::sealed::SealedHList {}


pub trait HListT<A, B>
where
    B: IsHList,
{
    fn head(&self) -> &A;
    fn tail(&self) -> &B;
}

/// An empty HList
#[derive(Debug, Clone, Default)]
pub struct HNil;

impl IsHList for HNil {}

impl crate::hlist::sealed::SealedHList for HNil {}


impl HListT<HNil, HNil> for HNil {
    fn head(&self) -> &HNil {
        &self
    }
    fn tail(&self) -> &HNil {
        &self
    }
}

/// Non-empty HList
#[derive(Debug, Clone)]
pub struct HList<A, B> {
    pub head: A,
    pub tail: B,
}

impl<A, B> crate::hlist::sealed::SealedHList for HList<A, B>
where
    B: IsHList,
{
}

impl<A, B> IsHList for HList<A, B>
where
    B: IsHList,
{
}

impl<A, B> HListT<A, B> for HList<A, B>
where
    B: IsHList,
{
    fn head(&self) -> &A {
        &self.head
    }
    fn tail(&self) -> &B {
        &self.tail
    }
}


pub(crate) mod sealed {
    pub trait SealedHList {}
}


pub(crate) mod macros {

    #[macro_export]
    macro_rules! hlist {
        () => { $crate::hlist::HNil };
        ($head:expr, $($tail:tt)*) => {
            $crate::hlist::HList {
                head: $head,
                tail: $crate::hlist::hlist!($($tail)*)
            }
        };
    }

    macro_rules! tuple_hlist_type {
        () => { $crate::hlist::HNil };
        ($head:ident $(, $tail:ident)*) => { $crate::hlist::HList<$head, $crate::hlist::macros::tuple_hlist_type!($($tail),*)> };
    }

    macro_rules! tuple_hlist_expr {
        ( $param:ident ) => { 
            $crate::hlist::HList { head: $param, tail: $crate::hlist::HNil }
        };
        ( $head:ident, $( $tail:ident ),* ) => {
            $crate::hlist::HList { head: $head, tail: $crate::hlist::macros::tuple_hlist_expr!($( $tail ),*) }
        };
    }

    macro_rules! impl_from_tuple {
        ( $( $name:ident ),+ ) => {
            impl< $( $name ),+ > From<( $( $name, )+ )> for $crate::hlist::macros::tuple_hlist_type!( $( $name ),+ )
            where
                // Ensure the components satisfy the recursion requirements if needed.
                // By construction, the tail will be an HList or HNil, satisfying the B: IsHList bound.
                $crate::hlist::macros::tuple_hlist_type!( $( $name ),+ ): $crate::hlist::IsHList
            {
                #[allow(non_snake_case)]
                fn from(tuple: ( $( $name, )+ )) -> Self {
                    let ( $( $name, )+ ) = tuple;
                    $crate::hlist::macros::tuple_hlist_expr!( $( $name ),+ )
                }
            }
        }
    }

    macro_rules! impl_from_tuples_recursive {
        () => {};
        ($head:ident $(, $tail:ident)*) => {
            $crate::hlist::macros::impl_from_tuple!($head $(, $tail)*);
            $crate::hlist::macros::impl_from_tuples_recursive!($($tail),*);
        };
    }


    pub(crate) use {impl_from_tuples_recursive, impl_from_tuple, tuple_hlist_expr, tuple_hlist_type};
    pub use hlist;
}

crate::hlist::macros::impl_from_tuples_recursive!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);


/*
impl<T1> std::ops::Deref for (T1,) {
    type Target = HList<T1, HNil>;

    // Required method
    fn deref(&self) -> &Self::Target {
        todo!()
    }
}
*/
