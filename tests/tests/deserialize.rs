use std::path::Path;

use archive::{
    archive::{Archive, Archived},
    endian::LittleEndian,
    serialize::Serialize,
};
use tests::{endian::endian, layout::layout, simple::simple};

fn from_bytes<T: Serialize + Archive + std::fmt::Debug>(
    buf: &mut Vec<u8>,
) -> &T::ArchiveType<LittleEndian> {
    let min_len = size_of::<T::ArchiveType<LittleEndian>>();

    dbg!(&buf);

    if buf.len() < min_len {
        buf.resize(min_len, 0);
    }

    T::from_bytes::<LittleEndian>(buf).unwrap()
}

fn run_test<T>(name: &str, expect: T)
where
    T: Serialize + Archive + std::fmt::Debug + PartialEq,
{
    let dir = Path::new("samples");
    let mut bytes = std::fs::read(dir.join(name)).unwrap();
    let found = from_bytes::<T>(&mut bytes).deserialize();

    assert_eq!(found, expect);
}

#[test]
fn test_simple() {
    run_test("simple", simple());
}

#[test]
fn test_endian() {
    run_test("endian", endian());
}

#[test]
fn test_layout() {
    run_test("layout", layout());
}
