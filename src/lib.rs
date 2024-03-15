use std::error::Error;
use std::io::BufRead;

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Default)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Atano Okakura <atanookakura@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number nonblank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

pub fn open(filename: &str) -> MyResult<Box<dyn std::io::BufRead>> {
    match filename {
        "-" => Ok(Box::new(std::io::BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(std::io::BufReader::new(std::fs::File::open(
            filename,
        )?))),
    }
}

fn show_content(
    stream: Box<dyn std::io::BufRead>,
    line_num: &mut i32,
    number: bool,
    number_nonbrank: bool,
) {
    for line in stream.lines() {
        let line = line.unwrap();
        if number {
            println!("{:>6}\t{}", line_num, line);
            *line_num += 1;
        } else if number_nonbrank {
            if !line.is_empty() {
                println!("{:>6}\t{}", line_num, line);
                *line_num += 1;
            } else {
                println!();
            }
        } else {
            println!("{}", line);
        }
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let mut line_num = 1;
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(buffer) => show_content(
                buffer,
                &mut line_num,
                config.number_lines,
                config.number_nonblank_lines,
            ),
        }
    }
    Ok(())
}

// pub fn run(config: Config) -> MyResult<()> {
//     for filename in config.files {
//         match open(&filename) {
//             Err(err) => eprintln!("Failed to open {}: {}", filename, err),
//             Ok(file) => {
//                 let mut last_num = 0;
//                 for (line_num, line) in file.lines().enumerate() {
//                     let line = line?;
//                     if config.number_lines {
//                         println!("{:>6}\t{}", line_num + 1, line);
//                     } else if config.number_nonblank_lines {
//                         if !line.is_empty() {
//                             println!("{:>6}\t{}", last_num + 1, line);
//                             last_num += 1;
//                         } else {
//                             println!();
//                         }
//                     } else {
//                         println!("{}", line);
//                     }
//                 }
//             }
//         }
//     }
//     Ok(())
// }
