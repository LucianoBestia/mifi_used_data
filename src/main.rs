//! mifi - track usage

mod findinhtml;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp_str = reqwest::get("http://192.168.225.1/cgi-bin/en-jio/mStatus.html")
        .await?
        .text()
        .await?;

    // find
    let (sentp, cursor_pos) =
        findinhtml::find_between(&resp_str, r#"<label id="lsentPackets">"#, "</label>", 0);
    println!("{:#?}", sentp);
    let i_sentp = findinhtml::parse_mb(sentp);
    println!("{:#?}", i_sentp);
    let (recp, _) = findinhtml::find_between(
        &resp_str,
        r#"<label id="lRecPackets">"#,
        "</label>",
        cursor_pos,
    );
    println!("{:#?}", recp);
    let i_recp = findinhtml::parse_mb(recp);
    println!("{:#?}", i_recp);

    Ok(())
}
