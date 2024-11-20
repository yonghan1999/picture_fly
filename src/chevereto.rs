use crate::common::UploadResponse;
use crate::error::UploadError;
use reqwest::{multipart, Client, Error, Response};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use std::env;
use std::path::{Path, PathBuf};
use url::Url;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheveretoUploadResponse {
    #[serde(rename = "status_code")]
    pub status_code: u16,
    pub success: Success,
    pub image: Image,
    #[serde(rename = "status_txt")]
    pub status_txt: String,
}

impl UploadResponse for CheveretoUploadResponse {
    fn upload_file_url(&self) -> Url {
        Url::parse(&(self.image.url)).unwrap()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Success {
    pub message: String,
    pub code: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub name: String,
    pub extension: String,
    pub size: String,
    pub width: String,
    pub height: String,
    pub date: String,
    #[serde(rename = "date_gmt")]
    pub date_gmt: String,
    pub title: String,
    pub description: Value,
    pub nsfw: String,
    #[serde(rename = "storage_mode")]
    pub storage_mode: String,
    pub md5: String,
    #[serde(rename = "source_md5")]
    pub source_md5: Value,
    #[serde(rename = "original_filename")]
    pub original_filename: String,
    #[serde(rename = "original_exifdata")]
    pub original_exifdata: Value,
    pub views: String,
    #[serde(rename = "category_id")]
    pub category_id: Value,
    pub chain: String,
    #[serde(rename = "thumb_size")]
    pub thumb_size: String,
    #[serde(rename = "medium_size")]
    pub medium_size: String,
    #[serde(rename = "expiration_date_gmt")]
    pub expiration_date_gmt: Value,
    pub likes: String,
    #[serde(rename = "is_animated")]
    pub is_animated: String,
    #[serde(rename = "is_approved")]
    pub is_approved: String,
    pub file: File,
    #[serde(rename = "id_encoded")]
    pub id_encoded: String,
    pub filename: String,
    pub mime: String,
    pub url: String,
    #[serde(rename = "url_viewer")]
    pub url_viewer: String,
    #[serde(rename = "url_short")]
    pub url_short: String,
    pub image: Image2,
    pub thumb: Thumb,
    pub medium: Medium,
    #[serde(rename = "size_formatted")]
    pub size_formatted: String,
    #[serde(rename = "display_url")]
    pub display_url: String,
    #[serde(rename = "display_width")]
    pub display_width: String,
    #[serde(rename = "display_height")]
    pub display_height: i64,
    #[serde(rename = "views_label")]
    pub views_label: String,
    #[serde(rename = "likes_label")]
    pub likes_label: String,
    #[serde(rename = "how_long_ago")]
    pub how_long_ago: String,
    #[serde(rename = "date_fixed_peer")]
    pub date_fixed_peer: String,
    #[serde(rename = "title_truncated")]
    pub title_truncated: String,
    #[serde(rename = "title_truncated_html")]
    pub title_truncated_html: String,
    #[serde(rename = "is_use_loader")]
    pub is_use_loader: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub resource: Resource,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image2 {
    pub filename: String,
    pub name: String,
    pub mime: String,
    pub extension: String,
    pub url: String,
    pub size: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumb {
    pub filename: String,
    pub name: String,
    pub mime: String,
    pub extension: String,
    pub url: String,
    pub size: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Medium {
    pub filename: String,
    pub name: String,
    pub mime: String,
    pub extension: String,
    pub url: String,
    pub size: String,
}

/// url: chevereto upload url
/// image: image file absolute path
async fn upload_local_image_chevereto(
    url: &Url,
    image: &Path,
) -> Result<CheveretoUploadResponse, UploadError> {
    if !image.exists() || !image.is_file() {
        return Err(UploadError::new(&format!(
            "{} is not exits",
            image.to_str().unwrap()
        )));
    }
    let client = Client::new();
    let image_file_res = multipart::Form::new().file("source", image).await;
    let form;
    match image_file_res {
        Ok(file_form) => form = file_form,
        Err(error) => {
            return Err(UploadError::new(&format!(
                "{} {}",
                image.to_str().unwrap(),
                error.to_string()
            )));
        }
    }

    let res = client.post(url.clone()).multipart(form).send().await;
    return resp_result(res).await;
}

/// 上传远程图片，http链接
async fn upload_remote_image_chevereto(
    url: &Url,
    image: &Url,
) -> Result<CheveretoUploadResponse, UploadError> {
    let req_url = format!("{}&source={}", url.as_str(), image.as_str());
    let client = Client::new();
    let res = client.post(req_url).send().await;
    return resp_result(res).await;
}

/// 处理请求返回结果
async fn resp_result(res: Result<Response, Error>) -> Result<CheveretoUploadResponse, UploadError> {
    match res {
        Ok(res) => {
            if !res.status().is_success() {
                return Err(UploadError::new("Some errors caused the upload to fail. Please check the chevereto configuration and command parameters."));
            }
            let resp_body: Result<CheveretoUploadResponse, Error> = res.json().await;
            match resp_body {
                Ok(resp_body) => Ok(resp_body),
                Err(error) => Err(UploadError::new(error.to_string().as_str())),
            }
        }
        Err(error) => Err(UploadError::new(error.to_string().as_str())),
    }
}

pub async fn upload_image_cheverto(
    url_string: &String,
    key: &String,
    image: &String,
) -> Result<CheveretoUploadResponse, UploadError> {
    let upload_url = Url::parse_with_params(
        url_string.as_str(),
        &[("key", key.as_str()), ("format", "json")],
    )
    .expect("url parse error.");

    if image.starts_with("https://") || image.starts_with("http://") {
        let image_url = Url::parse(&image).unwrap();
        upload_remote_image_chevereto(&upload_url, &image_url).await
    } else {
        let mut image_absolute_path: PathBuf = env::current_dir().expect("can not get current directory.");
        image_absolute_path.push(&image);
        let image_path = Path::new(image_absolute_path.as_path());
        upload_local_image_chevereto(&upload_url, &image_path).await
    }
}
