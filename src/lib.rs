extern crate zip;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

use std::{fs, path};

pub mod result;
pub mod book;
pub mod container;
pub mod opf;
pub mod ncx;

pub fn open(file: path::PathBuf) -> result::Result<book::Book> {
    return Ok(book::Book::new(try!(zip::ZipArchive::new(try!(
        fs::File::open(&file)
    )))));
}
