#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate ring;

#[cfg(test)]
extern crate tempfile;

use std::fs::File;
use std::io::{self, BufWriter, Write};

use clap::{Arg, App};
use ring::rand::SystemRandom;

fn main() {
    let args = make_parser().get_matches();
    let writer = get_writer(args.value_of("output"));
    let size = value_t!(args, "SIZE", usize).unwrap();

    match write_bytes(size, writer) {
        Ok(()) => {}
        Err(e) => panic!("{}", e),
    }
}

fn make_parser<'a, 'b>() -> App<'a, 'b>
    where 'a: 'b
{
    App::new("rand-bytes")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Generate some random bytes")
        .arg(Arg::with_name("output")
                 .short("o")
                 .long("output")
                 .value_name("FILE")
                 .help("Specify a file to output the bytes to, rather than stdout")
                 .takes_value(true))
        .arg(Arg::with_name("SIZE")
                 .help("Sets the number of bytes to generate random values for")
                 .required(true)
                 .index(1))
}

fn rng() -> &'static SystemRandom {
    use std::ops::Deref;

    lazy_static! {
        static ref RANDOM: SystemRandom = SystemRandom::new();
    }

    RANDOM.deref()
}

fn rand_bytes(size: usize) -> Result<Vec<u8>, String> {
    let mut bytes: Vec<u8> = vec![0; size];
    rng().fill(&mut bytes).map_err(|e| e.to_string())?;
    Ok(bytes)
}

fn get_writer<'a>(path: Option<&'a str>) -> Box<Write> {
    let output =
        path.map(|path| Box::new(File::create(path).expect("Unable to create file")) as Box<Write>)
            .unwrap_or_else(|| Box::new(io::stdout()) as Box<Write>);
    Box::new(BufWriter::new(output))
}

fn write_bytes<W: Write>(size: usize, mut writer: W) -> Result<(), String> {
    let bytes = rand_bytes(size)?;
    writer.write_all(&bytes).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use tempfile::NamedTempFile;

    use super::*;

    /// Smoke test to generate a 32 bytes random value to some temporary file
    #[test]
    fn smoke_test() {
        let mut file = NamedTempFile::new().unwrap();
        {
            let arg_vec = vec!["rand-bytes", "-o", file.path().to_str().unwrap(), "32"];
            let args = make_parser().get_matches_from(arg_vec);
            let writer = get_writer(args.value_of("output"));
            let size = value_t!(args, "SIZE", usize).unwrap();
            write_bytes(size, writer).unwrap();
        }

        let mut bytes = Vec::<u8>::new();
        let actual_size = file.read_to_end(&mut bytes).unwrap();
        assert_eq!(32, actual_size);
    }
}
