use std::error::Error;
use std::process;

use serde::Deserialize;
use std::fs::File;
use std::collections::HashMap;
use regex::{Regex};
use std::time::Instant;
use std::path::Path;
use itertools::Itertools;
use lazy_static::lazy_static;

#[derive(Debug, Deserialize)]
struct Movie {
    #[serde(rename = "movieId")]
    movie_id: u32,
    title: String,
    genres: String,
}

#[derive(Debug)]
struct Stat {
    genre: String,
    total: u32,
    total_by_year: HashMap<u16, u32>,
}

impl Stat {
    fn new(genre: String) -> Stat {
        Stat {
            genre,
            total: 0,
            total_by_year: HashMap::new(),
        }
    }

    fn update(&mut self, movies: Vec<Movie>) {
         //TODO Update statistics based on the movies passed as argument
         lazy_static! {
            static ref REGEX: Regex = Regex::new(r"\(([0-9]{4})\)").unwrap();
        }
        let self_genre = self.genre.clone();
        for (year, movies) in &movies.iter()
            .filter(|&m| m.genres.to_lowercase().contains(&self_genre))
            .group_by(|&m| {
                match REGEX.captures(&m.title) {
                    None => { 0 }
                    Some(c) => {
                        c.get(1).unwrap().as_str().parse::<u16>().unwrap()
                    }
                }
            }) {
            let nbr = movies.count() as u32;
            *self.total_by_year.entry(year).or_insert(0) += nbr;
            self.total += nbr;
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    //parse genre from command line
    let genre = std::env::args().nth(1).expect("no genre given").to_lowercase();

    //open file
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(Path::new("./movies.csv")) {
        Err(why) => panic!("couldn't open the file: {}", why),
        Ok(file) => file,
    };
    let mut rdr = csv::Reader::from_reader(file);

    let mut stat = Stat::new(genre);

    for result in rdr.deserialize() { //reads chucks of lines
        stat.update(result?);
    }
    println!("{:?}", stat);

    Ok(())
}

fn main() {
    let now = Instant::now();

    if let Err(err) = run() {
        println!("error running parser: {}", err);
        process::exit(1);
    }

    println!("{}ms", now.elapsed().as_millis());
}
