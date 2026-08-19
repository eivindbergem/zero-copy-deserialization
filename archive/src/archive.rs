#[derive(Debug)]
pub enum Error {
    BufferTooSmall { expected: usize, found: usize },
}

pub trait Archived {
    type DeserializedType;

    fn deserialize(&self) -> Self::DeserializedType;
}

pub trait Archive {
    type ArchiveType: core::fmt::Debug + Archived<DeserializedType = Self>;

    fn from_bytes(buffer: &[u8]) -> Result<&Self::ArchiveType, Error> {
        if buffer.len() < size_of::<Self::ArchiveType>() {
            return Err(Error::BufferTooSmall {
                expected: size_of::<Self::ArchiveType>(),
                found: buffer.len(),
            });
        }

        assert!(buffer.len() >= size_of::<Self::ArchiveType>());
        let ptr = buffer.as_ptr();
        let archive = unsafe { &*(ptr as *const Self::ArchiveType) };

        Ok(archive)
    }
}
