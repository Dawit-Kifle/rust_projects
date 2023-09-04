#![allow(unused)]

use std::borrow::Borrow;
use std::io;
use std::ptr;

const VERSION: &str = "1.0.0";

fn calculate_length(s: &String) -> (String, usize) {
    // These ampersands represent references, and they allow you to refer to some value without taking ownership of it. Figure 4-5 depicts this concept.
    // references를 사용한다는 것은 ownershiop을 이전하지 않고 사용하는 것을 의미하는 것 같다.
    // 그래서 main 함수에서 재사용 가능
    let length = s.len(); // len() returns the length of a String

    (s.to_string(), length)
}

fn change(s: &mut String) -> String {
    s.push_str(",,,,,,,,,,, world");
    s.to_string()
}

fn dangle() -> String {
    let s = String::from("helsssslo");

    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}



fn main() {

    let v = vec![1,2,3,4,5];

    let does_not_exist: &i32 = &v[1];

    println!("{:p}", does_not_exist);
    println!("{}", does_not_exist);
    println!("{:p}", &v[1]);
    println!("{:b}", &v[1]);


    // //let v: Vec<i32>  = Vec::new();
    //
    // fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    //     let s1 = sum_vec(v1);
    //     let s2 = sum_vec(v2);
    //
    //     s1 + s2
    // }
    //
    // fn sum_vec(v: &Vec<i32>) -> i32 {
    //     v.iter().fold(0, |a, &b| a + b)
    // }
    //
    // let v1 = vec![1, 2, 3];
    // let v2 = vec![4, 5, 6];
    //
    // let v3 = sum_vec(&v1);
    //
    // let answer = foo(&v1, &v2);
    //
    // println!("{}", answer);

    // println!("{:?}, {:?}, {}", v3, v4, answer);
    //
    // println!("{:?}", v1);

    // let v = vec![1,2,3,4,5];
    //
    // let third = &v[2];
    // println!("{}", third);
    // println!("{:?}", v);


    //
    // let v = vec![1, 2, 3, 4, 5];
    // let a = &v[0];
    //
    // let third: Option<&i32> = v.get(2);
    //
    // println!("{}", a);
    // println!("{}", v[1]);
    //
    //
    //
    // let s = "sdfasd".to_string();
    //
    // let ss = &s;
    //
    // let s12 = &s[..];
    //
    //
    // let s:u8 = 1;
    //
    // let s = "강용";
    //
    // println!("{}", (&s));



    // let mut s1 = String::from("hellow");
    // let s2 = String::from("world");
    //
    // let a = s2.as_str();
    // let s3 = s1 + &s2;
    //
    // let s3 = s1 + a;
    //
    // println!("{s3}");




    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");




    // Recall that we talked about string liters being stored inside the binary. Now that we know about slices,
    // we can properly understand string literals;

    let s = "Hello world";

    //the type of s here is &str: it's a slice pointing to that specific point of the binary.
    // This is also why string literals are immutable; &str is an immutale reference.


    let my_string = String::from("hello world");

    // first)wird works on slices of String s, whether partial of whole
    let word = first_word(&my_string[..]);
    // let s: String = String::from("hello world");
    // first_word(&s);
    //
    //
    // println!("{:p}", &s);
    // // print!("{:?}", [0,7]);
    // let z = &s;
    //
    //
    // println!("{:p}", z);
    //
    // println!("{}", *z)

    // let mut s = String::from("hello world");
    //
    // let word = first_word(&s); // word will get the value 5
    // println!("{word}");
    // s.clear();
    //
    // println!("{word}");
    // println!("{reference}");
    //
    //
    // let mut s = String::from("hello");
    //
    // let r1 = &s;
    // let r2 = &s;
    //
    // println!("{}, {}", r1, r2);
    //
    // let r3 = &mut s; // no problem
    //
    // println!("{s} sdfsdfsd");
    //
    //
    // // println!("{}, {}", r1, r2);
    //
    // // runtime allocated variable
    // // let mut s1 = String::from("hello");
    // //
    // // let s2 = &mut s1;
    // // let s3 = &mut s1;
    // //
    // // println!("{}, {}", s2, s3);
    // //
    // // // We call the action of creating a reference borrowing.
    // // // let len = calculate_length(&s1);
    // //
    // // let s = change(&mut s1);
    // //
    // // println!("{s}");
    //
    //
    // // println!("The length of '{}' is {}.", len.0, len.1);
    // //
    // // println!("{s1}");
    //
    //
    // // shadowing은 type이 달라도 상관없다.
    // // 하지만 mut은 최초에 선언한 타입과 다른 타입의 값을 할당할 시 에러가 발생한다.
    // let spaces = "   ";
    // let spaces = spaces.len();
    //
    // println!("{spaces}");
    // let x = 5;
    // println!("The first of x is: {x}, {:p}", &x);
    //
    // //let x = x + 1;
    //
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}, {:p}", &x);
    //
    // }
    //
    // println!("The value of x is: {x}, {:p}", &x);
    //
    // let x = 6;
    // println!("The value of x is: {x}, {:p}", &x);
    //
    // println!("____________________________\n
    // ______________________________");
    //
    //
    // // 1. const는 shadowing이 안된다.
    // // 2. const는 타입 명시를 반드시 해줘야 한다. -> 결국엔 상수는 타입 추론이 아니라 컴파일 시에 명시적으로 알 수 있어야 하기 때문인것 같다.
    // const b: isize = 300;
    //
    //
    //
    //
    // let s = "sadfsf";
    // println!("first s : {:p}, {s}", &s);
    //
    // let ss = String::from(s);
    //
    // println!("  ss : {:p}, {ss}", &ss);
    //
    // let s = "sadf";
    // println!("second s : {:p}, {s}", &s);
    //
    //
    // println!("{s}");
    // println!("new {ss}");
    //
    //
    // let a = [10, 20, 30, 40, 50];
    //
    // for number in (1..4).rev() {
    //     println!("{number}");
    //
    // }
    //
    //
    // // let mut count = 0;
    // //
    // // 'counting_up: loop
    // // {
    // //     println!("count = {count}");
    // //
    // //     let mut remaining = 10;
    // //
    // //     loop
    // //     {
    // //         println!("remaining = {remaining}");
    // //
    // //         if remaining == 9 {
    // //             break;
    // //         }
    // //         if count == 2 {
    // //             break 'counting_up;
    // //         }
    // //         remaining -= 1;
    // //
    // //     }
    // //     count += 1;
    // // }
    // //
    // // println!("end count {count}");
    //
    //
    //
    //
    // let mut counter = 0;
    //
    // let result_ = loop {
    //     counter += 1;
    //
    //     if counter == 10 {
    //         break counter * 30;
    //     }
    // };
    //
    // println!("{result_} is  ");
    //
    //
    // let condition = false;
    // let number = if condition { "5" } else { "sdfsdf" };
    //
    // println!("{number}");
    //
    // fn five(num: u32) -> u32 {
    //     num + 5
    // }
    //
    // let a = five(1);
    // println!("{a}");
    //
    //
    // // Because Rust is an expression-based language
    // let c = {
    //     let cc = 11;
    //     cc + 12
    // };
    //
    // println!("ccccccc     {c}");
    //
    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    //
    // println!("the value is {y}");
    //
    // let mut guess = String::new();
    //
    // let a: [u32; 5] = [1, 2, 3, 4, 5];
    //
    // // array should access brackets
    // let a = [3; 5];
    //
    // println!("{}", a[1]);
    //
    // let mut index = String::new();
    // // io::stdin()
    // //     .read_line(&mut index)
    // //     .expect("Failed to read line");
    // //
    // // let index: i32 = index.trim().parse().expect("Index entered was not a number");
    // // println!("{}", index);
    //
    // // for i in a {
    // //     println!("{}", i);
    // // }
    //
    // // tuple should access dot .
    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // println!("{}", x.1);
    // // io::stdin().read_line(&mut guess).expect("Failed to read line");
    //
    // println!("{}", &mut guess);
    // // Constants can be declared in any scope, including the global scope,
    // // which makes them useful for values that many parts of code need to know about.
    //
    // // const A: usize = 1;
    //
    // // shadowing
    // let x = 1;
    // // let x = x +1;
    // let x = x + 1;
    // // println!("{x}")
    //
    // // let spaces = "   ";
    // // let spaces = spaces.len();
    //
    // let z = "42".parse::<usize>().expect("Not a number!");
    // println!("{}", z)
}
