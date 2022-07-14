mod cmd;
mod schema;

use crate::cmd::Opt;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
