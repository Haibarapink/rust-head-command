use std::io::{self, BufReader, BufRead};
use std::fs::File;
use clap::{App, Arg};
use std::error::Error;
type MyResult<T> = Result<T , Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    is_byte : bool,
    is_line : bool,
    byte_num : usize,
    line_num : usize,
    file : Vec<String>
}

pub fn get_args() -> Config {
    let matchs = App::new("rust head")
        .version("0.0.1")
        .arg(
            Arg::with_name("file")
                .multiple(true)
                .required(true)
        )
        .arg(
            Arg::with_name("line_count")
            .short("c")
            .required(false)
            .min_values(1)
            .max_values(1)
        )
        .arg(
            Arg::with_name("byte_count")
            .short("b")
            .required(false)
            .max_values(1)
            .min_values(1)
        ).get_matches();

    
    let mut cfg = Config {
        is_byte : matchs.is_present("byte_count"),
        is_line : matchs.is_present("line_count"),
        byte_num : 0,
        line_num : 0,
        file : matchs.values_of_lossy("file").unwrap(),
    };

    cfg.byte_num = if cfg.is_byte {
        matchs.value_of("byte_count").unwrap().parse::<usize>().unwrap()
    } else {
        0
    };

    cfg.line_num = if cfg.is_line {
        println!("{:?}", matchs);
        matchs.value_of("line_count").unwrap().parse::<usize>().unwrap()
    } else {
        0
    };

    cfg
}

pub fn run(c : Config) -> MyResult<()> {
    let mut remain_byte_num = c.byte_num;
    let mut remain_line_num = c.line_num;
    for f in c.file.as_slice() {
      let f_str = f.as_str();
      let fb = open(f_str).unwrap();
      for (line_num , line) in fb.lines().enumerate() {
        if ((c.is_byte == true && remain_byte_num == 0) || (c.is_line == true && remain_line_num == 0)) {
            break;
        }
        let line_as_str = line.unwrap();
        if (c.is_byte) {
            if (remain_byte_num < line_as_str.len()) {
                let tmp_ss = line_as_str.get(0..remain_byte_num).unwrap();
                println!("{}", tmp_ss);
                remain_byte_num = 0;
            } else {
                println!("{}", line_as_str);
                remain_byte_num -= line_as_str.len();
            }
        } else if (c.is_line) {
            remain_line_num -= 1;
            println!("{}", line_as_str);            
        } else {
            // all false
            println!("{}", line_as_str);
        }
      }
    }
    
    Ok(())
}


fn open(filename : &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

