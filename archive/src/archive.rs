use crate::endian::Endian;

#[derive(Debug)]
pub enum Error {
    BufferTooSmall { expected: usize, found: usize },
}

pub trait Archived {
    type DeserializedType;

    fn deserialize(&self) -> Self::DeserializedType;
}

pub trait Archive {
    type ArchiveType<E>: core::fmt::Debug + Archived<DeserializedType = Self>
    where
        E: Endian;

    unsafe fn from_bytes<E>(buffer: &[u8]) -> Result<&Self::ArchiveType<E>, Error>
    where
        E: Endian,
    {
        if buffer.len() < size_of::<Self::ArchiveType<E>>() {
            return Err(Error::BufferTooSmall {
                expected: size_of::<Self::ArchiveType<E>>(),
                found: buffer.len(),
            });
        }

        assert!(buffer.len() >= size_of::<Self::ArchiveType<E>>());
        let ptr = buffer.as_ptr();
        let archive = unsafe { &*(ptr as *const Self::ArchiveType<E>) };

        Ok(archive)
    }
}
