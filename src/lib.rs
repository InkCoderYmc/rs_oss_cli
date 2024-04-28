use aws_config::{SdkConfig,Region};
use aws_sdk_s3::config::{Credentials,SharedCredentialsProvider};
// use aws_sdk_s3::types::Error;

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

    pub async fn list_objects(&self, prefix: &str){
        println!("Listing objects in bucket {}...", &self.bucket);
        let mut response = self.client
            .list_objects_v2()
            .bucket(&self.bucket)
            .max_keys(100)
            .prefix(prefix)
            .into_paginator()
            .send();

        while let Some(result) = response.next().await {
            match result {
                Ok(output) => {
                    for object in output.contents() {
                        println!("{}", object.key().unwrap_or("Unknown"));
                    }
                }
                Err(err) => {
                    eprintln!("{err:?}")
                }
            }
        }
    }

    // pub async fn download_object(&self, key: &str, local_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    //     let data: GetObjectOutput = s3_service::download_object(&self.client, &self.bucket, &key).await?;
    //     let data_length: u64 = data
    //         .content_length()
    //         .unwrap_or_default()
    //         .try_into()
    //         .unwrap();
    //     if file.metadata().unwrap().len() == data_length {
    //         println!("Data lengths match.");
    //     } else {
    //         println!("The data was not the same size!");
    //     }
    // }
}
