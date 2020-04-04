//! requestmod

use reqwest;

/// get String from request
pub async fn request() -> Result<String, Box<dyn std::error::Error>> {
    let resp_str = reqwest::get("http://192.168.225.1/cgi-bin/en-jio/mStatus.html")
        .await?
        .text()
        .await?;
    //return
    Ok(resp_str)
}
