#![allow(unused)]

#[derive(Debug)]
enum ShirtColor {
    Red,
    Blue,
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}


use std::env::args;

fn main() {

    let user1 = User {
        active: true,
        username: String::from("강용수"),
        email: String::from("gys1120@naver.com"),
        sign_in_count: 23,
    };

    println!("{:?}", user1.active);

    println!("Hello, world!");
}

