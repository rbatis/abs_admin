#[cfg(test)]
mod test {
    #[async_std::test]
    pub async fn test_client() {
        let uri = "https://www.baidu.com";
        let string: String = surf::get(uri).recv_string().await.unwrap();
        println!("{}", string);
    }
}