//! mifi_lib

use mifi_lib::databasemod;
use mifi_lib::datetimemod;
use mifi_lib::findinhtmlmod;
use mifi_lib::requestmod;

/// the main just calls try_main and shows errors if there are any
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("mifi_used_data start");
    loop {
        if datetimemod::is_scheduled_run() {
            println!(
                "
            Executed every on 14,29,44,59 minute every hour."
            );
            try_main().await.unwrap();
        }
        // I need the tick resolution once per minute
        std::thread::sleep(std::time::Duration::from_millis(
            datetimemod::millis_until_next_minute(),
        ));
        if false {
            break;
        }
    }

    Ok(())
}

/// the errors are mostly propagated to the try_main
async fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    let resp_str = requestmod::request().await?;

    let (i_sentp, i_recp) = findinhtmlmod::find(&resp_str);

    databasemod::create_db()?;
    let minutes = datetimemod::elapsed_minutes_from_2020();
    databasemod::insert_data_used(minutes, i_sentp, i_recp)?;
    //databasemod::select()?;
    databasemod::calculate_graph()?;
    databasemod::select_graph()?;

    Ok(())
}
