use std::{fs::File, path::Path};

use archive::{serialize::Serialize, serializer::StdSerializer};
use tests::{endian::endian, simple::simple};

fn write_to_file<T: Serialize>(name: &str, item: &T) {
    let dir = Path::new("samples");
    std::fs::create_dir_all(dir).unwrap();

    let mut serializer = StdSerializer::new(File::create(dir.join(name)).unwrap());
    item.serialize(&mut serializer).unwrap();
}

fn main() {
    write_to_file("simple", &simple());
    write_to_file("endian", &endian());
}
