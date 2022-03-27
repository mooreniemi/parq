# parq

Examples of using Parquet, Arrow, Polars, or other dataframe readers in
Rust. `main.rs` is a "sandbox" and iterations are moved into `examples/`.

There's really only one "live" (updated recently) project under [dataframe
keyword](https://crates.io/keywords/dataframe), which is Polars. Peroxide
has a dataframe implementation but only csv and
[netcdf](https://www.unidata.ucar.edu/software/netcdf/) support, no
Parquet/Arrow reader.

The other major live project right now with Arrow support is
[DataFusion](https://github.com/apache/arrow-datafusion), which has
a similar feeling to Spark with a `ctx` object. However, this project
seems to be aimed very much at SQL querying, with `DataFrame` lagging
behind the Polars API. It doesn't appear to be possible to do the simple
"apply a udf on a column to create a new column" yet. So although
a distributed project
[Ballista](https://github.com/apache/arrow-datafusion/tree/master/ballista)
extends this to multiple machines, it's not a replacement for Spark for
processing (in contrast to querying) as yet.

## how to run

```
alex-XPS-13-9350:~/git/parq/ ⌂ cargo run --example polars_w_col_lazy data/example.parquet
   Compiling parq v0.1.0 (/home/alex/git/parq)
    Finished dev [unoptimized + debuginfo] target(s) in 23.65s
     Running `target/debug/examples/polars_w_col_lazy data/example.parquet`
took: 4 ms
[examples/polars_w_col_lazy.rs:21] &df = shape: (3, 4)
┌──────┬─────┬───────────────────┬─────┐
│ one  ┆ two ┆ __index_level_0__ ┆ len │
│ ---  ┆ --- ┆ ---               ┆ --- │
│ f64  ┆ str ┆ str               ┆ u32 │
╞══════╪═════╪═══════════════════╪═════╡
│ -1.0 ┆ f   ┆ a                 ┆ 1   │
├╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┤
│ null ┆ b   ┆ b                 ┆ 1   │
├╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┤
│ 2.4  ┆ d   ┆ c                 ┆ 1   │
└──────┴─────┴───────────────────┴─────┘
```

## associated links

- https://able.bio/haixuanTao/data-manipulation-polars-vs-rust--3def44c8#applying-function-in-polars
- https://stackoverflow.com/questions/70386839/writing-expression-in-polars-lazy-in-rust/71638091#71638091
- https://stackoverflow.com/questions/70959170/is-there-a-way-to-apply-a-udf-function-returning-multiple-values-in-rust-polars
- https://arrow.apache.org/docs/python/parquet.html
- https://github.com/apache/arrow-datafusion/issues/2101
