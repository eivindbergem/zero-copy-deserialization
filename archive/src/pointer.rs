use std::marker::PhantomData;

use crate::{
    endian::{Endian, ToFromEndian},
    primitive::ArchivedPrimitive,
};

pub trait Pointer:
    ToFromEndian + Copy + TryInto<usize> + TryFrom<usize> + core::fmt::Debug
{
}

impl<T> Pointer for T where
    T: ToFromEndian + Copy + TryInto<usize> + TryFrom<usize> + core::fmt::Debug
{
}

#[repr(C)]
pub struct RelPtr<T, P, E> {
    offset: ArchivedPrimitive<P, E>,
    _marker: PhantomData<T>,
}

impl<T, P, E> RelPtr<T, P, E>
where
    P: ToFromEndian + Copy + TryInto<usize>,
    E: Endian,
{
    pub fn as_ptr(&self) -> *const T {
        dbg!(self.offset.to_usize());
        let offset = self.offset.to_usize();
        let base = self as *const _ as *const u8;
        let ptr = unsafe { base.add(offset) };

        ptr as *const T
    }
}
