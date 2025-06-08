use std::{error::Error, io, path::Path};

pub struct Config<'a> {
    pub source_path: &'a Path,
    pub output_count: usize,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a Vec<String>) -> Result<Config<'a>, Box<dyn Error>> {
        if args.len() < 2 {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Not enough arguments!",
            )));
        }

        let source_path = Path::new(&args[1]);
        let output_count = if let Some(number) = args.get(2) {
            number.parse::<usize>()?
        } else {
            10
        };

        Ok(Config {
            source_path,
            output_count,
        })
    }
}
