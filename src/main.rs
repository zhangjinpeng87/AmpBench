mod cmd;

use structopt::StructOpt;
use crate::cmd::Opt;

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
