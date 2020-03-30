//! mifi_lib

use mifi_lib::databasemod;
use mifi_lib::datetimemod;
use mifi_lib::findinhtmlmod;

/// the main just calls try_main and shows errors if there are any
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    try_main().await.unwrap();

    Ok(())
}

/// the errors are mostly propagated to the try_main
async fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    let resp_str = reqwest::get("http://192.168.225.1/cgi-bin/en-jio/mStatus.html")
        .await?
        .text()
        .await?;

    // find
    let (sentp, cursor_pos) =
        findinhtmlmod::find_between(&resp_str, r#"<label id="lsentPackets">"#, "</label>", 0);
    println!("{:#?}", sentp);
    let i_sentp = findinhtmlmod::parse_mb(sentp);
    println!("{:#?}", i_sentp);
    let (recp, _) = findinhtmlmod::find_between(
        &resp_str,
        r#"<label id="lRecPackets">"#,
        "</label>",
        cursor_pos,
    );
    println!("{:#?}", recp);
    let i_recp = findinhtmlmod::parse_mb(recp);
    println!("{:#?}", i_recp);

    databasemod::create_db()?;
    databasemod::create_db2()?;
    let minutes = datetimemod::elapsed_minutes_from_2020();
    databasemod::insert(minutes, i_sentp, i_recp)?;
    databasemod::select()?;
    databasemod::calculate_graph()?;
    databasemod::select_graph()?;

    Ok(())
}
