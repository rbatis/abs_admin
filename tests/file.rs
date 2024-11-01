use abs_admin::service::{FileServiceOss, IFileService, S3Config};


#[tokio::test]
async fn test_oss_upload() {
    let server = FileServiceOss::new("", S3Config{
        bucket: "test".to_string(),
        endpoint_url: "http://127.0.0.1:9000".to_string(),
        access_key: "minioadmin".to_string(),
        secret_key: "minioadmin".to_string(),
    });
    let data = server.upload("/test.txt".to_string(),"test2222".into()).await.unwrap();
    println!("{:?}", data);
}

#[tokio::test]
async fn test_oss_list() {
    let server = FileServiceOss::new("", S3Config{
        bucket: "test".to_string(),
        endpoint_url: "http://127.0.0.1:9000".to_string(),
        access_key: "minioadmin".to_string(),
        secret_key: "minioadmin".to_string(),
    });
    let data = server.list("/".to_string()).await.unwrap();
    println!("{:?}", data);
}