use crate::{
    archive::{Archive, Archived, Deserialize},
    endian::Endian,
    pointer::{Pointer, RelPtr},
    primitive::ArchivedPrimitive,
    serialize::{Serialize, VariableDataTracker},
    serializer::Serializer,
};

#[repr(C)]
pub struct ArchivedSlice<T, P, E> {
    ptr: RelPtr<T, P, E>,
    len: ArchivedPrimitive<P, E>,
}

impl<T, P, E> ArchivedSlice<T, P, E>
where
    P: Pointer,
    E: Endian,
{
    pub fn as_slice(&self) -> &[T] {
        dbg!(self.ptr.as_ptr(), self.len.to_usize());
        unsafe { core::slice::from_raw_parts(self.ptr.as_ptr(), self.len.to_usize()) }
    }
}

impl<T, P, E> core::fmt::Debug for ArchivedSlice<T, P, E>
where
    T: core::fmt::Debug,
    P: Pointer,
    E: Endian,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<T, P, E> Archived for ArchivedSlice<T, P, E>
where
    T: 'static,
{
    type DeserializedType<'a> = &'a [T];
}

impl<T, P, E> Deserialize for ArchivedSlice<T, P, E>
where
    T: 'static,
    P: Pointer,
    E: Endian,
{
    fn deserialize(&self) -> Self::DeserializedType<'_> {
        self.as_slice()
    }
}

impl<T> Archive for [T]
where
    T: Archive + std::fmt::Debug + 'static,
{
    type ArchiveType<P, E>
        = ArchivedSlice<T, P, E>
    where
        P: Pointer,
        E: Endian;
}

impl<T> Serialize for [T]
where
    T: std::fmt::Debug + Archive + Serialize + 'static,
{
    fn serialize_fixed_data<S>(
        &self,
        writer: &mut S,
        tracker: &mut crate::serialize::VariableDataTracker,
    ) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        // We have to add padding to get right alignemnt for pointer
        // before we get the relative pointer value
        writer.write_padding::<S::Pointer>()?;

        let pointer =
            tracker.relative_pointer::<T::ArchiveType<S::Pointer, S::Endian>>(writer.position());

        // Write pointer and len
        writer.write_pointer(pointer)?;
        writer.write_pointer(self.len())?;

        self.track_variable_data::<S>(tracker);

        Ok(())
    }

    fn track_variable_data<S>(&self, tracker: &mut VariableDataTracker)
    where
        S: Serializer,
    {
        // Add variable size data to len
        tracker.add_data::<T::ArchiveType<S::Pointer, S::Endian>>(self.len());

        for item in self.iter() {
            item.track_variable_data::<S>(tracker);
        }
    }

    fn serialize_variable_data<S>(&self, writer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        let mut tracker = VariableDataTracker::new(writer.position());

        // tracker.add_data::<T::ArchiveType<S::Pointer, S::Endian>>(self.len());

        for item in self.iter() {
            item.serialize_fixed_data(writer, &mut tracker)?;
        }

        for item in self.iter() {
            item.serialize_variable_data(writer)?;
        }

        Ok(())
    }
}
