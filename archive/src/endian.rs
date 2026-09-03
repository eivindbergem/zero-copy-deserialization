pub enum Endianness {
    Little,
    Big,
}

pub trait Endian: Copy + core::fmt::Debug {
    const ENDIANNESS: Endianness;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BigEndian {}

impl Endian for BigEndian {
    const ENDIANNESS: Endianness = Endianness::Big;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LittleEndian {}

impl Endian for LittleEndian {
    const ENDIANNESS: Endianness = Endianness::Little;
}

pub trait ToFromEndian {
    fn to_le(self) -> Self;
    fn from_le(value: Self) -> Self;
    fn to_be(self) -> Self;
    fn from_be(value: Self) -> Self;
}

macro_rules! impl_to_from_endian {
    ($ty:ty) => {
        impl ToFromEndian for $ty {
            fn to_le(self) -> Self {
                self.to_le()
            }

            fn from_le(value: Self) -> Self {
                Self::from_le(value)
            }

            fn to_be(self) -> Self {
                self.to_be()
            }

            fn from_be(value: Self) -> Self {
                Self::from_be(value)
            }
        }
    };
}

impl_to_from_endian!(u8);
impl_to_from_endian!(u16);
impl_to_from_endian!(u32);
impl_to_from_endian!(u64);
impl_to_from_endian!(u128);

impl_to_from_endian!(i8);
impl_to_from_endian!(i16);
impl_to_from_endian!(i32);
impl_to_from_endian!(i64);
impl_to_from_endian!(i128);
