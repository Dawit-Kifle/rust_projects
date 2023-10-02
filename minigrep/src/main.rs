#![allow(unused)]
//
// #[derive(Debug)]
// enum ShirtColor {
//     Red,
//     Blue,
// }
//
// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
//
// #[derive(Debug)]
// struct UUser {
//     active: bool,
//     username: String,
//     email: &'static str,
//     sign_in_count: u64,
// }
//
//
//
// struct Color(i32, i32, i64);
//
// #[derive(Debug)]
// struct Point { x: i32, y: i32}
//
// struct AlwaysEqual;
//
// use std::env::args;
// use std::net::Shutdown::Read;
//
// fn print_point(p: &Point){
//     println!("{} {}", p.x, p.y);
// }
//
// #[derive(Debug)]
// struct Rectangle { width: u32, height: u32}
//
// impl Rectangle {
//
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//
//     fn set_width(&mut self, width: u32){
//         self.width = width
//     }
//     fn max(self, other: Rectangle) -> Rectangle {
//         Rectangle {
//             width: self.width.max(other.width),
//             height: self.height.max(other.height),
//         }
//     }
//
//     fn set_to_max(&mut self, other: Rectangle) {
//         let max = self.max(other);
//         *self = max;
//     }
//     // fn set_width(&mut self, width: u32){
//     //     self.width = width
//     // }
//     //
//     // fn area(&self) -> u32{
//     //     self.width * self.height
//     // }
//     //
//     // fn can_hold(&self, other: &Rectangle) -> bool {
//     //     self.width > other.width && self.height > other.height
//     // }
//     //
//     // fn is_width_over(self: &Self) -> bool {
//     //     self.width > 0
//     // }
//     //
//     // fn square(size: u32) -> Self {
//     //     Self {
//     //         width: size,
//     //         height: size,
//     //     }
//     // }
// }


#[derive(Debug)]
struct User {
    name: String,
    age: i32,

}

#[derive(Debug)]
struct Color (i32, i32, i32);

struct AlwaysEqual;

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle{
    // [GUIDELINE] This is associated function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // Self means the type of impl block points
    // function that has self parameter we call method
    fn area(self: &Self) -> u32{

        self.width * self.height
    }


    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle{
        Rectangle{
            width: self.width.max(other.width),
            height: self.height.max(other.height)
        }
    }

    fn set_to_max(&mut self, other: Rectangle) {
        let max = self.max(other);
        *self = max;
    }
}

#[derive(Debug)]
struct Point { x: i32, y: i32 }

impl Copy for Point { }

impl Clone for Point {

    fn clone(&self) -> Self {
        *self
    }
}

impl Point {
    fn get_x(&mut self) -> &i32{
        self.x +=23;
        &self.x
    }
}

const WIDTH: usize = size_of::<&()>();
const DOUBLE_WIDTH: usize = 2 * WIDTH;



use std::mem::{size_of, size_of_val};

use std::{fs, mem};
use std::rc::Rc;

// #[derive(Debug)]
// pub struct Range<Idx> {
//     pub start: Idx,
//     pub end: Idx,
// }

fn show(pt: &Point) {
    println!("({}, {})", pt.x, pt.y);
}

use std::env;
use std::fs::File;
use std::io::prelude::*;
//
// struct Config {
//     // In structs use owned values, such as String.
//     // In structs use owned values, such as String.
//     // In structs use owned values, such as String.
//     // In strcuts use owned values, such as String.
//     // In structs use owned values, such as String.
//     query: String,
//     file_path: String
// }
//
// impl Config {
//     // [GUIDELINE] In structs use owned values, such as String.
//     fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//             //panic!("not enough arguments");
//         }
//
//         let query = args[1].clone();
//         let file_path = args[2].clone();
//
//         Ok(Config { query, file_path })
//     }
// }
//
// fn run(config: Config) -> Result<(), Box<dyn Error>>{
//     let contents = fs::read_to_string(config.file_path)?;
//     //.expect("Should have been able to read the file");
//     println!("With text: \n{}", contents);
//     Ok(())
// }

