mod iterator_combinator;
use ark_serialize::{CanonicalDeserialize,Read};
use ark_bls12_381::{g2::Config, Bls12_381, Fr, G1Affine, G1Projective, G2Affine, G2Projective};




fn main(){

    // Reading binary file

    // let path = "/Users/upalc/Documents/puzzle-supervillain/public_keys.bin";
    // println!("Current directory: {:?}", std::env::current_dir());

    // /*When you call a function with a generic type parameter, you must provide a concrete type for the function to work with. 
    // This concrete type informs the compiler about what specific implementation to use for the generic functionality */
    // let keys : Vec<(G1Affine, G2Affine)> = iterator_combinator::read_file(path);
    // println!("sdsds{:?}",keys);


    // Reading csv file

    let file : &str = "./read.csv";
    if let Err(e) = iterator_combinator::readcsv(file) {
        println!("{}",e);
    }



    
}