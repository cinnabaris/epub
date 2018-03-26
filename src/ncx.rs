#[derive(Serialize, Deserialize, Debug)]
pub struct Ncx {
    pub version: String,
    pub head: Head,
    pub docTitle: DocTitle,
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
    pub navPoint: Vec<NavPoint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavPoint {
    pub id: String,
    pub playOrder: isize,
    #[serde(default)]
    pub navPoint: Vec<NavPoint>,
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
