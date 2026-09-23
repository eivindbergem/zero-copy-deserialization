use archive::{
    archive::{Archive, Archived},
    endian::{Endian, LittleEndian},
    pointer::Pointer,
    primitive::ArchivedPrimitive,
    serialize::Serialize,
    string::ArchivedString,
};

#[derive(Debug, PartialEq)]
#[repr(u32)]
pub enum Enum {
    UnitVariant,
    TupleVariant(String),
    StructVariant { value1: u32, value2: i64 },
}

#[cfg(target_endian = "little")]
mod discriminants {
    pub const UNIT_VARIANT: u32 = 0x00000000;
    pub const TUPLE_VARIANT: u32 = 0x00000001;
    pub const STRUCT_VARIANT: u32 = 0x00000002;
}

#[cfg(target_endian = "big")]
mod discriminants {
    pub const UNIT_VARIANT: u32 = 0x00000000;
    pub const TUPLE_VARIANT: u32 = 0x01000000;
    pub const STRUCT_VARIANT: u32 = 0x02000000;
}

#[repr(C)]
#[repr(u32)]
// #[derive(Debug)]
pub enum ArchivedEnum<P, E>
where
    P: Pointer,
    E: Endian,
{
    UnitVariant = discriminants::UNIT_VARIANT,
    TupleVariant(ArchivedString<P, E>) = discriminants::TUPLE_VARIANT,
    StructVariant {
        value1: ArchivedPrimitive<u32, E>,
        value2: ArchivedPrimitive<i64, E>,
    } = discriminants::STRUCT_VARIANT,
}

impl<P, E> core::fmt::Debug for ArchivedEnum<P, E>
where
    P: Pointer,
    E: Endian,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnitVariant => write!(f, "UnitVariant"),
            Self::TupleVariant(arg0) => f.debug_tuple("TupleVariant").field(arg0).finish(),
            Self::StructVariant { value1, value2 } => {
                dbg!(self as *const _, value1 as *const _, value2 as *const _);

                f.debug_struct("StructVariant")
                    .field("value1", value1)
                    .field("value2", value2)
                    .finish()
            }
        }
    }
}

impl<P, E> Archived for ArchivedEnum<P, E>
where
    P: Pointer,
    E: Endian,
{
    type DeserializedType = Enum;

    fn deserialize(&self) -> Self::DeserializedType {
        match self {
            ArchivedEnum::UnitVariant => Enum::UnitVariant,
            ArchivedEnum::TupleVariant(archived_string) => {
                Enum::TupleVariant(archived_string.deserialize().into())
            }
            ArchivedEnum::StructVariant { value1, value2 } => Enum::StructVariant {
                value1: value1.deserialize(),
                value2: value2.deserialize(),
            },
        }
    }
}

impl Archive for Enum {
    type ArchiveType<P, E>
        = ArchivedEnum<P, E>
    where
        E: Endian,
        P: Pointer;
}

impl Serialize for Enum {
    fn serialize_fixed_data<S>(
        &self,
        writer: &mut S,
        tracker: &mut archive::serialize::VariableDataTracker,
    ) -> Result<(), S::Error>
    where
        S: archive::serializer::Serializer,
    {
        let start = writer.position();

        let tag = match self {
            Enum::UnitVariant => 0,
            Enum::TupleVariant(_) => 1,
            Enum::StructVariant { .. } => 2,
        };

        writer.write_discriminant(ArchivedPrimitive::<u32, LittleEndian>::from_usize(tag))?;
        writer.write_padding::<Self::ArchiveType<S::Pointer, S::Endian>>()?;

        match self {
            Enum::UnitVariant => {}
            Enum::TupleVariant(string) => {
                string.serialize_fixed_data(writer, tracker)?;
            }
            Enum::StructVariant { value1, value2 } => {
                value1.serialize_fixed_data(writer, tracker)?;
                value2.serialize_fixed_data(writer, tracker)?;
            }
        }

        let written = writer.position() - start;
        let padding = size_of::<Self::ArchiveType<S::Pointer, S::Endian>>() - written;
        writer.write_zeros(padding)?;

        Ok(())
    }

    fn track_variable_data<S>(&self, tracker: &mut archive::serialize::VariableDataTracker)
    where
        S: archive::serializer::Serializer,
    {
        match self {
            Enum::UnitVariant => (),
            Enum::TupleVariant(string) => string.track_variable_data::<S>(tracker),
            Enum::StructVariant { value1, value2 } => {
                value1.track_variable_data::<S>(tracker);
                value2.track_variable_data::<S>(tracker);
            }
        }
    }

    fn serialize_variable_data<S>(&self, writer: &mut S) -> Result<(), S::Error>
    where
        S: archive::serializer::Serializer,
    {
        match self {
            Enum::UnitVariant => (),
            Enum::TupleVariant(string) => string.serialize_variable_data(writer)?,
            Enum::StructVariant { value1, value2 } => {
                value1.serialize_variable_data(writer)?;
                value2.serialize_variable_data(writer)?;
            }
        }

        Ok(())
    }
}

pub fn enum_unit() -> Enum {
    Enum::UnitVariant
}

pub fn enum_tuple() -> Enum {
    Enum::TupleVariant("All your base are belong to us".into())
}

pub fn enum_struct() -> Enum {
    Enum::StructVariant {
        value1: 0x12345678,
        value2: -273,
    }
}
