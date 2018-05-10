use std::io::Read;
use std::sync::{Arc, Mutex};
use std::{fs, io, path::Path};

use serde_xml_rs;
use zip;

use super::container::Container;
use super::ncx::Ncx;
use super::opf::Opf;
use super::result::{Error, Result};

pub struct Book {
    file: Arc<Mutex<zip::read::ZipArchive<fs::File>>>,
}

impl Book {
    pub fn new(file: zip::read::ZipArchive<fs::File>) -> Book {
        return Book {
            file: Arc::new(Mutex::new(file)),
        };
    }

    pub fn mimetype(&self) -> Result<String> {
        Ok(String::from_utf8(self.open(&"mimetype".to_string())?)?)
    }

    pub fn container(&self) -> Result<Container> {
        let it: Container = serde_xml_rs::deserialize(
            self.open(&"META-INF/container.xml".to_string())?.as_slice(),
        )?;
        return Ok(it);
    }

    pub fn index(&self) -> Result<String> {
        let ct = try!(self.container());
        let mut buf = String::new();

        for opf_n in ct.opf() {
            let mut opf = try!(self.opf(&opf_n));
            if let Some(toc_n) = opf.toc() {
                let mut toc = try!(self.toc(&opf_n, &toc_n));
                buf.push_str(&toc.html());
            }
        }
        return Ok(buf);
    }

    pub fn show(&self, href: &String) -> Result<(String, String)> {
        let ct = try!(self.container());

        for opf_n in ct.opf() {
            let mut opf = try!(self.opf(&opf_n));
            if let Some(root) = Path::new(&opf_n).parent() {
                for it in &mut opf.manifest.item {
                    if let Some(name) = root.join(&it.href).to_str() {
                        if name == href {
                            return Ok((
                                it.media_type.clone(),
                                String::from_utf8(self.open(&name.to_string())?)?,
                            ));
                        }
                    }
                }
            }
        }

        return Err(Error::Io(io::Error::new(
            io::ErrorKind::NotFound,
            format!("can't find page {}", href),
        )));
    }

    pub fn opf(&self, name: &String) -> Result<Opf> {
        let it: Opf = serde_xml_rs::deserialize(self.open(name)?.as_slice())?;
        return Ok(it);
    }

    pub fn toc(&self, opf: &String, name: &String) -> Result<Ncx> {
        match Path::new(opf).parent() {
            Some(root) => match root.join(name).to_str() {
                Some(n) => {
                    let it: Ncx = serde_xml_rs::deserialize(self.open(&n.to_string())?.as_slice())?;
                    return Ok(it);
                }
                None => Err(Error::Io(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("can't find toc for name {}", name),
                ))),
            },
            None => Err(Error::Io(io::Error::new(
                io::ErrorKind::NotFound,
                format!("can't find toc for opf {}", opf),
            ))),
        }
    }

    pub fn read(&self, opf: &String, name: &String) -> Result<String> {
        match Path::new(opf).parent() {
            Some(root) => match root.join(name).to_str() {
                Some(n) => Ok(String::from_utf8(self.open(&n.to_string())?)?),
                None => Err(Error::Io(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("bad name {}", name),
                ))),
            },
            None => Err(Error::Io(io::Error::new(
                io::ErrorKind::NotFound,
                format!("bad opf {}", opf),
            ))),
        }
    }

    fn open(&self, name: &String) -> Result<Vec<u8>> {
        let file = self.file.clone();
        let file = file.lock();
        match file {
            Ok(mut file) => {
                let mut file = try!(file.by_name(&name[..]));
                let mut buf = Vec::new();
                file.read_to_end(&mut buf)?;
                Ok(buf)
            }
            Err(e) => Err(Error::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{:?}", e),
            ))),
        }
    }
}
