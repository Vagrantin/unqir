use clap::{App, Arg};
use std:: {
    error::Error,
    fs::File,
    io::{self,BufRead,BufReader},
};

type MyResult<T> = Result<T,Box<dyn Error>>;

#[derive(Debug)]
pub struct Config{
    in_file: String,
    out_file: Option<String>,
    count:bool,
}


pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .version("0.1.0")
        .author("Math Duc <mathduc@gamil.com>")
        .about("Rust uniq")
        .arg(
            Arg::with_name("in_file")
            .value_name("FILE")
            .help("input file")
            .default_value("-")
        )
        .arg(
            Arg::with_name("out_file")
            .value_name("OUT FILE")
            .help("output file")
        )
        .arg(
            Arg::with_name("count")
            .short("c")
            .long("count")
            .help("print the newline counts")
            )
        .get_matches();
    let count = matches.is_present("count");

    Ok(Config{
        in_file:matches.value_of("in_file").unwrap().to_string(),
        out_file:matches.value_of("out_file").map(|s| s.to_string()),
        count,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
pub fn run(config: Config) -> MyResult<()> {
    match open(&config.in_file) {
        Err(_err) => {
            eprint!("{}: .* [(]os error 2[)]", config.in_file);
            std::process::exit(1)
        },
        Ok(file) => {
            if config.count {
                let mut myfile = Vec::new();
                let lines = file.lines().map(|v| v).collect::<Result<String,_>>().unwrap();
                for c in lines.chars() {
                    myfile.push(c); 
                }
//                myfile.dedup();
                for x in &myfile {
                    println!("{x}");
//                    println!("{}",myfile.len())
                }
                for x in &myfile {
                    let pre_el = String::new();
                    let el = String::new();
                    let mut count = 0;
                    let pre_el = x;
                    let el = x;
                    if pre_el == el {
                        count += 1
                    }
                        println!("test {count} {x}");
                }
            }
            /*
            let mut prevline = &String::new();
            for (_line_num,line)  in file.lines().enumerate() {
                let line = line?;
                prevline = &line;
                let mut count = 0;
                if &line == prevline {
                    count += 1;
                }
                println!("{} {}", count,line)
            }
            */

        }

       // println!(" c'est mon fichier ? {:?}", config.in_file );
    }
    /*
    println!("{:?}", config);

    */
    Ok(())
}

