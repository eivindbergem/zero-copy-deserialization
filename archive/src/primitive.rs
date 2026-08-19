use crate::{
    archive::{Archive, Archived},
    serialize::Serialize,
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
pub struct ArchivedPrimitive<T> {
    inner: T,
}

impl<T> ArchivedPrimitive<T> {
    pub fn as_bytes(&self) -> &[u8] {
        let len = size_of::<T>();
        let ptr = &self.inner as *const _ as *const u8;
        unsafe { core::slice::from_raw_parts(ptr, len) }
    }
}

impl<T> ArchivedPrimitive<T>
where
    T: Copy,
{
    pub fn to_inner(&self) -> T {
        self.inner
    }
}

impl<T> From<T> for ArchivedPrimitive<T>
where
    T: Copy,
{
    fn from(value: T) -> Self {
        Self { inner: value }
    }
}

impl<T> core::fmt::Debug for ArchivedPrimitive<T>
where
    T: Copy + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.to_inner().fmt(f)
    }
}

impl<T> Archived for ArchivedPrimitive<T>
where
    T: Copy,
{
    type DeserializedType = T;

    fn deserialize(&self) -> Self::DeserializedType {
        self.to_inner()
    }
}

impl<T> Archive for T
where
    T: Primitive + Copy + core::fmt::Debug,
{
    type ArchiveType = ArchivedPrimitive<T>;
}

impl<T> Serialize for T
where
    T: Archive + Copy,
{
    fn serialize<S>(&self, writer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
        T: Into<ArchivedPrimitive<T>>,
    {
        writer.write_primitive(ArchivedPrimitive::from(*self))
    }
}
