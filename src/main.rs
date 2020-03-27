//! mifi - track usage

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp_str = reqwest::get("http://192.168.225.1/cgi-bin/en-jio/mStatus.html")
        .await?
        .text()
        .await?;

    // find
    let (sentp, cursor_pos) =
        find_between(&resp_str, r#"<label id="lsentPackets">"#, "</label>", 0);
    println!("{:#?}", sentp);
    let i_sentp = parse_mb(sentp);
    println!("{:#?}", i_sentp);
    let (recp, _) = find_between(
        &resp_str,
        r#"<label id="lRecPackets">"#,
        "</label>",
        cursor_pos,
    );
    println!("{:#?}", recp);
    let i_recp = parse_mb(recp);
    println!("{:#?}", i_recp);
    Ok(())
}

pub fn parse_mb(source_str: &str) -> f64 {
    let mb: f64;
    if source_str.contains("MB") {
        mb = source_str[..source_str.len() - 3].parse::<f64>().unwrap();
    } else if source_str.contains("GB") {
        mb = source_str[..source_str.len() - 3].parse::<f64>().unwrap() * 1000.0;
    } else {
        mb = 0.0;
    }
    //return
    mb
}

/// find and return the first occurrence between start and end delimiters
/// returns also the cursor position, so the next find will continue there
/// returns empty string if no found
pub fn find_between<'a>(
    source_str: &'a str,
    start_delimiter: &str,
    end_delimiter: &str,
    start_cursor_pos: usize,
) -> (&'a str, usize) {
    let mut sentp = "";
    let mut cursor_pos = start_cursor_pos;
    if let Some(pos1) = source_str[start_cursor_pos..].find(start_delimiter) {
        cursor_pos += pos1;
        if let Some(pos2) = source_str[start_cursor_pos + pos1..].find(end_delimiter) {
            cursor_pos += pos2 + end_delimiter.len();
            sentp = &source_str
                [start_cursor_pos + pos1 + start_delimiter.len()..start_cursor_pos + pos1 + pos2];
        }
    }
    //return
    (sentp, cursor_pos)
}
