use reqwest;

pub async fn get_site(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let result = client.get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36")
        .send()
        .await?;
    let text = result.text().await?;
    println!("{:#?}", text);
    Ok(text)
}
