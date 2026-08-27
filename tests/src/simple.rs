use archive::archive::{Archive, Archived};
use archive::endian::Endian;
use archive::serialize::Serialize;
use archive::serializer::Serializer;

#[derive(PartialEq, Debug)]
pub struct Simple {
    value: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct ArchivedSimple<E>
where
    E: Endian,
{
    value: <u8 as Archive>::ArchiveType<E>,
}

impl<E> Archived for ArchivedSimple<E>
where
    E: Endian,
{
    type DeserializedType = Simple;

    fn deserialize(&self) -> Self::DeserializedType {
        Simple {
            value: self.value.deserialize(),
        }
    }
}

impl Archive for Simple {
    type ArchiveType<E>
        = ArchivedSimple<E>
    where
        E: Endian;
}

impl Serialize for Simple {
    fn serialize<S>(&self, writer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        self.value.serialize(writer)
    }
}

pub const fn simple() -> Simple {
    Simple { value: 42 }
}
