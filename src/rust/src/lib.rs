use extendr_api::prelude::*;
use extendr_polars as ep;
use polars::prelude as pl;

/// export rpolars DataFrame from rust to rpolars
/// @return r-polars DataFrame
/// @details this function requires r-polars polars package >= 0.8.0.
/// @export
#[extendr]
fn export_as_rpolars_df() -> Result<Robj> {
    use pl::*;
    let df = df! [
        "names" => ["a", "b", "c"],
        "values" => [1, 2, 6],
        "values_nulls" => [Some(1), None, Some(3)]
    ]
    .map_err(|err| extendr_api::Error::Other(err.to_string()))?;
    ep::export_dataframe::to_rpolars_dataframe(df)
}

/// import rpolars DataFrame to rust and print back to R terminal
/// @param rdf r-polars DataFrame
/// @return NULL
/// @details this function requires r-polars polars package >= 0.8.0
/// @export
#[extendr]
fn import_rpolars_df_and_print(rdf: Robj) -> Result<()> {
    let df: pl::DataFrame =
        ep::import_dataframe::rpolars_to_rust_dataframe(rdf).map_err(|err| err.to_string())?;
    rprintln!("this df was imported to rust side:\n{}", df);
    Ok(())
}

/// import rpolars DataFrame to rust and export again
/// @param rdf rpolars dataframe
/// @return rpolars dataframe
/// @details this function requires r-polars polars package >= 0.8.0
/// @export
#[extendr]
fn test_round_trip(rdf: Robj) -> Result<Robj> {
    let df: pl::DataFrame =
        ep::import_dataframe::rpolars_to_rust_dataframe(rdf).map_err(|err| err.to_string())?;
    ep::export_dataframe::to_rpolars_dataframe(df)
}


// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod extendrpolarsexamples;
    fn export_as_rpolars_df;
    fn import_rpolars_df_and_print;
    fn test_round_trip;
}
