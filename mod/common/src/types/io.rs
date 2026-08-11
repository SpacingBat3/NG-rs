use reqwest::{Client,header};

/// A representation of any remote data
/// without any external metadata.
pub struct RemoteFileData {
    pub src:  Box<str>,
    pub mime: Box<str>,
}

/// A representation of images with metadata.
pub struct RemoteImage {
    pub image: RemoteFileData,
    pub alt: Option<Box<str>>,
    pub dim: (u32,u32),
}

/// A representation of any file format.
pub enum RemoteFile {
    Any(RemoteFileData),
    Image(RemoteImage)
}

impl RemoteFileData {
    /// Fetches file, using `session` parameter.
    pub async fn fetch(&self, session:&Client) -> reqwest::Result<Box<[u8]>> {
        let buff = session.get(&self.src as &str)
            .header(header::ACCEPT, &self.mime as &str)
            .send().await?
            .bytes()
            .await?;
        Ok(Vec::<u8>::from(buff).into_boxed_slice())
    }
}