use std::sync::Mutex;
use std::{fs::File, marker::PhantomData};
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



// Marker patter : Phantom data : store state as type parameter

pub struct Dog<Breed>{
        pub name:String,
        pub breed:PhantomData<Breed>,
    }

pub struct Poodle {}
pub struct Labardor {}




// pecialized implementations to get the name of the breed, without having to actually
//store that value as state in the structure
//doing a concrete specialization for with the given type in compile time opt
impl Dog<Labardor> {
    pub fn breed_name (&self) -> &'static str {
        "labardor"
    }
}

impl Dog<Poodle> {
    pub fn breed_feature (&self) -> &'static str {
        "poodle's feature like barking"
    }
}


// Traits and trait objects : it allows 
// Separation of state and functions AND polymorphism (trait objects)

pub struct Sf;

pub struct Multi;

struct Aptcolx;


trait Bathroom {
    fn add (&self) -> u8;
}

trait Garage {
    fn add (&self) -> u8;
}

pub trait Room {
    fn add (&self) -> u8;
}

// trait Build = Bathroom + Garage + Room;

impl Room for Sf {
    fn add (&self) -> u8{
        4
    }
    
}

impl Room for Multi {
    fn add (&self) -> u8{
        8
    }
    
}




pub fn build (house : &impl Room) -> u8 { // instead of multi or sf use trait to use trait obj
    let s = house.add();

    println!("{:?}",s);

    s

}

