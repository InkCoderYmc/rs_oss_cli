use rs_oss_cli::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// 操作类型：上传、下载、删除
    #[arg(short, long)]
    action: String,

    /// 是否启用目录
    #[arg(short, long, default_value_t = false)]
    enable_dir: bool,

    /// 本地路径
    #[arg(short, long)]
    local_path: String,

    /// OSS路径
    #[arg(short, long)]
    oss_path: String,

    /// 配置文件路径
    #[arg(long, default_value = "./config.yaml")]
    config_path: String,

    /// 配置选项
    #[arg(long, default_value = "default")]
    config_name: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let client = OssClient::new_from_file(&args.config_path, &args.config_name);
    match args.action.as_str() {
        "upload" => {
            if args.enable_dir {
                client.upload_dir(DirPathPair::new(args.oss_path, args.local_path)).await;
            } else {
                let _ = client.upload_object(PathPair::new(args.oss_path, args.local_path)).await;
            }
        }
        "download" => {
            if args.enable_dir {
                client.download_dir(DirPathPair::new(args.oss_path, args.local_path)).await;
            } else {
                let _ = client.download_object(PathPair::new(args.oss_path, args.local_path)).await;
            }
        }
        "delete" => {
            if args.enable_dir {
                client.delete_dir(&args.oss_path).await;
            } else {
                let _ = client.delete_object(&args.oss_path).await;
            }
        }
        "list" => {
            let oss_files = client.list_objects(&args.oss_path).await;
            for oss_file in oss_files {
                println!("{}", oss_file);
            }
        }
        _ => {
            println!("Invalid action: {}", args.action);
        }
    }
}