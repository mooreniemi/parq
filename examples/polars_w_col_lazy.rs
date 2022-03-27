use polars::prelude::*;
use std::env;
use std::time::SystemTime;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let start = SystemTime::now();
    let lf = LazyFrame::scan_parquet(file_path.to_string(), ScanArgsParquet::default())
        .expect("polars read the file");

    // not sure how you'd find this type easily from apply documentation
    let o = GetOutput::from_type(DataType::UInt32);
    // this avoids mutating two, and creates new column len
    let lf = lf.with_column(col("two").alias("len").apply(str_to_len, o));

    // actually do the computation and give us something we can inspect
    let df = lf.collect().expect("eager");
    println!("took: {} ms", start.elapsed().unwrap().as_millis());
    dbg!(&df);

    Ok(())
}

/// the str_val is the input column
fn str_to_len(str_val: Series) -> Result<Series> {
    let x = str_val
        .utf8()
        .unwrap()
        .into_iter()
        // your actual custom function would be in this map
        .map(|opt_name: Option<&str>| opt_name.map(|name: &str| name.len() as u32))
        .collect::<UInt32Chunked>();
    Ok(x.into_series())
}
