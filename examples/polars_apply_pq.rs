use polars::prelude::*;
use std::env;
use std::time::SystemTime;

use arrow2::error::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let start = SystemTime::now();
    let lf = LazyFrame::scan_parquet(file_path.to_string(), ScanArgsParquet::default())
        .expect("polars read the file");
    let mut df = lf.collect().expect("eager");
    dbg!(&df);

    // mutates in place, thus why no sep dest name
    df.apply("two", str_to_len).expect("applied");
    println!("took: {} ms", start.elapsed().unwrap().as_millis());
    dbg!(&df);

    Ok(())
}

fn str_to_len(str_val: &Series) -> Series {
    str_val
        .utf8()
        .unwrap()
        .into_iter()
        .map(|opt_name: Option<&str>| opt_name.map(|name: &str| name.len() as u32))
        .collect::<UInt32Chunked>()
        .into_series()
}
