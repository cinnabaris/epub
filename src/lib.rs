extern crate zip;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

pub mod result;
pub mod book;
pub mod container;
pub mod opf;
pub mod ncx;

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
        match book::Book::new(Path::new("tmp").join("test.epub")) {
            Ok(mut bk) => {
                match bk.mimetype() {
                    Ok(t) => println!("mimetype: {:?}", t),
                    Err(e) => fail(e),
                };
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
            }
            Err(e) => fail(e),
        };
    }
}
