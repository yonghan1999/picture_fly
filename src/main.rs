mod chevereto;
mod common;
mod error;

use crate::chevereto::upload_image_cheverto;
use crate::common::UploadResponse;
use crate::error::UploadError;
use clap::Parser;
use futures::future::join_all;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(
        short,
        long,
        value_name = "suppress error messages.",
        default_value_t = false
    )]
    force: bool,
    #[arg(
        short,
        long,
        value_name = "print the successfully uploaded files.",
        default_value_t = true
    )]
    print: bool,
    /// chevereto api key
    key: String,
    /// your chevereto upload url e.g. https://images.hanblog.fun/api/1/upload/
    url: String,
    /// image file path. e.g. https://image.hanblog.fun/images/2024/11/20/test.jpg OR local file path.
    files: Vec<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if cli.files.is_empty() {
        return;
    }

    let errs = Arc::new(RwLock::new(Vec::<UploadError>::new()));
    let resp = Arc::new(RwLock::new(Vec::<Box<dyn UploadResponse>>::new()));
    process(&cli, &errs, &resp).await;
    print(&cli, errs, resp).await;
}

async fn process(
    cli: &Cli,
    errs: &Arc<RwLock<Vec<UploadError>>>,
    resp: &Arc<RwLock<Vec<Box<dyn UploadResponse>>>>,
) {
    let mut all_handle = vec![];
    for file_path in &cli.files {
        let c_file_path = file_path.clone();
        let c_url = cli.url.clone();
        let upload_errs = errs.clone();
        let upload_resp = resp.clone();
        let c_key = cli.key.clone();

        let handle = tokio::spawn(async move {
            let file_path = c_file_path;
            let key = c_key;
            let url = c_url;
            let res = upload_image_cheverto(&url, &key, &file_path).await;
            match res {
                Ok(response) => {
                    upload_resp.write().await.push(Box::new(response));
                }
                Err(error) => upload_errs.write().await.push(error),
            }
        });
        all_handle.push(handle);
    }
    join_all(all_handle).await;
}

async fn print(
    cli: &Cli,
    errs: Arc<RwLock<Vec<UploadError>>>,
    resp: Arc<RwLock<Vec<Box<dyn UploadResponse>>>>,
) {
    if cli.force == false {
        let errs_read = errs.read().await;
        if !errs_read.is_empty() {
            println!("Errors:");
            errs_read.iter().for_each(|error| {
                println!("{}", error);
            });
        }
    }
    if cli.print {
        let resp_read = resp.read().await;
        if !resp_read.is_empty() {
            println!("Upload Success:");
            resp_read.iter().for_each(|res| {
                println!("{}", res.upload_file_url());
            });
        }
    }
}
