use crate::{endian::Endian, pointer::Pointer};

#[derive(Debug)]
pub enum Error {
    BufferTooSmall { expected: usize, found: usize },
}

pub trait Archived {
    type DeserializedType<'a>;
}

pub trait Archive {
    type ArchiveType<P, E>: core::fmt::Debug + Archived
    where
        E: Endian,
        P: Pointer;

    fn from_bytes<P, E>(buffer: &[u8]) -> Result<&Self::ArchiveType<P, E>, Error>
    where
        E: Endian,
        P: Pointer,
    {
        if buffer.len() < size_of::<Self::ArchiveType<P, E>>() {
            return Err(Error::BufferTooSmall {
                expected: size_of::<Self::ArchiveType<P, E>>(),
                found: buffer.len(),
            });
        }

        assert!(buffer.len() >= size_of::<Self::ArchiveType<P, E>>());
        let ptr = buffer.as_ptr();
        let archive = unsafe { &*(ptr as *const Self::ArchiveType<P, E>) };

        Ok(archive)
    }
}

pub trait Deserialize: Archived {
    fn deserialize(&self) -> Self::DeserializedType<'_>;
}
