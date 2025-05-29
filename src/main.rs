use anyhow::{Context, Result};
use clap::Parser;
use reqwest::multipart::{Form, Part};
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 本地目录路径
    #[arg(short = 'd', required = true)]
    directory: PathBuf,

    /// 上传目标URL
    #[arg(required = true)]
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // 验证目录是否存在
    if !args.directory.exists() {
        anyhow::bail!("目录不存在: {:?}", args.directory);
    }

    // 遍历目录中的所有文件
    for entry in WalkDir::new(&args.directory) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let relative_path = file_path.strip_prefix(&args.directory)?;
            
            // 构建完整的URL
            let url = format!("{}/{}", args.url.trim_end_matches('/'), relative_path.to_string_lossy());
            
            // 读取文件内容
            let mut file = File::open(file_path).await?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).await?;
            
            // 创建multipart表单
            let form = Form::new()
                .part("file", Part::bytes(contents).file_name(relative_path.to_string_lossy().to_string()));
            
            // 发送请求
            let client = reqwest::Client::new();
            let response = client
                .post(&url)
                .multipart(form)
                .send()
                .await
                .context("上传文件失败")?;
            
            if response.status().is_success() {
                println!("成功上传: {}", relative_path.display());
            } else {
                eprintln!("上传失败 {}: {}", relative_path.display(), response.status());
            }
        }
    }

    Ok(())
}
