
use file_download::download::DownloadOptions;
use clap::Parser;
use regex::Regex;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args{
    #[clap(parse(try_from_str=check_url))]
    url: String,
    #[clap(short, long)]
    file_name: Option<String>,
    #[clap(short, long, default_value_t=1024*1024)]
    batch_size: u64,
    #[clap(short, long)]
    overwrite: bool

}

fn check_url(s: &str) -> Result<String,String>{
    let re = Regex::new("(^http://)|(^https://)").unwrap();
    if re.is_match(s) {
        Ok(s.to_string())
    }else{
        Err("No validate download url provide.".to_string())
    }
}
pub async fn do_download(args:Args){

    let u:String = args.url;
    let d = DownloadOptions::new()
        .batch_size(args.batch_size)
        .url(u)
        .overwrite(args.overwrite)
        .build().await;
    match d{
        Some(mut downloader) => {
            downloader.download().await;
        },
        None => {

        },
    }
}
#[tokio::main]
async fn main() {
    let args = Args::parse();
    do_download(args).await;
}
#[tokio::test]
async fn main_test(){
    let args = Args{ 
        url: "https://go.dev/dl/go1.18.3.linux-amd64.tar.gz".to_string(), 
        file_name: None, 
        batch_size: 10,
        overwrite: true
     };
    do_download(args).await;
}
#[tokio::test]
async fn large_test(){
    let args = Args{ 
        url: "mirrors.aliyun.com/archlinux/iso/2022.05.01/archlinux-2022.05.01-x86_64.iso".to_string(), 
        file_name: None, 
        batch_size: 5*1024*1024,
        overwrite: true
     };
    do_download(args).await;
}