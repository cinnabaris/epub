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

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn it_works() {
        let mut bk = open(Path::new("tmp").join("test.epub")).unwrap();
        // mimetype
        let mt = bk.mimetype().unwrap();
        println!("mimetype: {:?}", mt);
        // container
        let mut ct = bk.container().unwrap();
        println!("container: {:?}", ct);
        for opf_n in ct.opf() {
            let mut opf = bk.opf(opf_n).unwrap();
            println!("find opf: {:?}\n{:?}", opf_n, opf);
            let toc_n = opf.toc().unwrap();
            println!("toc: {:?}", toc_n);
            // test toc
            let toc = bk.toc(opf_n, toc_n).unwrap();
            println!("{:?}", toc);
        }

        // index
        let html = bk.index().unwrap();
        println!("index.html\n{:?}", html);
        // file types
        let (head, body) = bk.show("OEBPS/toc.xhtml").unwrap();
        println!("header: {}\nbody: \n{}", head, body);
    }
}
