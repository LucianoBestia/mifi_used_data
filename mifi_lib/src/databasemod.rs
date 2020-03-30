use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct DataUsed {
    /// how many hours passed from the start of recording
    hour: u32,
    ul: u32,
    dl: u32,
}

/// if the does not exist it will create it and the tables
pub fn create_db() -> Result<()> {
    let conn = Connection::open("data.db")?;

    conn.execute(
        "create table if not exists data_used (
             hour integer primary key,
             ul integer not null,
             dl integer not null
         )",
        NO_PARAMS,
    )?;

    Ok(())
}

pub fn insert(hour: u32, ul: u32, dl: u32) -> Result<()> {
    let conn = Connection::open("data.db")?;
    conn.execute(
        "INSERT INTO data_used (hour,ul,dl) values (?1,?2,?3)",
        &[hour, ul, dl],
    )?;
    Ok(())
}

pub fn select() -> Result<()> {
    let conn = Connection::open("data.db")?;
    let mut stmt = conn.prepare("SELECT c.hours, c.ul, c.dl from data_used c;")?;

    let d_u_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok(DataUsed {
            hour: row.get(0)?,
            ul: row.get(1)?,
            dl: row.get(2)?,
        })
    })?;

    for d_u in d_u_iter {
        println!("Select {:?}", d_u);
    }
    Ok(())
}
