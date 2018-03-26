extern crate zip;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

pub mod result;
pub mod book;
pub mod meta;

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
        match book::Book::open(Path::new("tmp").join("test.epub")) {
            Ok(mut bk) => {
                match bk.mimetype() {
                    Ok(t) => println!("mimetype: {:?}", t),
                    Err(e) => fail(e),
                };
                match bk.container() {
                    Ok(mut ct) => {
                        println!("container: {:?}", ct);
                        for name in ct.opf() {
                            match bk.opf(name) {
                                Ok(opf) => {
                                    println!("find opf: {:?}\n{:?}", name, opf);
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
