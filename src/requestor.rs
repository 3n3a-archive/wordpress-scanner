use reqwest;

pub async fn get_site() -> Result<String, Box<dyn std::error::Error>> {
    let result = reqwest::get("https://ifconfig.me/all").await?;
    let text = result.text().await?;
    println!("{:#?}", text);
    Ok(text)
}
