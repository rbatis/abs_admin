#[cfg(test)]
mod test {

    #[async_std::test]
    pub async fn test_client() {
        let resp = reqwest::get("http://www.baidu.com")
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        println!("{:#?}", String::from_utf8(resp.to_vec()));
    }
}
