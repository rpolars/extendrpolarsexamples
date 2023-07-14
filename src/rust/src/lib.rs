use extendr_api::prelude::*;
use extendr_polars as ep;
use polars::prelude as pl;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> Robj {
    use pl::*;
    let df = df! [
        "names" => ["a", "b", "c"],
        "values" => [1, 2, 3],
        "values_nulls" => [Some(1), None, Some(3)]
    ]
    .unwrap();
    let wdf = ep::WrapDataFrame(df);

    let robj = wdf.export_stream().unwrap();

    dbg!(wdf);
    robj
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    fn hello_world;
}
