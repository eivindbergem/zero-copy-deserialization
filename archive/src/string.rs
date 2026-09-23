use std::ffi::{CStr, c_char};

use crate::{
    archive::{Archive, Archived},
    endian::Endian,
    pointer::Pointer,
    primitive::ArchivedPrimitive,
    serialize::{Serialize, VariableDataTracker},
    serializer::Serializer,
    slice::ArchivedSlice,
};

#[repr(C)]
pub struct ArchivedString<P, E> {
    inner: ArchivedSlice<u8, P, E>,
}

impl<P, E> ArchivedString<P, E>
where
    P: Pointer,
    E: Endian,
{
    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.inner.as_slice()) }
    }

    pub fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.inner.as_slice().as_ptr() as *const c_char) }
    }
}

impl<P, E> core::fmt::Debug for ArchivedString<P, E>
where
    P: Pointer,
    E: Endian,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<P, E> Archived for ArchivedString<P, E>
where
    P: Pointer,
    E: Endian,
{
    type DeserializedType = String;

    fn deserialize(&self) -> Self::DeserializedType {
        self.as_str().into()
    }
}

impl Archive for str {
    type ArchiveType<P, E>
        = ArchivedString<P, E>
    where
        E: Endian,
        P: Pointer;
}

impl Serialize for str {
    fn serialize_fixed_data<S>(
        &self,
        writer: &mut S,
        tracker: &mut VariableDataTracker,
    ) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        self.as_bytes().serialize_fixed_data(writer, tracker)
    }

    fn track_variable_data<S>(&self, tracker: &mut VariableDataTracker)
    where
        S: Serializer,
    {
        self.as_bytes().track_variable_data::<S>(tracker);

        // Add NULL-termination
        tracker.add_data::<u8>(1);
    }

    fn serialize_variable_data<S>(&self, writer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        self.as_bytes().serialize_variable_data(writer)?;

        // Add NULL-termination
        writer.write_primitive(ArchivedPrimitive::<u8, S::Endian>::from(0))?;

        Ok(())
    }
}
