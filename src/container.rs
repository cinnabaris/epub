#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Container {
    pub version: String,
    pub rootfiles: RootFiles,
}

impl Container {
    pub fn opf(&self) -> Vec<String> {
        let files = self.rootfiles
            .rootfile
            .iter()
            .map(|it| it.full_path.clone())
            .collect::<Vec<_>>();
        return files;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RootFiles {
    pub rootfile: Vec<RootFile>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RootFile {
    #[serde(rename = "full-path")]
    pub full_path: String,
    #[serde(rename = "media-type")]
    pub media_type: String,
}

impl RootFile {}
