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

    fn fail(e: result::Error) {
        println!("fail: {:?}", e);
        assert!(false);
    }

    #[test]
    fn it_works() {
        match open(Path::new("tmp").join("test.epub")) {
            Ok(mut bk) => {
                // mimetype
                match bk.mimetype() {
                    Ok(t) => println!("mimetype: {:?}", t),
                    Err(e) => fail(e),
                };
                // container
                match bk.container() {
                    Ok(mut ct) => {
                        println!("container: {:?}", ct);
                        for opf_n in ct.opf() {
                            match bk.opf(opf_n) {
                                Ok(mut opf) => {
                                    println!("find opf: {:?}\n{:?}", opf_n, opf);
                                    match opf.toc() {
                                        Some(toc_n) => {
                                            println!("toc: {:?}", toc_n);
                                            // test toc
                                            match bk.toc(opf_n, toc_n) {
                                                Ok(toc) => {
                                                    println!("{:?}", toc);
                                                }
                                                Err(e) => fail(e),
                                            }
                                        }
                                        None => assert!(false, "toc not find"),
                                    }
                                }
                                Err(e) => fail(e),
                            }
                        }
                    }
                    Err(e) => fail(e),
                };
                // index
                match bk.index() {
                    Ok(h) => println!("index.html\n{:?}", h),
                    Err(e) => fail(e),
                }
                // file types
                match bk.show("OEBPS/toc.xhtml") {
                    Ok((h, b)) => println!("header: {}\nbody: \n{}", h, b),
                    Err(e) => fail(e),
                }
            }
            Err(e) => fail(e),
        };
    }
}
