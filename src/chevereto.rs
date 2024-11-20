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
    pub url: String,
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
        let input_image_path = Path::new(&image);
        if !input_image_path.exists() || !input_image_path.is_file() {
            return Err(UploadError::new(&format!("{} is not exits", &image)));
        }
        let image_path_buf;
        if input_image_path.is_relative() {
            let mut image_absolute_path: PathBuf =
                env::current_dir().expect("can not get current directory.");
            image_absolute_path.push(&image);
            image_path_buf = image_absolute_path;
        } else {
            image_path_buf = PathBuf::from(&input_image_path);
        }
        upload_local_image_chevereto(&upload_url, &image_path_buf.as_path()).await
    }
}
