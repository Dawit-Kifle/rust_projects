use std::io;

fn main() {

    let my_name = String::from("alpha");

    show_my_name(&my_name);

    println!("{}", my_name);
}

fn show_my_name(name: &String) {
    println!("{}", name);
}