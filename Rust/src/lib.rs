use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub struct Config {
    pub filename: String,
    pub use_stdin: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        match args.len() {
            1 => Ok(Config {
                filename: String::from(""),
                use_stdin: true,
            }),
            2 => Ok(Config {
                filename: args[1].clone(),
                use_stdin: false,
            }),
            _ => Err("Unable to parse file name"),
        }
    }
}

pub struct Counts {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
    is_word: bool,
}

impl Counts {
    pub fn new(lines: usize, words: usize, bytes: usize) -> Counts {
        let is_word = false;
        Counts {
            lines,
            words,
            bytes,
            is_word,
        }
    }

    pub fn read_line(&mut self, contents: &String) {
        self.bytes += contents.len();
        self.lines += 1;

        for c in contents.chars() {
            if c.is_whitespace() {
                self.is_word = false;
            } else if !self.is_word {
                self.words += 1;
                self.is_word = true;
            }
        }

        self.is_word = false;
    }
}

fn open_file(config: &Config) -> Result<Box<dyn io::BufRead>, io::Error> {
    if config.use_stdin {
        Ok(Box::new(io::BufReader::new(io::stdin())))
    } else {
        let file = File::open(&config.filename)?;
        let buf_reader = io::BufReader::new(file);
        Ok(Box::new(buf_reader))
    }
}

pub fn wc(reader: &mut impl BufRead) -> Result<Counts, io::Error> {
    let mut counts = Counts::new(0, 0, 0);

    let mut buffer = String::new();

    while reader.read_line(&mut buffer)? > 0 {
        counts.read_line(&buffer);
        buffer.clear();
    }

    Ok(counts)
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut reader = open_file(&config)?;
    let counts = wc(&mut reader)?;

    println!(
        "{}\t{}\t{}\t{}",
        counts.lines, counts.words, counts.bytes, config.filename
    );

    Ok(())
}
