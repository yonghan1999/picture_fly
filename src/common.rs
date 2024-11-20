use url::Url;

pub trait UploadResponse: Send + Sync {
    fn upload_file_url(&self) -> Url;
}
