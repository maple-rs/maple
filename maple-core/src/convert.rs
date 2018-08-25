use std::marker::Unsize;
use std::borrow::Cow;

pub trait MyFrom<T>: Sized {
    fn my_from(_: T) -> Self;
}

pub trait MyInto<T>: Sized {
    fn my_into(self) -> T;
}

impl<T, U: MyFrom<T>> MyInto<U> for T 
{
    fn my_into(self) -> U {
        U::my_from(self)
    }
}

impl<T: Unsize<U>, U: ?Sized> MyFrom<T> for Box<U> {
    fn my_from(val: T) -> Box<U> {
        Box::new(val) as Box<U>
    }
}

impl<T, X: MyInto<T>> MyFrom<X> for Option<T> {
    fn my_from(t: X) -> Option<T> {
        let tmp: T = t.my_into();
        tmp.into()
    }
}

impl<'a> MyFrom<&'a str> for Cow<'a, str> {
    fn my_from(t: &'a str) -> Cow<'a, str> {
        t.into()
    }
}

impl<'a> MyFrom<String> for Cow<'a, str> {
    fn my_from(t: String) -> Cow<'a, str> {
        t.into()
    }
}

impl<'a> MyFrom<&'a str> for &'a str {
    fn my_from(t: &'a str) -> &'a str {
        t
    }
}

macro_rules! simple_impl {
    ($from: ty) => {
        impl MyFrom<$from> for $from {
            fn my_from(t: $from) -> $from {
                t
            }
        }
    }
}

simple_impl!(i64);
simple_impl!(u64);
simple_impl!(u32);
simple_impl!(i32);
simple_impl!(u16);
simple_impl!(i16);
simple_impl!(u8);
simple_impl!(i8);
simple_impl!(bool);
simple_impl!(String);