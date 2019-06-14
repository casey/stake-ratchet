mod color;
mod common;
mod mode;
mod opt;

use crate::common::*;

fn main() {
  Opt::from_args().run();
}
