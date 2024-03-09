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


impl Dog<Labardor> {
    pub fn breed_name (&self) -> &'static str {
        "labardor"
    }
}

impl Dog<Poodle> {
    pub fn breed_feature (&self) -> &'static str {
        "poodle's features"
    }
}



/*
Without phantom data implementation:

Trait implementations can now be done on the breed instances themselves rather than on 
the Dog struct parameterized by a breed type, 
allowing for more straightforward trait usage and implementation but it uses storage. 
struct Dog<DogBreed> {
    name: String,
    breed: DogBreed,
}

trait BreedName {
    fn breed_name(&self) -> &'static str;
}

// Assume Dachshund is a struct representing the breed.
struct Dachshund;

// Implementing the BreedName trait for Dachshund.
impl BreedName for Dachshund {
    fn breed_name(&self) -> &'static str {
        "dachshund"
    }
}

// Example usage
fn main() {
    // Now you must provide an instance of Dachshund when creating a Dog.
    let my_dog = Dog {
        name: "Fido".to_string(),
        breed: Dachshund,
    };

    // Accessing the breed name through the method defined in the trait.
    println!("Breed name: {}", my_dog.breed.breed_name());
}


*/
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
pub fn build (house : &dyn Room) -> u8 { // instead of multi or sf use trait to use trait obj. use dyn or impl
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
pub struct Car {
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

pub struct CarBuilder {
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


// spread syntax
let car1 = Car {
        make: "fr".into(),
        model: "sub".into(),
        size: 51,
        color: "red".into(),
    };
        println!("{:?}", car1);
let car2 = Bicycle {
    size: 58,
    ..bicycle1
    };
    println!("{:?}", car2);




// Fluent builder pattern or method chaining
impl CarBuilder {
    fn withmake (self, make: &str) -> Self {
        Self{
            car:Car { make: make.into(),
                ..self.car
            }
        }
    }

     fn withcolor (self, color: &str) -> Self {
        Self{
            car:Car { color: color.into(),
                ..self.car
            }
        }
    }
     fn withmodel (self, model: &str) -> Self {
        Self{
            car:Car { model: model.into(),
                ..self.car
            }
        }
    }
     fn withsize (self, size: &u8) -> Self {
        Self{
            car:Car { size: size,
                ..self.car
            }
        }
    }
}



/* Observable pattern : an object, called the subject, maintains a list of its dependents, called observers, and notifies them of any state changes, usually by calling one of their methods.

Self::Subject and Self::Observer are ways to refer to the associated types declared in their respective traits.
They allow for generic programming by letting trait methods operate on unspecified types that are later concretely defined by the implementers of the traits.
Arc: A thread-safe reference-counting pointer. Arc stands for Atomically Reference Counted. It allows multiple owners of the same data, with the data cleaned up when the last owner goes out of scope.
Weak: A non-owning reference to a value within an Arc. Unlike Arc, a Weak reference does not increment the reference count. This is used to prevent circular references which could lead to memory leaks because the reference count never reaches 0.
dyn Observer: Indicates a trait object. It allows for polymorphism, meaning any type that implements the Observer trait can be referenced by dyn Observer. 
The Subject = Self part is a trait bound that ensures the Observer trait is implemented for the type Subject


*/

pub trait Observer {
    type Subject;
    fn observe (&self, subject: &Self::Subject);
}

pub trait Observable {
    type Observer;
    fn update(&self);
    fn attach(&mut self, observer: Self::Observer);
    fn detach(&mut self, observer: Self::Observer);
}


pub struct Subject {
    observers: Vec<Weak<dyn Observer<Subject=Self>>>,
}

impl Observable for Subject {
    type Observer = Arc<dyn Observer<Subject = Self>>;
    fn update(&self) {
        self.observers
            .iter()
            .flat_map(|o| o.upgrade())
            .for_each(|o| o.observe(self));
    }
    fn attach(&mut self, observer: Self::Observer) {
            self.observers.push(Arc::downgrade(&observer));
    }
    fn detach(&mut self, observer: Self::Observer) {
        self.observers
            .retain(|f| !f.ptr_eq(&Arc::downgrade(&observer)));
    }
}




// Creating state machine with methodless trait and struct tagging

pub trait SessionState {}

pub struct Session<State:SessionState=Initital>{
    session_id: Uuid,
    properties:HashMap<String,String>,
    phantom:PhantomData<State>,
}


pub struct Initital;
pub struct Anon;

pub struct Authenticated;

pub struct LoggedOut;


impl SessionState for Initital {}
impl SessionState for Anon {}
impl SessionState for Authenticated {}
impl SessionState for LoggedOut {}



pub enum ResumeResult {
    Invalid,
    Anon(Session<Anon>),
    Authenticated(Session<Authenticated>),
}


// Now we will implement same struct but with different tagging 
impl Session<Initial> {
/// Returns a new session, defaulting to the anonymous state
pub fn new() -> Session<Anon> {
    Session::<Anon> {
        session_id: Uuid::new_v4(),
        props: HashMap::new(),
        phantom: PhantomData,
    }
}
/// Returns the result of resuming this session from an existing ID.
pub fn resume_from(session_id: Uuid) -> ResumeResult {
// Here we'd have to check the session_id against a database,
// and return the result accordingly. For this example we'll
// just return a new authenticated session for test purposes.
    ResumeResult::Authenticated(Session::<Authenticated> {
        session_id,
        props: HashMap::new(),
        phantom: PhantomData,
        })
    }
}

// Transition from anon to authenticated
impl Session<Anon>{
    pub fn authenticate(
        self,
        username: &str,
        password: &str,
    ) -> Result<Session<Authenticated>,Session<Anon>> {
        
       if !username.is_empty() && !password.is_empty(){ 
        Ok(Session::<Authenticated>{
            session_id:self.session_id,
            props:HashMap::new(),
            phantom:PhantomData,
        })
    } else {
        Err(self)
    }

    }
}

// Adding trasitions for authenticated session

impl Session<Authenticated> {
    fn update_property(&mut self, key: &str, value:&str) {
        if let Some(prop) = self.properties.get_mut(key) {  // matching a single pattern
            *prop = value.to_string();
        } else {
            self.properties.insert(key.to_string(),value.to_string());
        }
    }

    fn logout (self) -> Session<LoggedOut>{
        // In real life delete the session from DB
        Session { session_id: Uuid::nil(), properties:HashMap::new(), phantom: PhantomData }
    }
}


// Metaprogramming with macros: macros are like matching 
macro_rules! newmacro {
    ($($arg::aa)*) => {
        println!("fasdfa: {}",$($arg)*)
    };
}

macro_rules! car_struct {
    ($carmodel:ident) => {
            struct $carmodel {
                    name: String,
                    age: i32,
                    model: String,
                }
            impl $carmodel {
                fn new(name: &str, age: i32) -> Self {
                            Self {
                                name: name.into(),
                                age,
                                model: stringify!($carmodel).into(),
                            }
                }
            }
        };
}
car_struct!(Suv);
car_struct!(Sedan);



// Pattern matching 
fn get_username(user_id: u32) -> Option<String> {
    match user_id {
        1 => Some(String::from("JohnDoe")),
        2 => Some(String::from("JaneDoe")),
        _ => None,
    }
}

fn patternmatching() {
    let user_id = 1;
    if let Some(username) = get_username(user_id) {
        println!("Found user: {}", username);
    } else {
        println!("No user found with ID {}", user_id);
    }
}



// Iterator combinator functional 
fn iteratorcombinator() {
    let numbers = vec![1, 2, 3, 4, 5];

        // Use `map` to square each number
    let squares = numbers.iter().map(|&x| x * x);

        // Use `fold` to sum up the squares
    let sum_of_squares = squares.fold(0, |acc, x| acc + x);

    
    let secret = Fr::from(BigInt!("123"));
    let my_key = G1Affine::generator().mul(secret).into_affine();
    let new_key = public_keys
            .iter()
            .fold(G1Projective::from(my_key), |acc, (key, _)| acc + key.neg())
            .into_affine();

    let my_proof = pok_prove(secret, new_key_index);
    let new_proof = public_keys
            .iter()
            .enumerate()
            .fold(G2Projective::from(my_proof), |acc, (i, (_, proof))| {
                let rhs = Fr::from(new_key_index as u128 + 1) * Fr::from(i as u128 + 1).inverse().unwrap();
                acc + proof.mul(rhs).neg()
            })
            .into_affine();

    let my_sig = bls_sign(secret, message);
    let fake_signature = public_keys
            .iter()
            .fold(G2Projective::from(my_sig), |acc, (_, proof)| acc + proof.neg())
            .into_affine();
    let aggregate_signature = public_keys
            .iter()
            .fold(G2Projective::from(fake_signature), |acc, (_, proof)| acc + proof)
            .into_affine();


}