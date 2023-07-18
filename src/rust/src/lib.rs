use extendr_api::prelude::*;
use extendr_polars as ep;
use polars::prelude as pl;






/// show case zero copy export from rust-polars to r-polars
/// @details conversion via arrow stream such to ensure version compatability
/// @export
#[extendr]
fn make_df() -> Result<Robj> {
    use pl::*;
    let df = df! [
        "names" => ["a", "b", "c"],
        "values" => [1, 2, 6],
        "values_nulls" => [Some(1), None, Some(3)]
    ].map_err(|err| {
        extendr_api::Error::Other(err.to_string())
    })?;
    let wdf = ep::WrapDataFrame(df);
    let robj = wdf.make_dataframe()?;
    
    Ok(robj)
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod extendrpolarsexamples;
    fn make_df;
}
