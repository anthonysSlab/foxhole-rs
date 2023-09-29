use vegemite::{run, sys, Get, Route};

fn get(_get: Get) -> u16 {
    println!("Get");
    200
}

fn main() {
    let router = Route::new(sys![get]);

    println!("Running on '127.0.0.1:5000'.");

    run("127.0.0.1:5000", router);
}
