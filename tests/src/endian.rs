use archive::{
    archive::{Archive, Archived},
    serialize::Serialize,
    serializer::Serializer,
};

#[derive(PartialEq, Debug)]
pub struct Endian {
    value: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct ArchivedEndian<E>
where
    E: archive::endian::Endian,
{
    value: <u32 as Archive>::ArchiveType<E>,
}

impl<E> Archived for ArchivedEndian<E>
where
    E: archive::endian::Endian,
{
    type DeserializedType = Endian;

    fn deserialize(&self) -> Self::DeserializedType {
        Endian {
            value: self.value.deserialize(),
        }
    }
}

impl Archive for Endian {
    type ArchiveType<E>
        = ArchivedEndian<E>
    where
        E: archive::endian::Endian;
}

impl Serialize for Endian {
    fn serialize<S>(&self, writer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        self.value.serialize(writer)
    }
}

pub const fn endian() -> Endian {
    Endian { value: 0x1234567 }
}
