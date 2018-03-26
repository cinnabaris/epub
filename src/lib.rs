extern crate zip;

pub mod result;
pub mod book;

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn it_works() {
        let it = book::Book::open(Path::new("tmp").join("test.epub"));
        assert!(it.is_ok());
        let mut bk = it.unwrap();

        let mt = bk.mimetype();
        assert!(mt.is_ok());
        println!("mimetype {:?}", mt.unwrap());
    }
}
