use std::{fs, io, path};
use std::io::Read;
use std::path::PathBuf;

use zip;
use serde_xml_rs;

use super::result::{Error, Result};
use super::container::Container;
use super::opf::Opf;
use super::ncx::Ncx;

pub struct Book {
    file: zip::read::ZipArchive<fs::File>,
}

impl Book {
    pub fn new(file: PathBuf) -> Result<Book> {
        let arc = try!(zip::ZipArchive::new(try!(fs::File::open(&file))));
        return Ok(Book { file: arc });
    }

    pub fn mimetype(&mut self) -> Result<String> {
        let mut buf = String::new();
        try!(try!(self.open("mimetype")).read_to_string(&mut buf));
        return Ok(buf);
    }

    pub fn container(&mut self) -> Result<Container> {
        let it: Container = try!(serde_xml_rs::deserialize(try!(self.open(
            "META-INF/container.xml"
        ))));
        return Ok(it);
    }

    pub fn index(&mut self) -> Result<String> {
        let mut ct = try!(self.container());
        let mut buf = String::new();

        for opf_n in ct.opf() {
            let mut opf = try!(self.opf(opf_n));
            if let Some(toc_n) = opf.toc() {
                let mut toc = try!(self.toc(opf_n, toc_n));
                buf.push_str(&toc.html());
            }
        }
        return Ok(buf);
    }

    pub fn show(&mut self, href: &str) -> Result<(String, String)> {
        let mut ct = try!(self.container());

        for opf_n in ct.opf() {
            let mut opf = try!(self.opf(opf_n));
            if let Some(root) = path::Path::new(opf_n).parent() {
                for it in &mut opf.manifest.item {
                    if let Some(name) = root.join(&it.href).to_str() {
                        if name == href {
                            let mut body = String::new();
                            try!(try!(self.open(name)).read_to_string(&mut body));
                            return Ok((it.media_type.clone(), body));
                        }
                    }
                }
            }
        }

        return Err(Error::Io(io::Error::new(io::ErrorKind::NotFound, href)));
    }

    pub fn opf(&mut self, name: &str) -> Result<Opf> {
        let it: Opf = try!(serde_xml_rs::deserialize(try!(self.open(name))));
        return Ok(it);
    }

    pub fn toc(&mut self, opf: &str, name: &str) -> Result<Ncx> {
        match path::Path::new(opf).parent() {
            Some(root) => match root.join(name).to_str() {
                Some(n) => {
                    let it: Ncx = try!(serde_xml_rs::deserialize(try!(self.open(n))));
                    return Ok(it);
                }
                None => Err(Error::Io(io::Error::new(io::ErrorKind::NotFound, name))),
            },
            None => Err(Error::Io(io::Error::new(io::ErrorKind::NotFound, opf))),
        }
    }

    pub fn read(&mut self, opf: &str, name: &str) -> Result<String> {
        match path::Path::new(opf).parent() {
            Some(root) => match root.join(name).to_str() {
                Some(n) => {
                    let mut buf = String::new();
                    try!(try!(self.open(n)).read_to_string(&mut buf));
                    return Ok(buf);
                }
                None => Err(Error::Io(io::Error::new(io::ErrorKind::NotFound, name))),
            },
            None => Err(Error::Io(io::Error::new(io::ErrorKind::NotFound, opf))),
        }
    }

    fn open<'a>(&'a mut self, name: &str) -> Result<zip::read::ZipFile<'a>> {
        let it = try!(self.file.by_name(name));
        return Ok(it);
    }
}
