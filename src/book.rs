use std::fs;
use std::io::Read;
use std::path::PathBuf;
use zip;
use super::result::Result;

pub struct Book {
    file: zip::read::ZipArchive<fs::File>,
}

impl Book {
    pub fn open(file: PathBuf) -> Result<Book> {
        let arc = try!(zip::ZipArchive::new(try!(fs::File::open(&file))));
        return Ok(Book { file: arc });
    }

    pub fn mimetype(&mut self) -> Result<String> {
        let mut buf = String::new();
        try!(try!(self.file.by_name("mimetype")).read_to_string(&mut buf));
        return Ok(buf);
    }
}
