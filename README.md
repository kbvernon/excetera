

<!-- README.md is generated from README.qmd. Please edit that file -->

# excetera

<!-- badges: start -->

<!-- badges: end -->

The goal of `{excetera}` is to provide an extendr-powered R wrapper
around the `tera` templating-engine in Rust.

## Installation

You can install the development version of excetera like so:

``` r
# install.packages("pak")
pak::pak("kbvernon/excetera")
```

## Example

This is a basic example which shows you how to solve a common problem:

``` r
library(excetera)

tdir <- file.path(tempdir(), "tera")

dir.create(tdir)

cat(
  "<p>Hello {{ x }}. This is {{ y }}.</p>",
  file = file.path(tdir, "hello-world.html")
)

cat(
  r"(<img class="{{ img_class }}" src="{{ img_source }}">)",
  file = file.path(tdir, "image-paths.html")
)

tera <- ExTera$new(file.path(tdir, "*"))

tera$list_templates()
#> [1] "image-paths.html" "hello-world.html"

tera$render_to_string(
  "hello-world.html",
  context = list(x = "world", y = "Tera")
)
#> [1] "<p>Hello world. This is Tera.</p>"

tera$render_to_string(
  "image-paths.html",
  context = list(img_class = "tera-img", img_source = "path-to-img.png")
)
#> [1] "<img class=\"tera-img\" src=\"path-to-img.png\">"
```
