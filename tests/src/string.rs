use archive::{
    archive::{Archive, Archived},
    endian::Endian,
    pointer::Pointer,
    serialize::{Serialize, VariableDataTracker},
    serializer::Serializer,
};

#[derive(Debug, PartialEq)]
pub struct String {
    s: std::string::String,
}

#[repr(C)]
#[derive(Debug)]
pub struct ArchivedString<P, E>
where
    P: Pointer,
    E: Endian,
{
    s: archive::string::ArchivedString<P, E>,
}

impl<P, E> Archived for ArchivedString<P, E>
where
    P: Pointer,
    E: Endian,
{
    type DeserializedType = String;

    fn deserialize(&self) -> Self::DeserializedType {
        String {
            s: self.s.deserialize(),
        }
    }
}

impl Archive for String {
    type ArchiveType<P, E>
        = ArchivedString<P, E>
    where
        E: Endian,
        P: Pointer;
}

impl Serialize for String {
    fn serialize_fixed_data<S>(
        &self,
        writer: &mut S,
        tracker: &mut VariableDataTracker,
    ) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        self.s.serialize_fixed_data(writer, tracker)
    }

    fn track_variable_data<S>(&self, tracker: &mut VariableDataTracker)
    where
        S: Serializer,
    {
        self.s.track_variable_data::<S>(tracker);
    }

    fn serialize_variable_data<S>(&self, writer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        self.s.serialize_variable_data(writer)
    }
}

pub fn string() -> String {
    String {
        s: "All your base are belong to us".into(),
    }
}
