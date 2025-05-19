#[cfg(feature = "storage_s3")]
use abs_admin::service::{FileS3Service, IStorageService, S3Config};

#[cfg(feature = "storage_s3")]
#[tokio::test]
async fn test_config_load() {
    let cfg = S3Config {
        bucket: "test".to_string(),
        endpoint: "http://127.0.0.1:9000".to_string(),
        access_key: "minioadmin".to_string(),
        secret_key: "minioadmin".to_string(),
        region: "".to_string(),
    };
    println!("source={}", serde_json::to_string(&cfg).unwrap());
    let config = S3Config::load(r#"s3://{"bucket":"test","endpoint":"http://127.0.0.1:9000","access_key":"minioadmin","secret_key":"minioadmin","region":""}"#).unwrap();
    assert_eq!(cfg, config);
}

#[cfg(feature = "storage_s3")]
#[tokio::test]
async fn test_oss_upload() {
    let server = FileS3Service::new(S3Config {
        bucket: "test".to_string(),
        endpoint: "http://127.0.0.1:9000".to_string(),
        access_key: "minioadmin".to_string(),
        secret_key: "minioadmin".to_string(),
        region: "".to_string(),
    });
    let data = server.upload("/test.txt".to_string(), "test2222".into()).await.unwrap();
    println!("{:?}", data);
}

#[cfg(feature = "storage_s3")]
#[tokio::test]
async fn test_oss_list() {
    let server = FileS3Service::new( S3Config {
        bucket: "test".to_string(),
        endpoint: "http://127.0.0.1:9000".to_string(),
        access_key: "minioadmin".to_string(),
        secret_key: "minioadmin".to_string(),
        region: "".to_string(),
    });
    let data = server.list("/".to_string()).await.unwrap();
    println!("{:?}", data);
}