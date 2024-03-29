use std::io::Read;
use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Dump {
    /// File containing a BUFR message to be dumped
    #[structopt(parse(from_os_str))]
    message: PathBuf,

    /// Print raw data for section 4
    #[structopt(long = "show-data")]
    show_data: bool,
}

fn main() -> Result<()> {
    let args = Dump::from_args();

    let mut buffer = vec![];
    let file = std::fs::File::open(args.message)?;
    let mut reader = std::io::BufReader::new(file);
    reader.read_to_end(&mut buffer)?;

    let data = bufr::decode(&buffer[..])?;

    if args.show_data {
        println!("{:?}", data);
    } else {
        println!("{}", data);
    }

    Ok(())
}
