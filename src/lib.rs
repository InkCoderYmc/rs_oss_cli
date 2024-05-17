use aws_config::{SdkConfig,Region};
use aws_sdk_s3::{config::{Credentials,SharedCredentialsProvider}, error::SdkError, operation::{delete_object::{DeleteObjectError, DeleteObjectOutput}, head_object::{HeadObjectError, HeadObjectOutput}, put_object::{PutObjectError, PutObjectOutput}}, primitives::ByteStream};
use core::str;
// use tokio::time::error::Error;
use std::{f32::consts::E, fmt::Error, fs::File, io::Write, path::{Path,PathBuf}, process::Output};
// use aws_sdk_s3::types::Error as S3Error;

pub struct PathPair {
    pub oss_path: String,
    pub local_path: String,
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

    // 获取指定前缀的对象列表
    pub async fn list_objects(&self, prefix: &str) -> Vec<String>{
        println!("Listing objects in bucket {}...", &self.config.bucket);
        let mut response = self.config.client
            .list_objects_v2()
            .bucket(&self.config.bucket)
            .max_keys(100)
            .prefix(prefix)
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

    // 删除OSS上的单个文件
    pub async fn delete_object(&self, key: &str) -> Result<DeleteObjectOutput, SdkError<DeleteObjectError>>{
        self.config.client
                .delete_object()
                .bucket(&self.config.bucket)
                .key(key)
                .send()
                .await
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
    pub fn get_all_files_in_dir(&self, path: &str) -> Vec<PathBuf> {
        let mut files = Vec::new();
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }
        files
    }
}