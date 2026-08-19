use std::io::Write;

use crate::primitive::ArchivedPrimitive;

pub trait Serializer
where
    Self: Sized,
{
    type Error;

    fn write_primitive<T>(&mut self, value: ArchivedPrimitive<T>) -> Result<(), Self::Error> {
        self.write(value.as_bytes())?;

        Ok(())
    }

    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error>;
    fn position(&self) -> usize;
}

pub struct StdSerializer<W> {
    writer: W,
    pos: usize,
}

impl<W> StdSerializer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer, pos: 0 }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W> Serializer for StdSerializer<W>
where
    W: Write,
{
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
