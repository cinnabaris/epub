#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ncx {
    pub version: String,
    pub head: Head,
    #[serde(rename = "docTitle")]
    pub doc_title: DocTitle,
    #[serde(rename = "navMap")]
    pub nav_map: NavMap,
}

impl Ncx {
    pub fn html(&self) -> String {
        let mut buf = String::from("<h2>");
        buf.push_str(&self.doc_title.text.content);
        buf.push_str("</h2>");
        for it in &self.nav_map.nav_point {
            buf.push_str(&it.html());
        }
        return buf;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Head {
    pub meta: Vec<Meta>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    pub name: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocTitle {
    pub text: Text,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavMap {
    #[serde(rename = "navPoint")]
    pub nav_point: Vec<NavPoint>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl NavPoint {
    pub fn html(&self) -> String {
        let mut buf = String::new();
        if self.nav_point.len() == 0 {
            return buf;
        }
        buf.push_str("<ul>");
        for it in &self.nav_point {
            buf.push_str("<a target=\"_blank\" href=\"");
            buf.push_str(&self.content.src);
            buf.push_str("\">");
            buf.push_str(&self.nav_label.text.content);
            buf.push_str("</a>");

            for jt in &it.nav_point {
                buf.push_str(&jt.html());
            }
        }
        buf.push_str("</ul>");
        return buf;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavLabel {
    pub text: Text,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Text {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
    pub src: String,
}
