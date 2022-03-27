use std::env;
use std::sync::Arc;
use std::time::SystemTime;

use datafusion::arrow::array::ArrayRef;
use datafusion::arrow::datatypes::DataType;
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::logical_plan::create_udf;
use datafusion::physical_plan::functions::{make_scalar_function, Volatility};
use datafusion::prelude::ExecutionContext;

#[tokio::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let start = SystemTime::now();
    let mut ctx = ExecutionContext::new();

    // https://github.com/apache/arrow-datafusion/blob/04da6a6d9ff78e5cf4b7fd1a4b602d7d5a01e6a8/datafusion/src/execution/context.rs#L2708
    let myfunc = |args: &[ArrayRef]| Ok(Arc::clone(&args[0]));
    let myfunc = make_scalar_function(myfunc);

    let udf = create_udf(
        "MY_FUNC",
        vec![DataType::Int32],
        Arc::new(DataType::Int32),
        Volatility::Immutable,
        myfunc,
    );
    ctx.register_udf(udf);

    // create the dataframe
    let df = ctx.read_parquet(file_path).await.expect("parquet ok");
    // no debug impl, instead have to await on show
    // dbg!(&df);
    df.show().await.expect("can show df");

    // execute any plan (none above yet)
    // https://github.com/apache/arrow-datafusion/issues/2101
    let results: Vec<RecordBatch> = df.collect().await.expect("collect ok");

    println!("took: {} ms", start.elapsed().unwrap().as_millis());
    dbg!(&results);

    Ok(())
}
