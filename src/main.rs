use clap::Clap;

use compiler::Opts;

fn main() {
    let opts = Opts::parse();
    compiler::run(opts);
}
