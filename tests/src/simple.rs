use archive::archive::{Archive, Archived, Deserialize};
use archive::endian::Endian;
use archive::pointer::Pointer;
use archive::serialize::{Serialize, VariableDataTracker};
use archive::serializer::Serializer;

#[derive(PartialEq, Debug)]
pub struct Simple {
    value: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct ArchivedSimple<P, E>
where
    P: Pointer,
    E: Endian,
{
    value: <u8 as Archive>::ArchiveType<P, E>,
}

impl<P, E> Archived for ArchivedSimple<P, E>
where
    P: Pointer,
    E: Endian,
{
    type DeserializedType<'a> = Simple;
}

impl<P, E> Deserialize for ArchivedSimple<P, E>
where
    P: Pointer,
    E: Endian,
{
    fn deserialize(&self) -> Self::DeserializedType<'_> {
        Simple {
            value: self.value.deserialize(),
        }
    }
}

impl Archive for Simple {
    type ArchiveType<P, E>
        = ArchivedSimple<P, E>
    where
        P: Pointer,
        E: Endian;
}

impl Serialize for Simple {
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

pub const fn simple() -> Simple {
    Simple { value: 42 }
}
