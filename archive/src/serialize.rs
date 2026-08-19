use crate::{archive::Archive, serializer::Serializer};

pub trait Serialize: Archive {
    fn serialize<S>(&self, writer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer;
}
