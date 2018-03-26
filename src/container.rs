#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub version: String,
    pub rootfiles: RootFiles,
}

impl Container {
    pub fn opf(&mut self) -> Vec<&String> {
        let files: Vec<&String> = self.rootfiles
            .rootfile
            .iter()
            .map(|it| &it.full_path)
            .collect();
        return files;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootFiles {
    pub rootfile: Vec<RootFile>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootFile {
    #[serde(rename = "full-path")]
    pub full_path: String,
    #[serde(rename = "media-type")]
    pub media_type: String,
}

impl RootFile {}
