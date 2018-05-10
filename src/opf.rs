#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Opf {
    #[serde(rename = "unique-identifier")]
    pub unique_identifier: String,
    pub version: String,
    pub metadata: MetaData,
    pub manifest: Manifest,
    pub spine: Spine,
}

impl Opf {
    pub fn toc(&self) -> Option<String> {
        match self.manifest.item.iter().find(|it| it.id == self.spine.toc) {
            Some(it) => Some(it.href.clone()),
            None => None,
        }
    }

    pub fn media_type(&self, href: &String) -> Option<String> {
        match self.manifest.item.iter().find(|it| it.href == *href) {
            Some(it) => Some(it.media_type.clone()),
            None => None,
        }
    }
}

// FIXME: dc namespace
#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Dc {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    pub name: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manifest {
    pub item: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: String,
    pub href: String,
    #[serde(rename = "media-type")]
    pub media_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spine {
    pub toc: String,
    pub itemref: Vec<ItemRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemRef {
    pub idref: String,
}
