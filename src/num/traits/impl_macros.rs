use super::{Signable, Zeroable};
macro_rules! int_impl {
    (
        Self = $SelfT:ty,
        ActualT = $ActualT:ident,
        UnsignedT = $UnsignedT:ty
    ) => {
        impl Zeroable for $SelfT {
            const ZERO: Self = !(<$UnsignedT>::MAX) as Self;
        }
        impl Signable for $SelfT {
            const SIGNED: bool = true;
        }
    };
}
macro_rules! uint_impl {
    (
        Self = $SelfT:ty,
        ActualT = $ActualT:ident,
        SignedT = $SignedT:ty
    ) => {
        impl Zeroable for $SelfT {
            const ZERO: Self = $ActualT::MIN as Self;
        }
        impl Signable for $SelfT {
            const SIGNED: bool = false;
        }
    };
}
macro_rules! float_impl {
    (
        Self = $SelfT:ty,
        ActualT = $ActualT:ident,
        UnsignedT = $UnsignedT:ident
    ) => {
        impl Zeroable for $SelfT {
            const ZERO: Self = <$UnsignedT as ::corex::num::zero::Zeroable>::ZERO as Self;
        }
        impl ::corex::num::sign::Signable for $SelfT {
            const SIGNED: bool = true;
        }
    };
}
macro_rules! zero_int_impl {
    (
        Self = $SelfT:ty,
        ActualT = $ActualT:ident,
        UnsignedT = $UnsignedT:ty,

        BITS = $BITS:literal,
        Zero = $Zero:literal,
    ) => {
        impl Zeroable for $SelfT {
            /// The zero value represented by this type
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```rust
            #[doc = concat!("assert_eq!(", stringify!($SelfT), "::ZERO, ", stringify!($Zero), ");")]
            /// ```
            const ZERO: Self = !(<$UnsignedT>::MAX) as Self;
        }
    };
}
macro_rules! zero_uint_impl {
    (
        Self = $SelfT:ty,
        ActualT = $ActualT:ident,
        SignedT = $SignedT:ty,

        BITS = $BITS:literal,
        Zero = $Zero:literal,
    ) => {
        impl Zeroable for $SeltT {
            /// The zero value represented by this type
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```rust
            #[doc = concat!("assert_eq!(", stringify!($SelfT), "::ZERO, ", stringify!($Zero), ");")]
            /// ```
            const ZERO: Self = !(<$UnsignedT>::MAX) as Self;
        }
    };
}
