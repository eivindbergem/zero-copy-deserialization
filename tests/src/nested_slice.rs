use archive::{
    archive::{Archive, Archived},
    endian::Endian,
    pointer::Pointer,
    primitive::ArchivedPrimitive,
    serialize::Serialize,
    serializer::Serializer,
    slice::ArchivedSlice,
};

#[derive(PartialEq, Debug)]
pub struct Inner {
    v1: Vec<u32>,
    v2: Vec<u16>,
}

#[repr(C)]
#[derive(Debug)]
pub struct ArchivedInner<P, E>
where
    P: Pointer,
    E: Endian,
{
    v1: ArchivedSlice<ArchivedPrimitive<u32, E>, P, E>,
    v2: ArchivedSlice<ArchivedPrimitive<u16, E>, P, E>,
}

impl<P, E> Archived for ArchivedInner<P, E>
where
    P: Pointer,
    E: Endian,
{
    type DeserializedType = Inner;

    fn deserialize(&self) -> Self::DeserializedType {
        Inner {
            v1: self.v1.deserialize().into(),
            v2: self.v2.deserialize().into(),
        }
    }
}

impl Archive for Inner {
    type ArchiveType<P, E>
        = ArchivedInner<P, E>
    where
        E: Endian,
        P: Pointer;
}

impl Serialize for Inner {
    fn serialize_fixed_data<S>(
        &self,
        writer: &mut S,
        tracker: &mut archive::serialize::VariableDataTracker,
    ) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        self.v1.serialize_fixed_data(writer, tracker)?;
        self.v2.serialize_fixed_data(writer, tracker)?;

        Ok(())
    }

    fn track_variable_data<S>(&self, tracker: &mut archive::serialize::VariableDataTracker)
    where
        S: Serializer,
    {
        self.v1.track_variable_data::<S>(tracker);
        self.v1.track_variable_data::<S>(tracker);
    }

    fn serialize_variable_data<S>(&self, writer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        self.v1.serialize_variable_data(writer)?;
        self.v2.serialize_variable_data(writer)?;
        Ok(())
    }
}

#[derive(PartialEq, Debug)]
pub struct NestedSlice {
    value: Vec<Inner>,
}

#[repr(C)]
#[derive(Debug)]
pub struct ArchivedNestedSlice<P, E>
where
    P: Pointer,
    E: Endian,
{
    value: ArchivedSlice<ArchivedInner<P, E>, P, E>,
}

impl<P, E> Archived for ArchivedNestedSlice<P, E>
where
    P: Pointer,
    E: Endian,
{
    type DeserializedType = NestedSlice;

    fn deserialize(&self) -> Self::DeserializedType {
        NestedSlice {
            value: self
                .value
                .as_slice()
                .iter()
                .map(|item| Inner {
                    v1: item.v1.deserialize().into(),
                    v2: item.v2.deserialize().into(),
                })
                .collect(),
        }
    }
}

impl Archive for NestedSlice {
    type ArchiveType<P, E>
        = ArchivedNestedSlice<P, E>
    where
        E: Endian,
        P: Pointer;
}

impl Serialize for NestedSlice {
    fn serialize_fixed_data<S>(
        &self,
        writer: &mut S,
        tracker: &mut archive::serialize::VariableDataTracker,
    ) -> Result<(), S::Error>
    where
        S: archive::serializer::Serializer,
    {
        self.value.serialize_fixed_data(writer, tracker)
    }

    fn track_variable_data<S>(&self, tracker: &mut archive::serialize::VariableDataTracker)
    where
        S: archive::serializer::Serializer,
    {
        self.value.track_variable_data::<S>(tracker);
    }

    fn serialize_variable_data<S>(&self, writer: &mut S) -> Result<(), S::Error>
    where
        S: archive::serializer::Serializer,
    {
        self.value.serialize_variable_data(writer)
    }
}

pub fn nested_slice() -> NestedSlice {
    NestedSlice {
        value: vec![
            Inner {
                v1: vec![42, 43, 44],
                v2: vec![45, 46, 47],
            },
            Inner {
                v1: vec![42, 43, 44],
                v2: vec![45, 46, 47],
            },
        ],
    }
}
