use archive::{
    archive::{Archive, Archived, Deserialize},
    endian::Endian,
    pointer::Pointer,
    serialize::Serialize,
    serializer::Serializer,
};

#[derive(PartialEq, Debug)]
pub struct Slice<'a> {
    pub v1: &'a [u32],
    pub v2: Vec<u16>,
}

#[derive(Debug)]
pub struct BorrowedSlice<'a> {
    v1: &'a [u32],
    v2: &'a [u16],
}

impl PartialEq<Slice<'_>> for BorrowedSlice<'_> {
    fn eq(&self, other: &Slice<'_>) -> bool {
        self.v1 == other.v1 && self.v2 == other.v2
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ArchivedSlice<P, E>
where
    P: Pointer,
    E: Endian,
{
    v1: archive::slice::ArchivedSlice<u32, P, E>,
    v2: archive::slice::ArchivedSlice<u16, P, E>,
}

impl<P, E> Archived for ArchivedSlice<P, E>
where
    P: Pointer,
    E: Endian,
{
    type DeserializedType<'a> = BorrowedSlice<'a>;
}

impl<P, E> Deserialize for ArchivedSlice<P, E>
where
    P: Pointer,
    E: Endian,
{
    fn deserialize(&self) -> Self::DeserializedType<'_> {
        BorrowedSlice {
            v1: self.v1.deserialize(),
            v2: self.v2.deserialize(),
        }
    }
}

impl Archive for Slice<'_> {
    type ArchiveType<P, E>
        = ArchivedSlice<P, E>
    where
        E: Endian,
        P: Pointer;
}

impl Serialize for Slice<'_> {
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
        self.v2.track_variable_data::<S>(tracker);
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

pub fn slice() -> Slice<'static> {
    Slice {
        v1: &[42, 43, 44],
        v2: vec![45, 46, 47],
    }
}
