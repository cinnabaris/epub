extern crate zip;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

use std::{fs, io, path::Path};

pub mod book;
pub mod container;
pub mod ncx;
pub mod opf;
pub mod result;

pub fn open<P: AsRef<Path>>(file: P) -> result::Result<book::Book> {
    if let Some(name) = file.as_ref().to_str() {
        return Ok(book::Book::new(zip::ZipArchive::new(fs::File::open(
            &name,
        )?)?));
    }
    Err(result::Error::Io(io::Error::new(
        io::ErrorKind::NotFound,
        format!("bad file {:?}", file.as_ref()),
    )))
}
