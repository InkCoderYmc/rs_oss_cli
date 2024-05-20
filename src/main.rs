use rs_oss_cli::*;
use std::path::Path;

#[tokio::main]
async fn main() {
    let client = OssClient::new(
        "T90UlXgHTqGfovOh".to_string(),
        "tPIA6vHV7Jxkpmr4Ria75pj773AqDAZI".to_string(),
        "http://jssz-inner-boss.bilibili.co".to_string(),
        "jssz-inner".to_string(),
        "llm_snapshot".to_string());

    let objects_list = client.list_objects("archimedes/eval").await;
    println!("{:?}", objects_list);
    // let key = "archimedes/train/yuanmingchao";
    // let res = client.delete_object(key).await;
    // let local_path = "./test";
    // let path_pair = DirPathPair::new(key.to_string(), local_path.to_string());
    // let file_check = client.upload_object(path_pair).await;
    // let _ = client.download_dir(path_pair).await;
    // client.upload_dir(path_pair).await;
    // client.delete_dir(key).await;

    // println!("{:?}", files);
}