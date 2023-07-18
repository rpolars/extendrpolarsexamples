# TEMPLATE: extend polars (the R package) with rust


## Installation

Install R package polars e.g. from binary. Also needed for end user
```R
  install.packages("polars", repos = "https://rpolars.r-universe.dev")
```

Install rextendr R package to assist write R bindings for your rust project.
See extendr github page or visit the discord to ask questions.
```R
  install.packages("rextendr")
```
s

### System dependencies

To install the full version of rust-polars you will need rust nightly e.g. this version. 
Stable is also possible, check with rust-polars docs.

- Install [`rustup`](https://rustup.rs/), the cross-platform Rust
  installer. Then:

  ``` sh
  rustup toolchain install nightly-2023-05-07
  rustup default nightly-2023-05-07
  ```

- Windows: Make sure the latest version of
  [Rtools](https://cran.r-project.org/bin/windows/Rtools/) is installed
  and on your PATH.

- MacOS: Make sure [`Xcode`](https://developer.apple.com/support/xcode/)
  is installed.

- Install [CMake](https://cmake.org/) and add it to your PATH.


## Build templates
Once Rust is working, you can build this 
```r
remotes::install_github("rpolars/extendrpolarsexamples")
```

After installation, the following should work:
```r
library(extendrpolarsexamples)

# zero-copy export a rust-polars DataFrame to valid r-polars DataFrame
extendrpolarsexamples::make_df()
┌───────┬────────┬──────────────┐
│ names ┆ values ┆ values_nulls │
│ ---   ┆ ---    ┆ ---          │
│ str   ┆ i32    ┆ i32          │
╞═══════╪════════╪══════════════╡
│ a     ┆ 1      ┆ 1            │
│ b     ┆ 2      ┆ null         │
│ c     ┆ 6      ┆ 3            │
└───────┴────────┴──────────────┘
```

## Development

 - fork this template rpolars/extendrpolarsexamples
 - clone your fork
 
### Generate wrappers

When you make either of the following changes to the Rust source code, you'll need to regenerate the wrappers.

* add a new function
* modify the signature of an existing function
* modify the documentation written on Rust code (on the lines starting with `///`)

This can be done by:

``` r
  rextendr::document()
```

Which will compile the Rust code as well as updating documentation.

## More rextendr help

For a fully worked out demonstration of how to create a Rust + R library see [here](https://extendr.github.io/rextendr/articles/package.html).


