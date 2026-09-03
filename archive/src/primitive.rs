use core::marker::PhantomData;

use crate::{
    archive::{Archive, Archived, Deserialize},
    endian::{Endian, Endianness, ToFromEndian},
    pointer::Pointer,
    serialize::{Serialize, VariableDataTracker},
    serializer::Serializer,
};

pub trait Primitive {}

impl Primitive for u8 {}
impl Primitive for u16 {}
impl Primitive for u32 {}
impl Primitive for u64 {}
impl Primitive for u128 {}

impl Primitive for i8 {}
impl Primitive for i16 {}
impl Primitive for i32 {}
impl Primitive for i64 {}
impl Primitive for i128 {}

#[repr(C)]
pub struct ArchivedPrimitive<T, E> {
    inner: T,
    __: PhantomData<E>,
}

impl<T, E> ArchivedPrimitive<T, E> {
    pub fn as_bytes(&self) -> &[u8] {
        let len = size_of::<T>();
        let ptr = &self.inner as *const _ as *const u8;
        unsafe { core::slice::from_raw_parts(ptr, len) }
    }
}

impl<T, E> ArchivedPrimitive<T, E>
where
    T: ToFromEndian + Copy,
    E: Endian,
{
    pub fn to_inner(&self) -> T {
        match E::ENDIANNESS {
            Endianness::Little => T::from_le(self.inner),
            Endianness::Big => T::from_be(self.inner),
        }
    }
}

impl<T, E> ArchivedPrimitive<T, E>
where
    T: ToFromEndian + Copy + TryInto<usize>,
    E: Endian,
{
    pub fn to_usize(&self) -> usize {
        self.to_inner()
            .try_into()
            .unwrap_or_else(|_| unreachable!())
    }
}

impl<T, E> ArchivedPrimitive<T, E>
where
    T: ToFromEndian + Copy + TryFrom<usize>,
    E: Endian,
{
    pub fn from_usize(value: usize) -> Self {
        value.try_into().unwrap_or_else(|_| unreachable!())
    }
}

impl<T, E> From<T> for ArchivedPrimitive<T, E>
where
    T: ToFromEndian + Copy,
    E: Endian,
{
    fn from(value: T) -> Self {
        let value = match E::ENDIANNESS {
            Endianness::Little => value.to_le(),
            Endianness::Big => value.to_be(),
        };
        Self {
            inner: value,
            __: PhantomData,
        }
    }
}

impl<T, E> core::fmt::Debug for ArchivedPrimitive<T, E>
where
    T: ToFromEndian + Copy + core::fmt::Debug,
    E: Endian,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.to_inner().fmt(f)
    }
}

impl<T, E> Archived for ArchivedPrimitive<T, E> {
    type DeserializedType<'a> = T;
}

impl<T, E> Deserialize for ArchivedPrimitive<T, E>
where
    T: ToFromEndian + Copy,
    E: Endian,
{
    fn deserialize(&self) -> Self::DeserializedType<'_> {
        self.to_inner()
    }
}

impl<T, E> TryFrom<usize> for ArchivedPrimitive<T, E>
where
    T: TryFrom<usize> + ToFromEndian + Copy,
    E: Endian,
{
    type Error = <T as TryFrom<usize>>::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let value: T = value.try_into()?;
        Ok(ArchivedPrimitive::from(value))
    }
}

impl<T, E> TryInto<usize> for ArchivedPrimitive<T, E>
where
    T: TryInto<usize> + ToFromEndian + Copy,
    E: Endian,
{
    type Error = <T as TryInto<usize>>::Error;

    fn try_into(self) -> Result<usize, Self::Error> {
        Ok(self.to_inner().try_into()?)
    }
}

impl<T> Archive for T
where
    T: Primitive + Copy + ToFromEndian + PartialEq + core::fmt::Debug,
{
    type ArchiveType<P, E>
        = ArchivedPrimitive<T, E>
    where
        P: Pointer,
        E: Endian;
}

impl<T> Serialize for T
where
    T: Primitive + Archive + ToFromEndian + Copy,
{
    fn serialize_fixed_data<S>(
        &self,
        writer: &mut S,
        _tracker: &mut VariableDataTracker,
    ) -> Result<(), S::Error>
    where
        S: Serializer,
        T: Into<ArchivedPrimitive<T, S::Endian>>,
    {
        let value = ArchivedPrimitive::<T, S::Endian>::from(*self);
        writer.write_primitive(value)
    }

    fn track_variable_data<S>(&self, _tracker: &mut VariableDataTracker)
    where
        S: Serializer,
    {
    }

    fn serialize_variable_data<S>(&self, _writer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        Ok(())
    }
}
