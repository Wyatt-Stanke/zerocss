#![deny(clippy::all)]

use zerocss_core::zerocss;

#[macro_use]
extern crate napi_derive;

#[napi]
fn compile_CSS() {
  zerocss::compile();
}
