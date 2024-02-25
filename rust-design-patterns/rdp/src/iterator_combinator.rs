use std::fs::File;
use std::io::Cursor;
use ark_serialize::{CanonicalDeserialize,Read};
use std::error::Error;

// const path: &str = r"/Users/upalc/Documents/puzzle-supervillain/public_keys.bin";

pub fn read_file<T:CanonicalDeserialize>(path:&str) -> T {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    T::deserialize_uncompressed_unchecked(Cursor::new(&buffer)).unwrap()
}

// read csv file into a structure
use serde::Deserialize;

#[derive(Deserialize)]
struct Record {
    year : u16,
    make : String,
    model : String,
    description : String,
}


pub fn readcsv(path : &str) -> Result<(),Box<dyn Error>> {
    // let mut file = File::open(path);
    
    // if reading from command line
    // let mut reader = csv::Reader::from_reader(io::stdin());

    // if reading from path
    let mut reader = csv::Reader::from_path(path).unwrap();
    let headers = reader.headers().unwrap();
    println!("{:?}",headers);

    for record in reader.deserialize() { // reader.records() in case just reading
        let record : Record = record.unwrap();
        println!(
            "In {}, {} built the {} model. It is a {}.",
            record.year,
            record.make,
            record.model,
            record.description
        );
    }

    Ok(())
    
}




