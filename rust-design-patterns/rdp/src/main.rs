mod iterator_combinator;
use ark_serialize::{CanonicalDeserialize,Read};
use ark_bls12_381::{g2::Config, Bls12_381, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use core::marker::PhantomData;
use crate::iterator_combinator::{Aptcolx, Buildable, Labardor, Multi, Poodle, Sf};




fn main(){

    // Reading binary file

    // let path = "/Users/upalc/Documents/puzzle-supervillain/public_keys.bin";
    // println!("Current directory: {:?}", std::env::current_dir());

    // /*When you call a function with a generic type parameter, you must provide a concrete type for the function to work with. 
    // This concrete type informs the compiler about what specific implementation to use for the generic functionality */
    // let keys : Vec<(G1Affine, G2Affine)> = iterator_combinator::read_file(path);
    // println!("sdsds{:?}",keys);


    // Reading csv file

    let file : &str = "/Users/upalc/Documents/howMuchRust/rust-design-patterns/rdp/src/read.csv";
    if let Err(e) = iterator_combinator::readcsv(file) {
        println!("{}",e);
    }



    // Marker pattern
    // create instance first 
    let mypoodle : iterator_combinator::Dog<Poodle> = iterator_combinator::Dog{name:"ez".into(),breed:PhantomData};

    let mylab : iterator_combinator::Dog<Labardor> = iterator_combinator::Dog{name:"dfs".into(),breed:PhantomData};
     
    println!("{:?}",mypoodle.breed_feature());


    // trait and trait objects 
    iterator_combinator::build(&Aptcolx);



    // pattern matching
    iterator_combinator::test_write_to_file();





    // builder pattern
    let mut new_car_builder = iterator_combinator::CarBuilder::new();
    new_car_builder::withmake("Forrester");
    let car = new_car_builder::build();


    //Fluent builder or method chaining
    let car = iterator_combinator::Car::builder()
        .withmake("forrester")
        .withcolor("black")
        .withsize(33)
        .withmodel("long")
    ;


}


