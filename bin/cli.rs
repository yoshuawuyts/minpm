extern crate getopts;

use getopts::Options;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut options = Options::new();
  options.optflag("h", "help", "Output usage information");

  let matches = match options.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => { panic!(f.to_string()) }
  };

  // help command
  if matches.opt_present("h") {
    print_usage(&program, options);
    return;
  }
}

// print CLI usage
fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} <file> [options]", program);
  print!("{}", opts.usage(&brief));
}
