#![allow(unused_variables)] // Suppresses warnings for unused variables (for demonstration)
#![warn(unused_mut)] // Warns if a mutable variable is never mutated


use std::cell::RefCell;
use std::sync::Mutex;
use std::marker::PhantomData;

trait MyTrait {
    fn process(&self) -> i32; // a simple method for demonstration
}

struct Container<T: MyTrait + 'static> {
    values: Mutex<RefCell<Vec<Box<dyn MyTrait>>>>,
    _marker: PhantomData<T>,
}

impl<T: MyTrait + 'static> Container<T> {
    fn new() -> Container<T> {
        Container {
            values: Mutex::new(RefCell::new(Vec::new())),
            _marker: PhantomData,
        }
    }

    fn add(&self, value: T) {
        let values = self.values.lock().unwrap();
        values.borrow_mut().push(Box::new(value));
    }

    // Applies a function to each item and collects the results
    fn map<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(&dyn MyTrait) -> R,
    {
        self.values.lock().unwrap().borrow().iter().map(|value| f(&**value)).collect()
    }

    // Combines all items into a single result
    fn fold<R, F>(&self, init: R, f: F) -> R
    where
        F: Fn(R, &dyn MyTrait) -> R,
        R: Clone,
    {
        self.values.lock().unwrap().borrow().iter().fold(init, |acc, value| f(acc, &**value))
    }

    // Finds the first item satisfying a predicate and returns a reference to it
 // Finds the first item satisfying a predicate and returns the result of processing it
    fn find<F>(&self, f: F) -> Option<i32>
    where
        F: Fn(&dyn MyTrait) -> bool,
    {
        self.values.lock().unwrap().borrow().iter()
            .find(|&value| f(&**value))
            .map(|value| value.process())
    }

}

struct MyType(i32);

impl MyTrait for MyType {
    fn process(&self) -> i32 {
        self.0
    }
}

fn main() {
    let container = Container::<MyType>::new();
    container.add(MyType(10));
    container.add(MyType(20));
    container.add(MyType(30));

    // Use map to process each element
    let results: Vec<i32> = container.map(|item| item.process());
    println!("Mapped results: {:?}", results);

    // Use fold to sum up the results
    let sum = container.fold(0, |acc, item| acc + item.process());
    println!("Sum: {}", sum);

    // Use find to get the first item with a value greater than 15
    match container.find(|item| item.process() > 15) {
    Some(result) => println!("Found and processed item: {}", result),
    None => println!("No item found"),
}
}
