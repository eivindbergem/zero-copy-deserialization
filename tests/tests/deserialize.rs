use archive::{
    archive::{Archive, Archived},
    serialize::Serialize,
};
use tests::{endian::endian, simple::simple};

fn from_bytes<T: Serialize + Archive + std::fmt::Debug>(buf: &mut Vec<u8>) -> &T::ArchiveType {
    let min_len = size_of::<T::ArchiveType>();

    dbg!(&buf);

    if buf.len() < min_len {
        buf.resize(min_len, 0);
    }

    unsafe { T::from_bytes(buf).unwrap() }
}

fn run_test<T>(serialized: &[u8], expect: T)
where
    T: Serialize + Archive + std::fmt::Debug + PartialEq,
    <T::ArchiveType as Archived>::DeserializedType: PartialEq<T> + std::fmt::Debug,
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
