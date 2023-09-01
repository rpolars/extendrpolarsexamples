test_that("export rust-polars data-frame to r-polars", {
  skip_if_not_installed("polars")
  rdf_actual = export_as_rpolars_df()
  
  rdf_expected = polars::pl$DataFrame(
    names = c("a", "b", "c"),
    values = c(1L, 2L, 6L),
    values_nulls = c(1L,NA,3L)
  )
  
  expect_identical(
    rdf_actual$to_list(),
    rdf_expected$to_list()
  )
  
})

test_that("import rpolars dataframe to rust and print", {
  skip_if_not_installed("polars")
  
  rdf_expected = polars::pl$DataFrame(
    names = c("a", "b", "c"),
    values = c(1L, 2L, 6L),
    values_nulls = c(1L,NA,3L)
  )
  
  lines = capture.output(invisible(import_rpolars_df_and_print(rdf_expected)))
  expected_lines = capture.output(rdf_expected)
  
  expect_identical(lines[1],"this df was imported to rust side:")
  expect_identical(lines[-1],expected_lines)
  
})


test_that("import rpolars dataframe to rust and print", {
  skip_if_not_installed("polars")
  
  rdf_expected = polars::pl$DataFrame(
    names = c("a", "b", "c"),
    values = c(1L, 2L, 6L),
    values_nulls = c(1L,NA,3L)
  )
  
  lines = capture.output(invisible(import_rpolars_df_and_print(rdf_expected)))
  expected_lines = capture.output(rdf_expected)
  
  expect_identical(lines[1],"this df was imported to rust side:")
  
  #this will not hold if two polars versions print in different styles
  #expect_identical(lines[-1],expected_lines)
  
})


test_that("round trip conversion", {
  skip_if_not_installed("polars")
  
  rdf_expected = polars::pl$DataFrame(
    names = c("a", "b", "c"),
    values = c(1L, 2L, 6L),
    values_nulls = c(1L,NA,3L)
  )
  
  rdf_actual = test_round_trip(rdf_expected)
  expect_identical(rdf_actual$to_list(), rdf_expected$to_list())
  
})
