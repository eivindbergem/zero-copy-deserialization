use archive::{
    archive::{Archive, Archived},
    endian::LittleEndian,
    serialize::Serialize,
};
use tests::{
    endian::endian,
    enums::{enum_struct, enum_tuple, enum_unit},
    layout::layout,
    nested_slice::nested_slice,
    simple::simple,
    slice::slice,
    string::string,
};

fn from_bytes<T: Serialize + Archive + std::fmt::Debug>(
    buf: &mut Vec<u8>,
) -> &T::ArchiveType<u16, LittleEndian> {
    let min_len = size_of::<T::ArchiveType<u16, LittleEndian>>();

    dbg!(&buf);

    if buf.len() < min_len {
        buf.resize(min_len, 0);
    }

    unsafe {T::from_bytes::<u16, LittleEndian>(buf).unwrap()}
}

fn run_test<T>(serialized: &[u8], expect: T)
where
    T: Serialize + Archive + std::fmt::Debug + PartialEq,
    <T::ArchiveType<u16, LittleEndian> as Archived>::DeserializedType:
        PartialEq<T> + std::fmt::Debug,
{
    let mut serialized = serialized.to_vec();
    let archived = from_bytes::<T>(&mut serialized);

    dbg!(&archived);
    let deserialized = archived.deserialize();

    assert_eq!(deserialized, expect);
}

#[test]
fn test_simple() {
    run_test(include_bytes!("../samples/simple"), simple());
}

#[test]
fn test_endian() {
    run_test(include_bytes!("../samples/endian"), endian());
}

#[test]
fn test_layout() {
    run_test(include_bytes!("../samples/layout"), layout());
}

#[test]
fn test_slice() {
    run_test(include_bytes!("../samples/slice"), slice());
}

#[test]
fn test_nested_slice() {
    run_test(include_bytes!("../samples/nested_slice"), nested_slice());
}

#[test]
fn test_string() {
    run_test(include_bytes!("../samples/string"), string());
}

#[test]
fn test_enum_unit() {
    run_test(include_bytes!("../samples/enum_unit"), enum_unit());
}

#[test]
fn test_enum_tuple() {
    run_test(include_bytes!("../samples/enum_tuple"), enum_tuple());
}

#[test]
fn test_enum_struct() {
    run_test(include_bytes!("../samples/enum_struct"), enum_struct());
}
