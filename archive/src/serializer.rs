use std::{io::Write, marker::PhantomData};

use crate::{endian::Endian, primitive::ArchivedPrimitive};

pub trait Serializer
where
    Self: Sized,
{
    type Endian: Endian;
    type Error;

    fn write_primitive<T>(
        &mut self,
        value: ArchivedPrimitive<T, Self::Endian>,
    ) -> Result<(), Self::Error> {
        self.write(value.as_bytes())?;

        Ok(())
    }

    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error>;
    fn position(&self) -> usize;
}

pub struct StdSerializer<W, E> {
    writer: W,
    pos: usize,
    __: PhantomData<E>,
}

impl<W, E> StdSerializer<W, E> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            pos: 0,
            __: PhantomData,
        }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W, E> Serializer for StdSerializer<W, E>
where
    W: Write,
    E: Endian,
{
    type Endian = E;
    type Error = std::io::Error;

    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.writer.write_all(bytes)?;
        self.pos += bytes.len();

        Ok(())
    }

    fn position(&self) -> usize {
        self.pos
    }
}
