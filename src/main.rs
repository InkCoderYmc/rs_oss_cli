use rs_oss_cli::*;

#[tokio::main]
async fn main() -> () {
    let config = OssConfig::new(
        "T90UlXgHTqGfovOh".to_string(),
        "tPIA6vHV7Jxkpmr4Ria75pj773AqDAZI".to_string(),
        "http://jssz-inner-boss.bilibili.co".to_string(),
        "jssz-inner".to_string(),
        "llm_snapshot".to_string());

    config.list_objects("archimedes/eval").await;
    // println!("{:?}", res);
}