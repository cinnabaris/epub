#[derive(Serialize, Deserialize, Debug)]
pub struct Ncx {
    pub version: String,
    pub head: Head,
    #[serde(rename = "docTitle")]
    pub doc_title: DocTitle,
    #[serde(rename = "navMap")]
    pub nav_map: NavMap,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Head {
    pub meta: Vec<Meta>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub name: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocTitle {
    pub text: Text,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavMap {
    #[serde(rename = "navPoint")]
    pub nav_point: Vec<NavPoint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavPoint {
    pub id: String,
    #[serde(rename = "playOrder")]
    pub play_order: String,
    #[serde(rename = "navLabel")]
    pub nav_label: NavLabel,
    pub content: Content,
    #[serde(rename = "navPoint", default)]
    pub nav_point: Vec<NavPoint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavLabel {
    pub text: Text,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub src: String,
}