use std::process;
use minigrep::Config;


// fn dangling() -> &i32 {
//
//     let x = 33;
//     &x
// }

// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
//     if x.len() > y.len(){
//         x
//     }
//     else{
//         y
//     }
// }

struct ImportExcerpt<'a> {
    part: &'a str
}
//
// impl ImportExcerpt {
//     fn level(&self) -> i32 {
//         3
//     }
//
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("{}", announcement);
//         self.part
//     }
// }

fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        println!("{:?} {}", item, i);
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn f<'a, 'b>(s: &'a str, s1: &'b str) -> Option<&'a str>{

    if s.len() >= s1.len(){
        Some(s)
    }else{
        None
    }
}

#[derive(Debug)]
struct S<'a> {
    first: &'a str,
    last: &'a str,
}

fn try_create(paragraph: &str) -> Option<S> {
    let mut sentences = paragraph.split('.').filter(|s| !s.is_empty());
    match (sentences.next(), sentences.next_back()){
        (Some(first), Some(last)) => Some(S { first, last}),
        (Some(first), None ) => Some(S { first, last: first} ),
        _ => None,
    }
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

fn main() {


    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);

    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    // println!("{:?}", config);

//     let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Duct tape.";
//     let query = "Rust";
//     let results = search(query, contents);
//
//     println!("{:?}", results);



    // let cont = contents.contains("rust");
    // println!("{}", contents.to_lowercase());

    // lines function feature is split with '\n' or carriage return



    // println!("{:#?}", splited_contents);
    // let s= f("ssdf213123123123123", "211111111112");
    // println!("{:?}", s);
    //
    // let o: Option<S> = try_create("djkfjkldsjkfsdjkfjsdklfj");
    //
    // println!("{:?}", o);

    // let novel = String::from("Call me Ishamel. sadfsd");
    //
    // // The function doesn't return a reference
    // // There is exactly one reference input parameter
    // // The function is a method, taking &self or &mut self as the first parameter
    //
    // first_word(&novel);
    // let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    //
    // let i = ImportExcerpt { part: first_sentence };
    //
    // println!("{:?}", first_sentence);

    // let string1 = String::from("abcd");
    // let string2 = "xyz";
    //
    // let result = longest(string1.as_str(), string2);
    //
    // println!("{}", result);




    /*
        let my_string = String::from("he");

        let word = first_word(&my_string[0..22]);
        let word = first_word(&my_string[..]);

        let word = first_word(&my_string);

     */

    //reference dangling

    // let r;
    //
    // {
    //     let x: &'static i32= &5;
    //     r = x;
    // }
    //
    // println!("{}", r);

    // let args: Vec<String> = env::args().collect();
    //
    // let config = Config::build(&args).unwrap_or_else(|err|{
    //     println!("Probelm parsing arguments: {err}");
    //     process::exit(1);
    // });
    //
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);
    //
    // if let Err(e) = minigrep::run(config){
    //     println!("Application error : {e}");
    //     process::exit(1);
    // }



    // let mut file =  File::create("poem.txt");
    // file.expect("What's wrong").write_all(b"I'm nobody! Who are you?
    //                                                 Are you nobody, too?
    //                                                 Then there's a pair of us - don't tell!
    //                                                 They'd banish us, you know.
    //
    //                                                 How dreary to be somebody!
    //                                                 How public, like a frog
    //                                                 To tell your name the livelong day
    //                                                 To an admiring bog!");

    // fn parse_configg(args: &Vec<String>){
    //     println!("{:?}", args);
    // }

    // let string_liters_slice = &args[0..2];
    // let query = &envs[1];
    // let file_path = &envs[2];

    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // println!("{}", contents);

    // let box_pt = Box::new(Point { x: 10, y: 20 });
    // let a = *box_pt;
    //
    //
    // // RefCell은 stack에 counter가 저장된다.
    // use std::cell::RefCell;
    // let rc = RefCell::new(42);
    // let b1 = rc.borrow_mut();
    // let b2 = rc.borrow_mut();
    //
    // println!("{}, {}", b1, b2);
    //
    //
    // // rc pointer type은 stack에 오로지 buffer만 가지고 heap에 reference를 비롯한 smart한 정보를 저장한다.
    // // let rangee = Range<i32>{
    // //     start: ,
    // //     end:
    // // };
    //
    //
    // let v = Rc::new(vec!["Odin".to_string(), "Loki".to_string(), "Thor".to_string()]);
    // let v_clone = v.clone();

    // [ stack frame ]
    // v => buffer of Vector of heap memory
    // v_clone => buufer of Vector of heap memory

    // [ heap memory ]
    // reference counted : 2/ Vec => buffer , len , capacity
    // Vec buffer direct to Odin String buffer , len, capacity | Loki String buffer , len, capacity Thor String buffer , len, capacity

    // "Odin" | "Loki" | "Thor"

    // let mut s = String::from("강용수 Hello KYS");
    // let s2 = s.bytes();
    // //bytess(&s);
    // println!("{:?}", s2);
    // let s1 = &s[0..3].is_ascii();
    // println!("{:?}", s1);
    //


    // let s = "용아 Kang Yongsu";
    // preallocated read-only memory space where string literals are stored
    // and string literals is collected when program starts and find reusable and duplicated string
    // the virtual memory architecture is code segment(text segment)
    // let s1 = s.chars();
    // println!("{:?}", s1);

    // argument memory
    // return address
    // stack base pointer
    // stack local variables

    // memory layout

    // env, args and so on
    // stack memory
    // heap memory
    // BSS segment
    // uninitialized global variables
    // data segment
    // The data segment contains initialized static variables, global variables, local static variables
    // text segment(code segment)
    // The text segment contains these exectuable instructions
    // text segment is read-only part


    // nested function은 .text(code) segment에 함수 선언부 그 내에 다시 선언된다.
    // In function stack frame the argument line and stack local variable line is differentiated
    // base pointer를 기준으로 위는 parameter 변수 아래는 -8 -16은 local variable이다.

    // main 함수에서 f 함수를 호출함.
    // 1. f 함수에 return address(main 함수)를 저장함.
    // 2. return address - 8에 stack base pointer(main 함수)를 저장함.
    // 3. main의 base pointer를 통해서 main 함수 내에 있는 loca varible들에 대해서 접근할 수있음.


    // let a = 222;
    //
    // let get_a = || {
    //     println!("a    : {}", a);
    // };
    //
    // get_a();
    //
    // // Rc
    // let r1 = Rc::new(vec!["Thor".to_string(), "Loki".to_string(),"Odin".to_string()]);
    // let r2 = r1.clone();
    //
    //
    //
    // // [ STACK FRAME ]
    // // let r1은 reference이다. Vec를 가리키는 Vec는 heap memory 상에 있다.
    // // let r2는 reference이다. Vec을 가리킨다. Vec은 heap memory 상에 있다.
    //
    // // [ HEAP ]
    // // Vec이 있다. Rc에 의해 할당되었기 때문에 Reference Counted라고 하는 값이 추가로 할당되어져 있다.
    // // 현재 Rc는 2이다. 왜냐하면 r2가 r1.clone을 했기 때문이다.
    // // Rc : 2 / buffer : String의 첫번째 주소를 가리킨다. / len : 3 / capacity : 3
    // // String str buffer / len / capacity || str buffer2 / len2 / capacity2 || str buffer3 / len3 / capacity3
    // // "THOR" / "LOKI" / "ODIN"
    //
    // let mut t1 = Rc::new(1);
    // t1 = Rc::new(2323);
    // println!("rcsss : {}", t1);
    //
    //
    // let a = 23;
    // let b = &a;
    // let c = &b;
    // //println!("{} {}", size_of::<i32>(), size_of::<&i32>());
    //
    // let a = [55,66,77];
    //
    // let b = &a;
    //
    // //let c = &a[0..2];
    // let s = "sssss";
    //
    // let s1 = &s[1..2];
    //
    // println!("ccc: {:?}", s1);
    //
    //
    // //println!("{:?}", size_of::<[i32;3]>());
    //
    // // This also means that you cannot store it in variables since it doesn't have a known size
    //
    // // this is &[i32] i32 slices
    // // this is &[i32] i32 slices
    // // this is &[i32] i32 slices
    //
    // // i32 slices인 이유는 array에서 특정 index의 포인터만 가지고 있기 때문에
    // // 그리고 length와 &str이 preallocated memory 상에 있는 str literal의 pointer와 length를 가지고 있듯이
    // // 혹은 String으로부터 받아온 시작점의 String heap memory의 pointer address를 가지고 있고 length를 가지고 있듯이.
    //
    // // When the next function is called, it's stack frame will overwrite this memory
    // // in a language with a garbage collector the compiler will automatically detect this and allocate that variable
    // // on the heap memory and return a reference to it, but there is some extra
    // // cost to allocating on the heap
    //
    // //assert_eq!(4, size_of::<Option<bool>>());
    //
    //
    //
    // // data length stored in type
    // // an [i32; 3] is an array of three i32s
    //
    //
    // let nums = &[22;3];
    // let a = nums[0];
    //
    // assert_eq!(WIDTH, size_of::<&[i32;3]>());
    //
    //
    // println!("{:?}", a);
    // let nums_index = &nums[1..];
    //
    // println!("{:?}", nums_index);
    // // println!("{:?}", nums);
    // let mut sum = 0;
    //
    // for num in nums{
    //     sum += num;
    // }
    //
    // println!("{}", sum);
    //
    // let nums = &[1,2,3];
    //
    // assert_eq!(DOUBLE_WIDTH, size_of::<&[i32]>());


    // slices are double-width becuase they store a pointer to the array and the number of elements in the array

    // unsized types are
    // &str / &[i32] / &dyn ToString
    // Box<dyn ToString> // &Unsized

    // let mut p = Point{ x:22, y: 23};
    // let a = Point{x:2323, y:1111};
    // p.clone_from(&a);
    //
    // println!("{:?}", p);
    // println!("{:?}", a);
    // let copy1 = p;

    //println!("{:?} {:?}", p, copy1);
    // println!("{:?} ", p);
    // p.clone_from()
    //  vector안의 타입이 primitive type이냐
    // String 혹은 (custom type or heap memory에 올라가는 type이냐)에
    // 따라서 Copy trait이냐 Clone trait이냐가 달라진다.

    // let v = vec![1,2,3];
    // // this means ownership is transferred
    // let n_ref = &v[0];
    //
    // let n = *n_ref;
    // println!("{}", n_ref);
    //
    // // case1 ownership을 아예 옮김
    // let v = vec![String::from("Hello")];
    // // let n_ref = &v[0];
    // let s = v;
    //
    // // case2 v도 heap memory 위에 있는 String을 가리키고 있고
    // // ownership도 옮아가지 않을 상태임
    // let v = vec![String::from("Hello")];
    // // 그 상태에서 heap memory 위에 있는 String의 메모리를
    // // stack에다가 옮기니 참조가 두 개가 동시에 되고 있음
    // // let n_ref = v[0];
    //
    // println!("{}", 0u32);


