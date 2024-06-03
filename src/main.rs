use std::env;
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
    #[arg(long, default_value = "")]
    config_path: String,

    /// 配置选项
    #[arg(long, default_value = "")]
    config_name: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    // 从环境变量中获取配置文件路径和配置选项，如果没有则使用命令行参数，环境变量优先级最高
    let mut config_path = env::var("OSS_CONFIG_PATH")
        .unwrap_or("".to_string());
    let mut config_name: String = env::var("OSS_CONFIG_NAME")
        .unwrap_or("".to_string());
    if args.config_path.is_empty() {
        if config_path.is_empty() {
            config_path = "./config.yaml".to_string();
        }
        println!("Using config file: {}", config_path);
    } else {
        config_path = args.config_path.clone();
        println!("Using config file: {}", config_path);
    }
    if args.config_name.is_empty() {
        if config_name.is_empty() {
            config_name = "default".to_string();
        }
        println!("Using config name: {}", args.config_name);
    } else {
        config_name = args.config_name.clone();
        println!("Using config name: {}", config_name);
    }
    let client = OssClient::new_from_file(&config_path, &config_name);
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