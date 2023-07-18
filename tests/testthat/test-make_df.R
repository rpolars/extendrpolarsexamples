test_that("export rust-polars data-frame to r-polars", {
  skip_if_not_installed("polars")
  df = make_df()
  expect_snapshot(df)
})
