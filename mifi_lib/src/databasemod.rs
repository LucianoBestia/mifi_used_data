use crate::*;

use rusqlite::NO_PARAMS;
use rusqlite::{params, Connection, Result};

#[derive(Debug, Clone)]
pub struct DataUsed {
    /// how many minutes passed from the start of recording
    elapsed_minutes: u32,
    ul: u32,
    dl: u32,
}
#[derive(Debug, Clone)]
pub struct DataForGraph {
    /// every 5 minutes or so
    elapsed_minutes: u32,
    ul: u32,
    dl: u32,
}

/// if the does not exist it will create it and the tables
pub fn create_db() -> Result<()> {
    println!("create_db");
    let conn = Connection::open("data.db")?;

    conn.execute(
        "create table if not exists data_used (
            elapsed_minutes integer primary key,
             ul integer not null,
             dl integer not null
         )",
        NO_PARAMS,
    )?;

    Ok(())
}

pub fn create_db2() -> Result<()> {
    println!("create_db");
    let conn = Connection::open("data.db")?;

    conn.execute(
        "create table if not exists data_for_graph (
            elapsed_minutes integer primary key,
             ul integer not null,
             dl integer not null
         )",
        NO_PARAMS,
    )?;

    Ok(())
}

pub fn insert(elapsed_minutes: u32, ul: u32, dl: u32) -> Result<()> {
    println!("insert");
    let conn = Connection::open("data.db")?;
    //elapsed_minutes can get primary key error. let just ignore that
    let x = conn.execute(
        "INSERT INTO data_used (elapsed_minutes,ul,dl) values (?1,?2,?3)",
        &[elapsed_minutes, ul, dl],
    );
    if let Err(e) = x {
        println!("sql error: {}", e);
    }

    Ok(())
}

pub fn select() -> Result<()> {
    println!("select");
    let conn = Connection::open("data.db")?;
    let mut stmt = conn.prepare("SELECT c.elapsed_minutes, c.ul, c.dl from data_used c;")?;

    let d_u_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok(DataUsed {
            elapsed_minutes: row.get(0)?,
            ul: row.get(1)?,
            dl: row.get(2)?,
        })
    })?;

    for d_result in d_u_iter {
        println!("Select {:?}", d_result);
    }
    Ok(())
}

/// calculate table for graph
pub fn calculate_graph() -> Result<()> {
    println!("calculate_graph");
    let conn = Connection::open("data.db")?;
    let mut stmt = conn.prepare("SELECT c.elapsed_minutes, c.ul, c.dl from data_used c;")?;
    let d_u_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok(DataUsed {
            elapsed_minutes: row.get(0)?,
            ul: row.get(1)?,
            dl: row.get(2)?,
        })
    })?;
    // prev for previous values
    let mut prev_opt: Option<DataUsed> = None;
    let mut x2 = 0;
    let interval = 15;
    let mut prev_cumul_graph: (u32, u32) = (0, 0);
    let mut vec =  Vec::<DataForGraph>::new();
    for d_result in d_u_iter {
        if let Ok(d) = &d_result {
            if let Some(p) = &prev_opt {
                let x1 = p.elapsed_minutes;
                let x3 = d.elapsed_minutes;
                let y1u = p.ul;
                let y3u = d.ul;
                let y1d = p.dl;
                let y3d = d.dl;

                // if x2 is between x1 and x3
                while x2 < x3 {
                    //this is the cumulative numbers
                    let y2u = y1u + ((x2 - x1) * (y3u - y1u) / (x3 - x1));
                    let y2d = y1d + ((x2 - x1) * (y3d - y1d) / (x3 - x1));
                    // calculate the difference
                    let y2ud = y2u - prev_cumul_graph.0;
                    let y2dd = y2d - prev_cumul_graph.1;
                    //println!(" {} {} {}", datetimemod::elapsed_to_string(x2), y2ud, y2dd);
                    // single insert commands are far too slow because sqlite makes a transaction around every and each
                    // i will prepare a vector and then insert the whole vector as one transaction
                    vec.push(DataForGraph{
                        elapsed_minutes: x2,
                        ul: y2ud,
                        dl: y2dd,
                    });

                    // increment only if x2 is between x1 and x3
                    x2 += interval;
                    prev_cumul_graph = (y2u, y2d);
                }
                if x2 >= x3 {
                    prev_opt = Some(d.clone());
                }
            } else {
                //first loop, prev_opt is none
                x2 = d.elapsed_minutes;
                let y2u = d.ul;
                let y2d = d.dl;
                println!(" {} {} {}", datetimemod::elapsed_to_string(x2), y2u, y2d);
                x2 = d.elapsed_minutes + interval;
                prev_cumul_graph = (y2u, y2d);
                prev_opt = Some(d.clone());
            }
        }
    }
    insert_for_graph(vec);

    Ok(())
}
/// inserting rows one by one is super slow because of each transaction
/// bulk insert with one transaction is super fast
pub fn insert_for_graph(vec:Vec<DataForGraph>) -> Result<()> {
    println!("insert_for_graph start");
    let mut conn = Connection::open("data.db")?;    
    conn.execute("delete from data_for_graph", params![])?;
    let tr = conn.transaction()?;
    {
        for dfg in vec{
            let _x = tr.execute(
                "INSERT INTO data_for_graph (elapsed_minutes,ul,dl) values (?1,?2,?3)",
                &[dfg.elapsed_minutes, dfg.ul, dfg.dl],
            );
        }
    }
    tr.commit();
    println!("insert_for_graph end");
    Ok(())
}

pub fn select_graph() -> Result<()> {
    println!("select graph");
    let conn = Connection::open("data.db")?;
    let mut stmt = conn.prepare("SELECT c.elapsed_minutes, c.ul, c.dl from data_for_graph c;")?;

    let d_u_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok(DataForGraph {
            elapsed_minutes: row.get(0)?,
            ul: row.get(1)?,
            dl: row.get(2)?,
        })
    })?;

    for d_result in d_u_iter {
        println!("Select {:?}", d_result);
    }
    Ok(())
}
