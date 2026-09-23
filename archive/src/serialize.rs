use crate::{
    archive::Archive,
    serializer::{Padding, Serializer},
};

pub struct VariableDataTracker {
    pos: usize,
}

impl VariableDataTracker {
    pub fn new(pos: usize) -> Self {
        Self { pos }
    }

    pub fn add_data<T: Padding>(&mut self, count: usize) {
        self.pos += T::get_padding(self.pos);
        self.pos += size_of::<T>() * count;
    }

    pub fn relative_pointer<T: Padding>(&self, pos: usize) -> usize {
        let padding = T::get_padding(self.pos);

        self.pos - pos + padding
    }
}

pub trait Serialize: Archive {
    fn serialize_fixed_data<S>(
        &self,
        writer: &mut S,
        tracker: &mut VariableDataTracker,
    ) -> Result<(), S::Error>
    where
        S: Serializer;

    fn track_variable_data<S>(&self, tracker: &mut VariableDataTracker)
    where
        S: Serializer;

    fn serialize_variable_data<S>(&self, writer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer;

    fn serialize<S>(&self, writer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        let mut tracker = VariableDataTracker::new(writer.position());

        tracker.add_data::<Self::ArchiveType<S::Pointer, S::Endian>>(1);

        self.serialize_fixed_data(writer, &mut tracker)?;
        self.serialize_variable_data(writer)?;

        Ok(())
    }
}
