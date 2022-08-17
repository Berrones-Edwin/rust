use std::env; //env => enviroments
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing errors: {err}");
        process::exit(1);
    });

    // println!("{:?}", args);
    // println!("searching for {:?}", config.query);
    // println!("in file {}", config.file_path);

    if let Err(e) = minigrep::run(config){
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let content_file =
//         fs::read_to_string(config.file_path).expect("Should have been to read the file");

//     println!("With text:\n{content_file}");

//     Ok(())
// }

// struct Config {
//     query: String,
//     file_path: String,
// }

// impl Config {
//     fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
//         let query = args[1].clone();
//         let file_path = args[2].clone();
//         Ok(Config { query, file_path })
//     }
// }
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     // (query,file_path)
//     Config { query, file_path }
// }
