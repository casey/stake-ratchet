use gnuplot::{AxesCommon, Figure, Fix};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::common::*;

#[derive(StructOpt)]
pub struct Opt {
  #[structopt(long = "delta-magnitude", default_value = "0.0001")]
  delta_magnitude: f64,
  #[structopt(long = "steps", default_value = "1000000")]
  simulation_steps: usize,
  #[structopt(long = "graph-samples", default_value = "1000")]
  graph_samples: usize,
  #[structopt(long = "simulations", default_value = "10")]
  simulations: usize,
  #[structopt(long = "caption")]
  caption: Option<String>,
  #[structopt(long = "output", default_value = "graph.png")]
  output: String,
  #[structopt(long = "open")]
  open: bool,
  #[structopt(subcommand)]
  mode: Mode,
}

impl Opt {
  pub fn run(self) {
    let mut fg = Figure::new();

    fg.set_terminal("pngcairo size 1024, 1024", &self.output);

    let axes = fg.axes2d();

    let title = match self.caption {
      Some(caption) => format!("{} - {}", caption, self.mode),
      None => self.mode.to_string(),
    };

    axes
      .set_title(&title, &[])
      .set_x_label("time", &[])
      .set_y_label("centralization", &[])
      .set_y_range(Fix(-1.0), Fix(1.0));

    for simulation in 0..self.simulations {
      let mut rng = StdRng::seed_from_u64(simulation as u64);

      let mut states = Vec::new();

      while states.len() < self.simulation_steps {
        let (t, last) = states.last().cloned().unwrap_or((0i64, 0.0f64));

        let delta = rng.gen_range(-self.delta_magnitude, self.delta_magnitude);

        let modified = self.mode.modify_delta(delta);

        let raw = last + modified;

        let clamped = raw.max(-1.0).min(1.0);

        states.push((t + 1, clamped));
      }

      let step = (states.len() as f64 / self.graph_samples as f64).floor() as usize;

      let mut ts = Vec::new();
      let mut vs = Vec::new();

      for (t, v) in states.into_iter().step_by(step).take(self.graph_samples) {
        ts.push(t);
        vs.push(v);
      }

      axes.lines(&ts, &vs, &[]);
    }

    fg.show();
    fg.close();

    if self.open {
      Command::new("open").arg(self.output).status().unwrap();
    }
  }
}
