use clap::{App, Arg};
use std:: {
    error::Error,
    fs::File,
    io::{self,BufRead,BufReader,Write},
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
            eprintln!("{}: .* [(]os error 2[)]", config.in_file);
            std::process::exit(1)
        },
        Ok(_file) => {
            //Initiate Current line being read and the previous line to be compared with
            let mut line = String::new();
            let mut previous = String::new();
            //Initialize the count to 0, this is the number of time we have found identic lines
            let mut count = 0;
            //Open either an input file or stdin depending on what is provided in the cli
            let mut file = open(&config.in_file)
                .map_err(|e| format!("{}: {}", config.in_file,e))?;
            //Open an output file or an stdout depending on what is provided in the cli
            let mut out_file: Box<dyn Write> = match &config.out_file {
                Some(out_name) => Box::new(File::create(out_name)?),
                _ => Box::new(io::stdout()),
            };
            //This closure will have as parameters the count and the content of the previously read
            //If the count is at 0 we don't write anything else we either write the output to file
            //or stdout with tho count or not if specified in the CLI ( config.count )
            let mut print = |count:u64, text: &str| -> MyResult<()> {
                if count > 0 {
                    if config.count {
                        write!(out_file,"{:>4} {}", count, text)?;
                               } else {
                                   write!(out_file,"{}",text)?;
                               }
                    };
                Ok(())
                };
            //Start an ifinite loop
            loop {
                //We start to read the file or stdin line by line until line return 
                let bytes = file.read_line(&mut line)?;
                //If we don't have any data it meams we are at the end of the file so we break out
                //of the loop.
                if bytes == 0 {
                    break;
                }
                //If the current line and the previous line are different we trigger the print()
                //closure then we backup the current line in the previous variable for future check
                //and reset the count to 0
                if line.trim_end() != previous.trim_end() { 
                    print(count,&previous)?;
                    previous = line.clone();
                    count = 0;
                }
                // if both line are the same we increase the count by one then we clear the line
                // buffer to get the next line in the next round.
                count += 1;
                line.clear();
            }
            //finally we handle the last line of the file
            print(count,&previous)?;
        
    Ok(())
}
}
}

