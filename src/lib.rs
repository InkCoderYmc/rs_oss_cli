extern crate serde;
extern crate serde_yaml;
use aws_config::{SdkConfig,Region};
use aws_sdk_s3::{config::{Credentials,SharedCredentialsProvider}, error::SdkError, operation::{delete_object::{DeleteObjectError, DeleteObjectOutput}, head_object::{HeadObjectError, HeadObjectOutput}, put_object::{PutObjectError, PutObjectOutput}}, primitives::ByteStream};
use core::str;
use std::{fmt::Error, io::Write, path::{Path, PathBuf}};

pub struct PathPair {
    pub oss_path: String,
    pub local_path: String,
}

impl PathPair {
    pub fn new(oss_path: String, local_path: String) -> PathPair{
        PathPair{
            oss_path: oss_path,
            local_path: local_path,
        }
    }
}

pub struct DirPathPair {
    pub oss_path: String,
    pub local_path: String,
}

impl DirPathPair {
    pub fn new(oss_dir_path: String, local_dir_path: String) -> DirPathPair{
        let mut oss_dir_path_checked = oss_dir_path.clone();
        let mut local_dir_path_checked = local_dir_path.clone();
        if oss_dir_path.ends_with("/")==false {
            oss_dir_path_checked = oss_dir_path + "/";
        }
        if local_dir_path.ends_with("/")==false {
            local_dir_path_checked = local_dir_path + "/";
        }
        DirPathPair{
            oss_path: oss_dir_path_checked,
            local_path: local_dir_path_checked,
        }
    }
}

pub struct OssConfig {
    pub access_key: String,
    pub secret_key: String,
    pub endpoint: String,
    pub region: String,
    pub bucket: String,
    pub client: aws_sdk_s3::Client,
}

impl OssConfig {
    pub fn new(access_key: String, secret_key: String, endpoint: String, region: String, bucket: String) -> OssConfig {
        let config = SdkConfig::builder()
            .credentials_provider(SharedCredentialsProvider::new(Credentials::new(
                &access_key,
                &secret_key,
                None,
                None,
                "Static",
            )))
            .endpoint_url(&endpoint)
            .region(Region::new(region.clone()))
            .build();
        let s3_config_builder = aws_sdk_s3::config::Builder::from(&config);
        let client = aws_sdk_s3::Client::from_conf(s3_config_builder.build());
        OssConfig {
            access_key,
            secret_key,
            endpoint,
            region,
            bucket,
            client,
        }
    }

    pub fn new_from_file(file_path: &str, config_name: &str) -> OssConfig {
        println!("Reading config file from: {}", file_path);
        let yaml_str = std::fs::read_to_string(file_path).unwrap();
        let config = serde_yaml::from_str::<serde_yaml::Value>(&yaml_str).unwrap();
        let config = config[config_name].clone();
        let access_key = config["access_key"].as_str().unwrap().to_string();
        let secret_access_key = config["secret_access_key"].as_str().unwrap().to_string();
        let endpoint_url = config["endpoint_url"].as_str().unwrap().to_string();
        let region = config["region"].as_str().unwrap().to_string();
        let bucket = config["bucket"].as_str().unwrap().to_string();
        OssConfig::new(access_key, secret_access_key, endpoint_url, region, bucket)
    }
}

pub struct OssClient {
    pub config: OssConfig,
}

impl OssClient {
    pub fn new(access_key: String, secret_key: String, endpoint: String, region: String, bucket: String) -> OssClient {
        OssClient {
            config: OssConfig::new(access_key, secret_key, endpoint, region, bucket),
        }
    }

    pub fn new_from_file(file_path: &str, config_name: &str) -> OssClient {
        OssClient {
            config: OssConfig::new_from_file(file_path, config_name),
        }
    }

    pub fn new_from_config(config: OssConfig) -> OssClient {
        OssClient {
            config: config,
        }
    }

    // 获取指定前缀的对象列表
    pub async fn list_objects(&self, prefix: &str) -> Vec<String>{
        // println!("Listing objects in bucket {}...", &self.config.bucket);
        // 容错检查
        let mut prefix_checked = prefix.to_owned();
        if prefix.ends_with("/")==false {
            prefix_checked = prefix.to_owned() + "/";
        }
        let mut response = self.config.client
            .list_objects_v2()
            .bucket(&self.config.bucket)
            .max_keys(100)
            .prefix(prefix_checked)
            .into_paginator()
            .send();

        let mut objects = Vec::new();
        while let Some(result) = response.next().await {
            match result {
                Ok(output) => {
                    // return output;
                    for object in output.contents() {
                        let object_key = object.key().unwrap_or("Unknown");
                        objects.push(object_key.to_string());
                        // println!("{}", object_key);
                    }
                }
                Err(err) => {
                    eprintln!("{err:?}");
                }
            }
        }
        objects
    }

