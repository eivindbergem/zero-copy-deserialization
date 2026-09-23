use std::{io::Write, marker::PhantomData};

use crate::{endian::Endian, pointer::Pointer, primitive::ArchivedPrimitive};

pub trait Padding: Sized {
    fn get_padding(pos: usize) -> usize {
        let alignment = align_of::<Self>();
        let modulo = pos % alignment;

        if modulo != 0 { alignment - modulo } else { 0 }
    }
}

impl<T> Padding for T where T: Sized {}

pub trait Serializer
where
    Self: Sized,
{
    type Endian: Endian;
    type Pointer: Pointer;
    type Error;

    fn write_primitive<T>(
        &mut self,
        value: ArchivedPrimitive<T, Self::Endian>,
    ) -> Result<(), Self::Error> {
        self.write_padding::<ArchivedPrimitive<T, Self::Endian>>()?;
        self.write(value.as_bytes())?;

        Ok(())
    }

    fn write_padding<T: Padding>(&mut self) -> Result<(), Self::Error> {
        let padding = T::get_padding(self.position());

        for _ in 0..padding {
            self.write(&[0])?;
        }

        Ok(())
    }

    fn write_pointer(&mut self, value: usize) -> Result<(), Self::Error> {
        let value = ArchivedPrimitive::<Self::Pointer, Self::Endian>::from_usize(value);
        self.write_primitive(value)?;

        Ok(())
    }

    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error>;
    fn position(&self) -> usize;
}

pub struct StdSerializer<W, P, E> {
    writer: W,
    pos: usize,
    __: PhantomData<(P, E)>,
}

impl<W, P, E> StdSerializer<W, P, E> {
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

impl<W, P, E> Serializer for StdSerializer<W, P, E>
where
    W: Write,
    P: Pointer,
    E: Endian,
{
    type Endian = E;
    type Pointer = P;
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
