use archive::{
    archive::{Archive, Archived, Deserialize},
    pointer::Pointer,
    serialize::{Serialize, VariableDataTracker},
    serializer::Serializer,
};

#[derive(PartialEq, Debug)]
pub struct Endian {
    value: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct ArchivedEndian<P, E>
where
    P: Pointer,
    E: archive::endian::Endian,
{
    value: <u32 as Archive>::ArchiveType<P, E>,
}

impl<P, E> Archived for ArchivedEndian<P, E>
where
    P: Pointer,
    E: archive::endian::Endian,
{
    type DeserializedType<'a> = Endian;
}

impl<P, E> Deserialize for ArchivedEndian<P, E>
where
    P: Pointer,
    E: archive::endian::Endian,
{
    fn deserialize(&self) -> Self::DeserializedType<'_> {
        Endian {
            value: self.value.deserialize(),
        }
    }
}

impl Archive for Endian {
    type ArchiveType<P, E>
        = ArchivedEndian<P, E>
    where
        P: Pointer,
        E: archive::endian::Endian;
}

impl Serialize for Endian {
    fn serialize_fixed_data<S>(
        &self,
        writer: &mut S,
        tracker: &mut VariableDataTracker,
    ) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        self.value.serialize_fixed_data(writer, tracker)
    }

    fn track_variable_data<S>(&self, tracker: &mut VariableDataTracker)
    where
        S: Serializer,
    {
        self.value.track_variable_data::<S>(tracker);
    }

    fn serialize_variable_data<S>(&self, writer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        self.value.serialize_variable_data(writer)
    }
}

pub const fn endian() -> Endian {
    Endian { value: 0x1234567 }
}
