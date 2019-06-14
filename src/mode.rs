use crate::common::*;

#[derive(Copy, Clone, StructOpt)]
pub enum Mode {
  #[structopt(name = "identity")]
  Identity,
  #[structopt(name = "damper")]
  Damper {
    decentralization: f64,
    centralization: f64,
  },
}

impl Mode {
  pub fn modify_delta(self, delta: f64) -> f64 {
    match self {
      Mode::Identity => delta,
      Mode::Damper {
        decentralization,
        centralization,
      } => {
        if delta < 0.0 {
          delta * (1.0 - decentralization)
        } else {
          delta * (1.0 - centralization)
        }
      }
    }
  }
}

impl Display for Mode {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Mode::Identity => write!(f, "identity"),
      Mode::Damper {
        decentralization,
        centralization,
      } => write!(f, "damper: {} / {}", decentralization, centralization),
    }
  }
}
