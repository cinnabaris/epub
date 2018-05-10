extern crate epub;

use std::path::{Path, PathBuf};

fn parse(file: PathBuf) -> epub::result::Result<()> {
    let bk = epub::open(file)?;

    // identifier
    println!("identifier: {}", bk.uid()?);

    // home-page
    println!("home page: \n{}", bk.index()?);

    // page content
    match bk.list()?.first() {
        Some((href, media_type)) => {
            println!("---- {} {} ----", href, media_type);
            let (body, _) = bk.show(&href)?;
            println!("{}", String::from_utf8(body)?);
        }
        None => {}
    }

    // mimetype
    println!("mime type: {}", bk.mimetype()?);

    let ct = bk.container()?;
    for opf_n in ct.opf() {
        println!("=== find opf: {} ===", opf_n);
        let opf = bk.opf(&opf_n)?;
        // creator
        let mt = opf.metadata;
        println!("creator: {}", mt.creator.content);
        println!("identifier: {}", mt.identifier.content);
        println!("title: {}", mt.title.content);
        println!("language: {}", mt.language.content);
        println!("publisher: {}", mt.publisher.content);
        println!("date: {}", mt.date.content);
        for mf in opf.manifest.item {
            println!("manifest {} {}", mf.href, mf.media_type);
        }
    }

    Ok(())
}

#[test]
fn it_works() {
    parse(Path::new("tmp").join("test.epub")).unwrap();
}
