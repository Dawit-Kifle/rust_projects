// use serde_json::{Value};
use std::io;
use std::ptr;

fn main() {

    let s = "sadfsf";
    println!("first s : {:p}, {s}", &s);

    let ss = String::from(s);

    println!("  ss : {:p}, {ss}", &ss);

    let s = "sadf";
    println!("second s : {:p}, {s}", &s);


    println!("{s}");
    println!("new {ss}");


    let a = [10, 20, 30, 40, 50];

    for number in (1..4).rev() {
        println!("{number}");

    }


    // let mut count = 0;
    //
    // 'counting_up: loop
    // {
    //     println!("count = {count}");
    //
    //     let mut remaining = 10;
    //
    //     loop
    //     {
    //         println!("remaining = {remaining}");
    //
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //
    //     }
    //     count += 1;
    // }
    //
    // println!("end count {count}");




    let mut counter = 0;

    let result_ = loop {
        counter += 1;

        if counter == 10 {
            break counter * 30;
        }
    };

    println!("{result_} is  ");


    let condition = false;
    let number = if condition { "5" } else { "sdfsdf" };

    println!("{number}");

    fn five(num: u32) -> u32 {
        num + 5
    }

    let a = five(1);
    println!("{a}");


    // Because Rust is an expression-based language
    let c = {
        let cc = 11;
        cc + 12
    };

    println!("ccccccc     {c}");

    let y = {
        let x = 3;
        x + 1
    };

    println!("the value is {y}");

    let mut guess = String::new();

    let a: [u32; 5] = [1, 2, 3, 4, 5];

    // array should access brackets
    let a = [3; 5];

    println!("{}", a[1]);

    let mut index = String::new();
    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");
    //
    // let index: i32 = index.trim().parse().expect("Index entered was not a number");
    // println!("{}", index);

    // for i in a {
    //     println!("{}", i);
    // }

    // tuple should access dot .
    let x: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", x.1);
    // io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("{}", &mut guess);
    // Constants can be declared in any scope, including the global scope,
    // which makes them useful for values that many parts of code need to know about.

    // const A: usize = 1;

    // shadowing
    let x = 1;
    // let x = x +1;
    let x = x + 1;
    // println!("{x}")

    // let spaces = "   ";
    // let spaces = spaces.len();

    let z = "42".parse::<usize>().expect("Not a number!");
    println!("{}", z)
}
