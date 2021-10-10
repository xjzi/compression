use compression;
use std::io::{self, Read, Write};
use std::fs::File;

#[macro_use]
extern crate clap;
use clap::App;

#[cfg(test)]
mod test;

fn input(input: Option<&str>) -> Result<Vec<u8>, io::Error> {
    if let Some(filename) = input {
        let mut file = File::open(filename)?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(buf)
    } else {
        let mut buf: Vec<u8> = Vec::new();
        io::stdin().read_to_end(&mut buf)?;
        Ok(buf)
    }
}

fn output(output: Option<&str>) -> Result<Box<dyn Write>, io::Error> {
    if let Some(filename) = output {
        let file = File::create(filename)?;
        Ok(Box::from(file))
    } else {
        let out = io::stdout();
        Ok(Box::from(out))
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input = input(matches.value_of("input-file")).unwrap();
    let output = output(matches.value_of("output-file")).unwrap();
   
    if matches.is_present("compress") {
        compress(&input, output);
    } else if matches.is_present("decompress") {
        decompress(input, output);
    } 

    //match matches.subcommand_name() {
    //    Some("c") => compress(&input, output),
    //    Some("d") => decompress(input, output),
    //    _ => {}
    //}
}

fn compress(bytes: &[u8], mut output: Box<dyn Write>) {
    let bits = compression::compress::compress(bytes);
    let file = compression::wrap_bits(bits);
    output.write_all(&file).ok();
}

fn decompress(bytes: Vec<u8>, mut output: Box<dyn Write>) {
    let bits = compression::unwrap_bytes(bytes);
    let bytes = compression::decompress::decompress(bits);
    output.write_all(&bytes).ok();
}

