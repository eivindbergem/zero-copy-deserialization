use archive::{
    archive::{Archive, Archived},
    endian::Endian,
    serialize::Serialize,
    serializer::Serializer,
};

#[derive(Debug, PartialEq)]
pub struct Layout {
    v1: u8,
    v2: u16,
    v3: u64,
    v4: u128,
}

#[repr(C)]
#[derive(Debug)]
pub struct ArchivedLayout<E>
where
    E: Endian,
{
    v1: <u8 as Archive>::ArchiveType<E>,
    v2: <u16 as Archive>::ArchiveType<E>,
    v4: <u64 as Archive>::ArchiveType<E>,
    v5: <u128 as Archive>::ArchiveType<E>,
}

impl<E> Archived for ArchivedLayout<E>
where
    E: Endian,
{
    type DeserializedType = Layout;

    fn deserialize(&self) -> Self::DeserializedType {
        Layout {
            v1: self.v1.deserialize(),
            v2: self.v2.deserialize(),
            v3: self.v4.deserialize(),
            v4: self.v5.deserialize(),
        }
    }
}
impl Archive for Layout {
    type ArchiveType<E>
        = ArchivedLayout<E>
    where
        E: Endian;
}

impl Serialize for Layout {
    fn serialize<S>(&self, writer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        self.v1.serialize(writer)?;
        self.v2.serialize(writer)?;
        self.v3.serialize(writer)?;
        self.v4.serialize(writer)?;

        Ok(())
    }
}

pub fn layout() -> Layout {
    Layout {
        v1: 42,
        v2: 43,
        v3: 44,
        v4: 45,
    }
}
