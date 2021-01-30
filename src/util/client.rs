#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[async_std::test]
    pub async fn test_client() {
        let resp = reqwest::get("https://www.baidu.com")
            .await.unwrap()
            .json::<HashMap<String, String>>()
            .await.unwrap();
        println!("{:#?}", resp);
    }
}