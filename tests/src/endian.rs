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
pub struct ArchivedEndian {
    value: <u32 as Archive>::ArchiveType,
}

impl Archived for ArchivedEndian {
    type DeserializedType = Endian;

    fn deserialize(&self) -> Self::DeserializedType {
        Endian {
            value: self.value.deserialize(),
        }
    }
}

impl Archive for Endian {
    type ArchiveType = ArchivedEndian;
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
