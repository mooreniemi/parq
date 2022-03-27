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

    // the fn we use to make the column names it too
    df.with_column(str_to_len(df.column("two").expect("has two")))
        .expect("with_column");
    println!("took: {} ms", start.elapsed().unwrap().as_millis());
    dbg!(&df);

    Ok(())
}

/// the str_val is the input column
fn str_to_len(str_val: &Series) -> Series {
    let mut x = str_val
        .utf8()
        .unwrap()
        .into_iter()
        .map(|opt_name: Option<&str>| opt_name.map(|name: &str| name.len() as u32))
        .collect::<UInt32Chunked>();
    // NB. this is naming the chunked array, before we even get to a series
    x.rename("len");
    x.into_series()
}
