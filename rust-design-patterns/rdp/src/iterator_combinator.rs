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

pub struct Aptcolx;


trait Bathroom {
    fn add (&self) -> u8;
}

trait Garage {
    fn add (&self) -> u8;
}

pub trait Room {
    fn add (&self) -> u8 { // This is default value 
        2
    }
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

impl Room for Aptcolx { // testing default
    // fn add (&self) -> u8
    // {
    //     8
    // }
    
}



pub fn build (house : &impl Room) -> u8 { // instead of multi or sf use trait to use trait obj. use dyn or impl
    let s = house.add();

    println!("{:?}",s);

    s

}

fn test_trait_obj (){
    let mut v = Vec::<Box<dyn Room>>::new(); // Trait objects are not sized thus need to use Box. other ?Sized are Rc, Refcell, Mutex
    v.push(Box::new(Sf {}));
    v.push(Box::new(Multi {}));
    v.iter().for_each(|i| i.add());
}


// Pattern matching 
fn write_to_file() -> std::io::Result<()> {

    use std::io::prelude::*;
    let mut file = File::create("write.csv")?;
    file.write_all(b"abcks cs")?;
    Ok(())
}
pub fn test_write_to_file() {
    match write_to_file() {
        Ok(()) => println!("Write suceeded"),
        Err(err) => println!("Write failed: {}", err.to_string()),
    }
}



// Builder pattern

pub trait Builder<T> {
    fn new() -> Self;
    fn build(self) -> T;
}


pub trait Buildable<T, B:Builder<T>> {
    fn builder () -> B;
}



#[derive(Debug)]
struct Car {
    make : String,
    model : String,
    size : u8,
    color : String
}

impl Car {
    fn make(&self) -> &String {
        &self.make
    }
    fn model(&self) -> &String {
        &self.model
    }
    fn size(&self) -> &u8 {
        &self.size
    }
    fn color(&self) -> &String {
        &self.color
    }

}

struct CarBuilder {
    car : Car,
}

impl  CarBuilder {
    // fn new () -> Self{

    //     Self { 
    //         car : Car {
    //                 make : String::new(),
    //                 model : String::new(),
    //                 size : 0,
    //                 color : String::new(),
    //         }
    //      }

    // }


    fn withmake (&mut self, make : &str) {
        self.car.make = make.into()
    }

    fn withmodel (&mut self, model : &str) {
        self.car.model = model.into()
    }

    fn withsize (&mut self, size: u8) {
        self.car.size = size
    }

    fn withcolor (&mut self, color: &str) {
        self.car.color = color.into()
    }
}

impl Builder<Car> for CarBuilder {
    fn new() -> Self {
                    Self {
                        car: Car {
                            make: String::new(),
                            model: String::new(),
                            size: 0,
                            color: String::new(),
                    },
                }
            }
    fn build(self) -> Car {
        self.car
    }
}

// get a new instance of  builder directly from a Car
impl Buildable<Car,CarBuilder> for Car { 
    fn builder () -> CarBuilder {
            CarBuilder::new()
        
    }

}








