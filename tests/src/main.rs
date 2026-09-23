use std::{fs::File, path::Path};

use archive::{endian::LittleEndian, serialize::Serialize, serializer::StdSerializer};
use tests::{
    endian::endian, layout::layout, nested_slice::nested_slice, simple::simple, slice::slice,
};

fn write_to_file<T: Serialize>(name: &str, item: &T) {
    let dir = Path::new("samples");
    std::fs::create_dir_all(dir).unwrap();

    let mut serializer =
        StdSerializer::<_, u16, LittleEndian>::new(File::create(dir.join(name)).unwrap());
    item.serialize(&mut serializer).unwrap();
}

fn main() {
    write_to_file("simple", &simple());
    write_to_file("endian", &endian());
    write_to_file("layout", &layout());
    write_to_file("slice", &slice());
    write_to_file("nested_slice", &nested_slice());
}
