use archive::{
    archive::{Archive, Archived},
    endian::LittleEndian,
    serialize::Serialize,
};
use tests::{endian::endian, layout::layout, simple::simple, slice::slice};

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
