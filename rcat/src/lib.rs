use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use::std::io::{self, BufRead, BufReader};


#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}


type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    //dbg!(config)ĸø;
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut line_number = 0;
                for line in file.lines() {
                    
                    let result = line.unwrap();
                     
                    if config.number_lines || config.number_nonblank_lines {
                        
                        if result.is_empty() && config.number_nonblank_lines {
                            println!("");
                            line_number -= 1;
                        }
                        else {
                            println!("{:>6}\t{}", line_number + 1 ,result);
                        }
                    
                    } else {

                        println!("{}", result)
                        }
                
                    line_number += 1;
                }

            }
        
        };
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("rcat")
        .version("0.1.0")
        .author("Hans Buss <hans.buss@mailfence.com>")
        .about("Rust implementation of cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        ).arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("number nonempty output lines, overrides -n")
                .takes_value(false),
            
        ).arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("number all output lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
            )
            .get_matches();
            
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
