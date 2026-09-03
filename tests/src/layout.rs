use archive::{
    archive::{Archive, Archived, Deserialize},
    endian::Endian,
    pointer::Pointer,
    serialize::{Serialize, VariableDataTracker},
    serializer::Serializer,
};

#[derive(Debug, PartialEq)]
pub struct Layout {
    v1: u8,
    v2: u16,
    v3: u32,
    v4: u64,
    v5: u128,
}

#[repr(C)]
#[derive(Debug)]
pub struct ArchivedLayout<P, E>
where
    P: Pointer,
    E: Endian,
{
    v1: <u8 as Archive>::ArchiveType<P, E>,
    v2: <u16 as Archive>::ArchiveType<P, E>,
    v3: <u32 as Archive>::ArchiveType<P, E>,
    v4: <u64 as Archive>::ArchiveType<P, E>,
    v5: <u128 as Archive>::ArchiveType<P, E>,
}

impl<P, E> Archived for ArchivedLayout<P, E>
where
    P: Pointer,
    E: Endian,
{
    type DeserializedType<'a> = Layout;
}

impl<P, E> Deserialize for ArchivedLayout<P, E>
where
    P: Pointer,
    E: Endian,
{
    fn deserialize(&self) -> Self::DeserializedType<'_> {
        Layout {
            v1: self.v1.deserialize(),
            v2: self.v2.deserialize(),
            v3: self.v3.deserialize(),
            v4: self.v4.deserialize(),
            v5: self.v5.deserialize(),
        }
    }
}

impl Archive for Layout {
    type ArchiveType<P, E>
        = ArchivedLayout<P, E>
    where
        P: Pointer,
        E: Endian;
}

impl Serialize for Layout {
    fn serialize_fixed_data<S>(
        &self,
        writer: &mut S,
        tracker: &mut VariableDataTracker,
    ) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        self.v1.serialize_fixed_data(writer, tracker)?;
        self.v2.serialize_fixed_data(writer, tracker)?;
        self.v3.serialize_fixed_data(writer, tracker)?;
        self.v4.serialize_fixed_data(writer, tracker)?;
        self.v5.serialize_fixed_data(writer, tracker)?;

        Ok(())
    }

    fn track_variable_data<S>(&self, tracker: &mut VariableDataTracker)
    where
        S: Serializer,
    {
        self.v1.track_variable_data::<S>(tracker);
        self.v2.track_variable_data::<S>(tracker);
        self.v3.track_variable_data::<S>(tracker);
        self.v4.track_variable_data::<S>(tracker);
        self.v5.track_variable_data::<S>(tracker);
    }

    fn serialize_variable_data<S>(&self, writer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        self.v1.serialize_variable_data(writer)?;
        self.v2.serialize_variable_data(writer)?;
        self.v3.serialize_variable_data(writer)?;
        self.v4.serialize_variable_data(writer)?;
        self.v5.serialize_variable_data(writer)?;

        Ok(())
    }
}

pub fn layout() -> Layout {
    Layout {
        v1: 42,
        v2: 43,
        v3: 44,
        v4: 45,
        v5: 46,
    }
}
