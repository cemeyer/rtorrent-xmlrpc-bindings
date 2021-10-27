//! Rtorrent multicall operation helpers

pub mod d;
pub mod f;
pub mod p;
pub mod t;

// Shared definition of distinct types for ops in seperate multicalls.
macro_rules! op_type {
    ( $(#[$meta:meta])* $name: ident ) => {
        $(#[$meta])*
        pub struct $name<T> {
            pub(crate) name: Cow<'static, str>,
            phantom: PhantomData<T>,
        }
    };
}
use op_type;

// Semi-shared definition of const definitions for multicalls.
macro_rules! op_const {
    ( $(#[$meta:meta])* $type: ident, $name: ident, $res: ty, $api_prefix: literal, $api: literal ) => {
        $(#[$meta])*
        pub const $name: $type<$res> = $type {
            name: Cow::Borrowed(concat!($api_prefix, $api)),
            phantom: PhantomData,
        };
    };
}
use op_const;

macro_rules! define_builder {
    // The pipe is an ugly kludge to allow us to list types left-to-right but avoid Rust macro
    // parsing ambiguity.
    ( $optype: ident, $prev: ident, $name: ident, $($phantoms:ident $ty:ident),* | $phantom_last:ident $ty_last:ident ) => {
        pub struct $name<$($ty: TryFromValue,)* $ty_last: TryFromValue> {
            inner: raw::$name<$($ty,)* $ty_last>,
        }

        impl<$($ty: TryFromValue,)* $ty_last: TryFromValue> $name<$($ty,)* $ty_last> {
            pub fn invoke(&self) -> Result<Vec<($($ty,)* $ty_last,)>> {
                self.inner.invoke()
            }
        }

        impl<$($ty: TryFromValue,)*> $prev<$($ty,)*> {
            /// Add a column (an accessor for `getter`) to the query represented by this builder.
            ///
            /// `call()` can be invoked again, repeatedly, on the result of `call()` invocations,
            /// to build queries with more columns.
            ///
            /// (The higher-order builder types are invisible in Rustdoc because they are generated
            /// by macros.)
            pub fn call<T: TryFromValue>(self, getter: $optype<T>) -> $name<$($ty,)* T> {
                $name {
                    inner: self.inner.call::<T>(&getter.name)
                }
            }
        }
    }
}
pub(super) use define_builder;