    // 下载单个文件到本地
    pub async fn download_object(&self, path_pair: PathPair) -> Result<PathPair, Error>{
        // 检查本地路径并创建文件夹
        self.create_local_dir(path_pair.local_path.rsplitn(2, "/").last().unwrap()).unwrap();
        let mut object = self.config.client
            .get_object()
            .bucket(&self.config.bucket)
            .key(&path_pair.oss_path)
            .send()
            .await
            .unwrap();

        let mut file = std::fs::File::create(&path_pair.local_path).unwrap();
        while let Some(bytes) = object.body.try_next().await.unwrap() {
            file.write_all(&bytes).unwrap();
        }
        println!("Downloaded {} to {}", &path_pair.oss_path, &path_pair.local_path);
        Ok(path_pair)
    }

    // 下载oss文件夹内所有文件到本地
    pub async fn download_dir(&self, dir_path_pair: DirPathPair){
        let oss_files = self.list_objects(&dir_path_pair.oss_path).await;
        for oss_file in oss_files{
            let local_file = oss_file.replace(&dir_path_pair.oss_path, &dir_path_pair.local_path);
            let path_pair = PathPair::new(oss_file, local_file);
            let _ = self.download_object(path_pair).await;
        }
    }

    // 上传单个文件到OSS
    pub async fn upload_object(&self, path_pair: PathPair) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
        let body = ByteStream::from_path(path_pair.local_path).await;
        self.config.client
            .put_object()
            .bucket(&self.config.bucket)
            .key(path_pair.oss_path)
            .body(body.unwrap())
            .send()
            .await
    }

    // 上传文件夹到OSS
    pub async fn upload_dir(&self, path_pair: DirPathPair){
        let local_files = self.get_all_files_in_dir(Path::new(&path_pair.local_path));
        println!("{:?}", local_files);
        for local_file in local_files{
            // Path类型转String
            let local_file = local_file.to_string_lossy().to_string();
            let oss_file = local_file.replace(&path_pair.local_path, &path_pair.oss_path);
            let path_pair = PathPair::new(oss_file, local_file);
            println!("Uploading {} to {}", &path_pair.local_path, &path_pair.oss_path);
            let _ = self.upload_object(path_pair).await;
        }
    }

    // 删除OSS上的单个文件
    pub async fn delete_object(&self, key: &str) -> Result<DeleteObjectOutput, SdkError<DeleteObjectError>>{
        self.config.client
                .delete_object()
                .bucket(&self.config.bucket)
                .key(key)
                .send()
                .await
    }

    // 删除OSS上的文件夹
    pub async fn delete_dir(&self, key: &str){
        let oss_files = self.list_objects(key).await;
        for oss_file in oss_files{
            let _ = self.delete_object(&oss_file).await;
            println!("Deleted bucket {} file: {}", &self.config.bucket, &oss_file);
        }
        println!("Finshed delete files in bucket: {}, path: {}", &self.config.bucket, key)
    }

    // OSS文件检查
    pub async fn check_object(&self, key: &str)-> Result<HeadObjectOutput, SdkError<HeadObjectError>> {
        self.config.client
            .head_object()
            .bucket(&self.config.bucket)
            .key(key)
            .send()
            .await
    }

    // 本地文件检查
    pub fn check_local_file(&self, path: &str) -> bool {
        Path::new(path).exists()
    }

    // 根据本地路径创建本地文件夹
    pub fn create_local_dir(&self, path: &str) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(path)
    }

    // 获取文件夹中所有文件的路径
    pub fn get_all_files_in_dir(&self, path: &Path) -> Vec<PathBuf> {
        if path.is_dir(){
            let mut files = Vec::new();
            for entry in std::fs::read_dir(path).unwrap() {
                let entry = entry.unwrap();
                files.append(&mut self.get_all_files_in_dir(&entry.path()))
            }
            files
        }
        else {
            vec![path.to_path_buf()]
        }
    }
}