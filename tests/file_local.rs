use abs_admin::service::{FileLocalService, IStorageService};

#[tokio::test]
async fn test_oss_upload() {
    let server = FileLocalService::new();
    let data = server.upload("test.txt".to_string(), "test2222".into()).await.unwrap();
    println!("{:?}", data);
}

#[tokio::test]
async fn test_oss_list() {
    let server = FileLocalService::new();
    let data = server.list("target/".to_string()).await.unwrap();
    println!("{:?}", data);
}