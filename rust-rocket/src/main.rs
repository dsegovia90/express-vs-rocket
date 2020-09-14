#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
#[macro_use] extern crate serde_derive;
use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;
use std::time::Instant;

#[derive(Serialize)]
struct ResponseBack {
    result: String,
    performance: String
}


#[get("/hello")]
fn hello() -> Json<ResponseBack> {
    let now = Instant::now();

    Json(ResponseBack {
        result: String::from("Hello world!"),
        performance: format!(
            "{}s and {}ns",
            now.elapsed().as_secs(), now.elapsed().as_nanos()
        )
    })
}

#[get("/fib?<n>")]
fn fib(n: u128) -> Json<ResponseBack> {
    let now = Instant::now();
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = replace(&mut f1, f2);
    }

    Json(ResponseBack {
        result: f0.to_string(),
        performance: format!(
            "{}s and {}ns",
            now.elapsed().as_secs(), now.elapsed().as_nanos()
        )
    })
}

#[get("/dumb-sum?<n>")]
fn dumb_sum(n: u128) -> Json<ResponseBack> {
    let now = Instant::now();

    let number = n;
    let mut arr = Vec::new();
    for i in 0..number + 1 {
        let mut sum: u128 = 0;
        for j in 0..i + 1 {
            sum = sum + j;
        }
        arr.push(sum);
    }

    Json(ResponseBack {
        result: arr[arr.len() - 1].to_string(),
        performance: format!(
            "{}s and {}ns",
            now.elapsed().as_secs(), now.elapsed().as_nanos()
        )
    })
}

fn main() {
    rocket::ignite().mount("/api", routes![
        hello,
        dumb_sum,
        fib
    ]).launch();
}