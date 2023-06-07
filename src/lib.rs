/// Implement conversion from `T` to its newtype wrapper.
///
/// Also works on types that convert to `T` via `into()`.
///
/// The wrapper can either be a newtype struct or a newtype variant of an enum:
///   - Struct
///     - Given `struct Foo(T);` and `impl From<U> for T`
///     - `newtype_wrap!(Foo, T);` implements `T -> Foo(T)`
///     - `newtype_wrap!(Foo, U);` implements `U -> Foo(T)`
///   - Enum
///     - Given `enum Foo{ Bar(T) };` and `impl From<U> for T`
///     - `newtype_wrap!(Foo, Bar, T);` implements `T -> Foo::Bar(T)`
///     - `newtype_wrap!(Foo, Bar, U);` implements `U -> Foo::Bar(T)`
#[macro_export]
macro_rules! newtype_wrap {
    ($wrapper: ident, $wrapped: ty) => {
        impl From<$wrapped> for $wrapper {
            fn from(val: $wrapped) -> Self {
                $wrapper(val.into())
            }
        }
    };
    ($wrapper: ty, $variant: ident, $wrapped: ty) => {
        impl From<$wrapped> for $wrapper {
            fn from(val: $wrapped) -> Self {
                <$wrapper>::$variant(val.into())
            }
        }
    };
}

/// The same as [`newtype_wrap`], except that the wrapping is implemented on
/// all types `U` that convert to `T` via `into()`.
///
/// Be careful when using this: there may be `U` types that you wish do not receive automatic wrapping.
/// In such a case, you should use [`newtype_wrap`] on each `U` type separately.
#[macro_export]
macro_rules! newtype_wrap_from_any {
    ($wrapper: ident, $wrapped: ty) => {
        impl<T> From<T> for $wrapper
        where
            T: Into<$wrapped>,
        {
            fn from(val: T) -> Self {
                $wrapper(val.into())
            }
        }
    };
    ($wrapper: ty, $variant: ident, $wrapped: ty) => {
        impl<T> From<T> for $wrapper
        where
            T: Into<$wrapped>,
        {
            fn from(val: T) -> Self {
                <$wrapper>::$variant(val.into())
            }
        }
    };
}

/// Implement conversion from `A` to `Z` via `B`, `C`, ...
///
/// `B` needs to implement `From<A>`, `C` needs to implement `From<B>`, and so on.
///
/// Example: `chained_into!(A, B, C, D, E);` implements `From<A> for E` by
/// converting `A` to `B`, then `B` to `C`, then `C` to `D`, and finally `D` to `E`.
#[macro_export]
macro_rules! chained_into {
    ($source: ty, $($tail: ty),+) => {
        chained_into!(@impl $source | $($tail),+ | chained_into!(@get_target $($tail),+));
    };
    (@impl $source: ty | $($intermediate: ty),* | $target: ty) => {
        impl From<$source> for $target {
            fn from(val: $source) -> $target {
                chained_into!(@next val, $($intermediate),*)
            }
        }
    };
    (@get_target $target: ty) => {
        $target
    };
    (@get_target $_: ty, $($tail: ty),+) => {
        chained_into!(@get_target $($tail),+)
    };
    (@next $val: expr, $intermediate: ty) => {
        <$intermediate>::from($val)
    };
    (@next $val: expr, $intermediate: ty, $($tail: ty),+) => {
        chained_into!(@next <$intermediate>::from($val), $($tail),+)
    };
}
