#[derive(Serialize, Deserialize, Debug)]
pub struct Opf {
    #[serde(rename = "unique-identifier")]
    unique_identifier: String,
    version: String,
    metadata: MetaData,
    manifest: Manifest,
    spine: Spine,
}

// FIXME: dc namespace
#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    #[serde(rename = "title")]
    pub title: Dc,
    #[serde(rename = "identifier")]
    pub identifier: Dc,

    #[serde(rename = "creator", default)]
    pub creator: Dc,
    #[serde(rename = "language", default)]
    pub language: Dc,
    #[serde(rename = "date", default)]
    pub date: Dc,
    #[serde(rename = "publisher", default)]
    pub publisher: Dc,
    #[serde(rename = "contributor", default)]
    pub contributor: Dc,
    #[serde(rename = "rights", default)]
    pub rights: Dc,

    pub meta: Vec<Meta>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Dc {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub name: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub item: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: String,
    pub href: String,
    #[serde(rename = "media-type")]
    pub media_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Spine {
    pub toc: String,
    pub itemref: Vec<ItemRef>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemRef {
    pub idref: String,
}