use rs_oss_cli::*;

#[tokio::main]
async fn main() {
    let client = OssClient::new(
        "T90UlXgHTqGfovOh".to_string(),
        "tPIA6vHV7Jxkpmr4Ria75pj773AqDAZI".to_string(),
        "http://jssz-inner-boss.bilibili.co".to_string(),
        "jssz-inner".to_string(),
        "llm_snapshot".to_string());

    // let objects_list = client.list_objects("archimedes/eval").await;
    let key = "archimedes/configs/baichuan2-13B/config.json";
    // let res = client.delete_object(key).await;
    // println!("{:?}", res);
    let local_path = "./test/1/2/config.json";
    let path_pair = PathPair {
        oss_path: key.to_string(),
        local_path: local_path.to_string(),
    };
    // let file_check = client.upload_object(path_pair).await;
    let _ = client.download_object(path_pair).await;
    // println!("{:?}", file_check);
}