//     let mut n = 0;
//     let a = &mut n;
//
//     *a += 22;
// // let b = a;
//     println!("{:?}", n);
//
//     //shallow copy
//     // deep clone
//
//     let v = vec![String::from("Hello")];
//
//     let s_ref = &v[0];
//
//     let s = &*s_ref;

    // An i32 does not own heap data, them it can be copied without a move
    // A String does own heap data, so it can not be copied without a move.
    // An &String does not own heap data, so it can be copied without a move.


    //
    // let mut p = Point {
    //     x: 1,
    //     y:2
    // };
    //
    // let x = p.get_x();
    // println!("{} {}", *x, p.y);
    // // println!("{} ", p.y);
    //
    //
    //
    // // Unlike functions, methods are defined within the context of a struct
    // // and their first parameter is always self, which represents the instace of the
    // // struct the method is being called on.
    //
    // let mut rect = Rectangle {
    //     width: 0,
    //     height: 1
    // };
    //
    // let other_rect = Rectangle{
    //     width: 1,
    //     height:0
    // };
    //
    // // 말하고자 하는 바는 애초에 set_to_max에 parameter로 &가(borrow로 넘어옴)
    // // argument가 borrow인데 set_to_max
    // rect.set_to_max(other_rect);


    // 벡터를 생각한다면 벡터는 unknowsized and 정확하게 정해져 있는 사이즈를 가지고 있지 않다.
    // 벡터의 특성
    // 1. 벡터의 사이즈는 변한다.(capacity를 통해서 총 사이즈를 조절한다.)
    // 2. 벡터의 사이즈가 변할 때 기존의 메모리 space에서 다른 spacd로 이동을 한다.
    // 3. 2번의 인과 -> 따라서 다른 메모리 space로 이동할 때 기존에 참조하고 있던 stack frame의 변수가 undefined될 수가 있다.

    // let other_rect = Rectangle{ width: 1, height:1 };
    // let max_rect = rect.max(Rectangle{ width: 1, height:1 });
    //
    // println!("{:?}", max_rect);

    return;

    let r =&mut Box::new(Rectangle{
        width: 1,
        height: 2,
    });

    let mut r1 = Box::new(Rectangle{
        width: 1,
        height: 2,
    });

    r.set_width(3333);
    //let aaa = *r1;
    r1.set_width(232323);

    // r1.width
    println!("{:?}", r1.width);

    //println!("{:?}", r);
    // let area1 = r.area();
    // let area2 = Rectangle::area(&r);
    //
    // assert_eq!(area1, area2);
    // r.set_width(32);
    // println!("{} {}", area1, area2);
    // println!("{:?}", r);

    // let mut p = Point { x: 0, y: String::from("123") };
    // // temporarily lose their permissions but not p.y
    // // On struct, p.y and p.x ownership has no relationship with those each other
    // let mut x = &mut p.x;
    //
    // println!("{:?}", p.y);
    //
    // *x += 12;


    // let subject = AlwaysEqual;
    //
    // let xlet black_color = Color(0,0,0);
    //
    //
    // let mut u1 = User {
    //     name: String::from("가"),
    //     age: 333,
    //
    // };
    //
    // // partially move!!
    // let u2 = User{
    //     //name: u1.name,
    //     name: String::from("sdfsdfsdf"),
    //     // The types of active and sign_in_count are types that implement the
    //     // copy trait, so the behavior we discussed in the Copying vs Moving out of a
    //     age: u1.age
    // };
    //
    //
    // println!("{:?}", u1);
    // println!("{:?}", u2);

        // let number_list = vec![34, 50, 25, 100, 65];
    //
    // let mut largest = number_list[0];
    //
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("{}", largest);
    // two hypens to indicate the following arguments are for our program rather than for cargo,
    // a strig to search for, and a path to a file to search in, like so:
    // let args: Vec<String> = env::args().collect();
    // // dbg!(args);
    //
    // println!("{:?}", args);

    // let mut rect = Rectangle { width: 0, height: 1 };
    // let other_rect = Rectangle { width: 1, height: 0 };
    // rect.set_to_max(other_rect);
    //
    //
    // // r.max(r);
    //
    // // let r = &mut Box::new(Rectangle{
    // //     height: 2,
    // //     width: 1,
    // // });
    // //
    // // let aa = &**r;
    // //
    // // let area1 = r.area();
    // // let area2 = Rectangle::area(*r);
    // //
    // // assert_eq!(area1, area2);
    // // println!("{:?}", r.width);
    //
    // // println!("{:?}", r.height);
    // // println!("{:?}", r.width);
    // // println!("{:?}", r.width);
    //
    // // let mut r = Rectangle{
    // //     width: 1,
    // //     height: 2
    // // };
    // //
    // // let area1 = r.area();
    // // let area2 = Rectangle::area(&r);
    // // assert_eq!(area1, area2);
    // //
    // // r.set_width(2);
    // // Rectangle::set_width(&mut r, 2);
    // //
    // // println!("{:?}", r);
    //
    // //
    // // let sq = Rectangle::square(32);
    // //
    // // println!("{:?}", sq);
    // //
    // //
    // // let rect1 = Rectangle {
    // //     width: 30,
    // //     height: 50,
    // // };
    // // let rect2 = Rectangle {
    // //     width: 10,
    // //     height: 40,
    // // };
    // // let rect3 = Rectangle {
    // //     width: 60,
    // //     height: 45,
    // // };
    // //
    // // println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // // println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    //
    // // let mut rect1 = Rectangle {
    // //     width: 30,
    // //     height: 50,
    // // };
    // //
    // // // method는 해당 instance에 대해서 반드시 validation을 해야하는 것이 있다든지
    // // // 자주 쓰일(re-use)것 같은 기능들을 작성하여 함수로 만들든지 추가적인 코드를 작성할 필요없이
    // // // 쉽게 기능을 구현할 수있게 하기 위함인 듯하다.
    // // println!(
    // //     "The area of the rectangle is {} square pixels.",
    // //     rect1.area()
    // // );
    // //
    // // println!(
    // //     "The area of the rectangle' width is over? {}",
    // //     rect1.is_width_over()
    // // );
    //
    // // let scale = 2;
    // // let rect1 = Rectangle{
    // //     width: dbg!(30 * scale),
    // //     height: 50,
    // // };
    // //
    // // dbg!(&rect1);
    // //
    // //
    // // let width1 = 30;
    // // let height1 = 50;
    // //
    // // let rect1 = (30, 50);
    // //
    // // let rect2 = Rectangle {width: 30, height: 560};
    // //
    // // println!(
    // //     "The area of the rectangle is {} square pixels.",
    // //     area2(rect2)
    // // );
    // //
    // // fn area(width: u32, height: u32) -> u32{
    // //     width * height
    // // }
    // //
    // // fn area1(rect: (u32, u32)) -> u32{
    // //     rect.0 * rect.1
    // // }
    // //
    // // fn area2(rect_struct: Rectangle) -> u32{
    // //     rect_struct.width * rect_struct.height
    // // }
    // // // Similar to our discussion in "Different Tuple Fields",
    // // // Rust's borrow checker will track ownership permissions at both the struct-level and field-level.
    // // // For example, if we borrow a field x of a Point structure, then both p and p.x temporarily lose their permissions (but not p.y):
    //
    // return;
    // // x: ro / *x: rw / p.x: r
    // // let x = &mut p.x;
    // //
    // // *x += 123;
    // // //*x += 1;
    // //
    // // println!("{}", p.x);
    // //
    // // let u = UUser {
    // //     active: true,
    // //     username: String::from("강용수"),
    // //     email: "sdfsdf",
    // //     sign_in_count: 23,
    // // };
    // //
    // // println!("{:?}", u);
    // //
    // // // heap memory는 deallocate 되기 전까지 프로그램 실행시간 동안 메모리가 해제되지 않는다.
    // // // &str은? static lifetime preallocated read-only
    // // let subject = AlwaysEqual;
    // //
    // //
    // // let black = Color(0, 0, 0);
    // //
    // //
    // // let mut user1 = User {
    // //     active: true,
    // //     username: String::from("강용수"),
    // //     email: String::from("gys1120@naver.com"),
    // //     sign_in_count: 23,
    // // };
    // //
    // // let user2 = User {
    // //     active: user1.active,
    // //     username: user1.username,
    // //     email: String::from("gys1120@gmail.com"),
    // //     sign_in_count: user1.sign_in_count + 1,
    // // };
    // //
    // // let user3 = User {
    // //     ..user2
    // // };
    //
    //
    // // note that the entire instance must be mutable; Rust doesn't allow us to mark only certain fields as mutable.
    // // user1.username = String::from("11");
    //
    // // [GUIDELINE] 중요 struct update syntax를 사용하게 되면 ownership이 이동하게 된다.
    // // println!("{}", user1.username);
    //
    //
    //
    //
    // fn build_user(email: String, username: String) -> User {
    //     User {
    //         active: true,
    //         username,
    //         email,
    //         sign_in_count: 1,
    //     }
    // }
}

