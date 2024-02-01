#[macro_use]
mod impl_macros;
mod signable;
pub use signable::Signable;
mod zeroable;
pub use zeroable::Zeroable;

int_impl!(Self = i8, ActualT = i8, UnsignedT = u8);
int_impl!(Self = i16, ActualT = i16, UnsignedT = u16);
int_impl!(Self = i32, ActualT = i32, UnsignedT = u32);
int_impl!(Self = i64, ActualT = i64, UnsignedT = u64);
int_impl!(Self = i128, ActualT = i128, UnsignedT = u128);
