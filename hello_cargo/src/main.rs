#![allow(unused)]

// mod lib;

use std::borrow::Borrow;
use std::{io, ops};
use std::ops::Add;
use std::ptr;

const VERSION: &str = "1.0.0";

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


fn practice_option(num: i32) -> Option<i32> {
    if num == 1 {
        None
    } else {
        Some(32)
    }
}

fn to_string_(s: &str) -> String {
    String::from(s)
}

use ops::Range;


fn main() {

    // 1. vector 자료구조는 힙메모리를 사용한다.
    // 2. 벡터는 고정된 메모리 주소 크기만큼 사용하고 메모리 사이즈를 넘어서 사용하게 되면
    // 돟적으로 할당할 수 있는 메모리 주소의 크기를 늘리는 형태의 자료조구이다.
    // 3. 이 과정에서(메모리 주소의 크기를 늘려야 할 때) 메모리 전체를 새로운 주소 공간(?!)으로 옮긴다.
    // 새로운 메모리 주소 공간으로 이동(move)하게 된다면 만약 벡터의 주소를 가지고 있는 기존의 다른 변수들이
    // mut이면 상관이 없지만 const ptr인 경우에는 옮긴 주소로 변경해줄 수 없다.!!!!!
    // 위의 내용이 가장 핵심적인 부분

    //Rather than indexing using [] with a single number, you can use [] with a range to create a string slice
    // containing particualr bytes


    // 1. 벡터의 자료구조는 어떻게 생겨먹었냐?
    // 2. slice 타입에 대해서 배움
    // 3. str과 reference &str이 뭔 차이가 있는지
    // 가장 핵심적인 부분은 결국 reference를 어떻게 다루고 어떻게 러스트에서 바라보고 있는건가를 이해하는 점

    fn dangle() -> String { // dangle returns a reference to a String

        "asd".to_string()
    }

    let s: String = String::from("hellsso"); // s is a new String

    println!("{}", s.to_string());

    println!("{:p}", &s);

    println!("=========================================");
    println!("\n");

    // let reference = dangle();
    //
    // println!("{reference}");






    fn calculate_length(s: &String) -> usize {
        // These ampersands represent references, and they allow you to refer to some value without taking ownership of it. Figure 4-5 depicts this concept.
        // references를 사용한다는 것은 ownershiop을 이전하지 않고 사용하는 것을 의미하는 것 같다.
        // 그래서 main 함수에서 재사용 가능
        let length = s.len(); // len() returns the length of a String

        length
    }

    let mut s: String = String::from("hee");

    let r1 = &s;
    let r2 = &s;

    // The scopes of the immutable references r1 and r2 end after the println! where they are last used
    println!("{}, {}", r1, r2);

    // 하고자 하는 말은 이와 같은 것 같다.
    // mutable reference가 있는 이하의 코드 section부터는 mutable 변수들이 쓰이자 않으면 된다.
    let r3 = &mut s;

    println!("{r3}");

    // 핵심적인 문제는 immutable하게 참조하고 있는 r1, r2가 만약에
    // mutable하게 바뀔 수 있는 r3에서 실제로 바뀌게 된다면 문제가 발생한다.




    // mutable variable의 reference룰 여러개 만들 때의 문제는
    // 하나의 변수에서의 수정에 대한 synchronize가 되지 않는 문제를 야기하기 때문에
    // 개발자가 이런 부분을 훌륭하게 통제할 수 있다면 문제가 되지 않겠지만 러스트에서는 이런 상황을 미연에 방지하고자 하는 것 같다.

    // but immutable한 reference는 여러개를 만들어도 상관없다. 어차피 reference가 가리키는 real value의 값이 변하지 않기 때문

    // {
    //     let r1 = &mut s;
    // }
    //
    // let r2 = &mut s;
    //
    //
    // println!("{}, {}", 1, r2);

    // let len:usize = calculate_length(&s1);
    //
    // println!("The length of '{}' is {}.", s1, len);








    fn first_word(s: &String) -> &str {

        let bytes = s.as_bytes();

        for (i, &b) in bytes.iter().enumerate(){
            if b == b' '{
                return &s[0..i];
            }
        }

        &s[..]
    }

    let s = String::from("hello- world:");

    println!("{}", first_word(&s));

    // let index = Range {start: 1, end: 34 };




    let hello = &s[0..5];
    let world = &s[6..11];

    // println!("{}", &h);

    {
        let h = String::from("hello");
        let mut s:String = String::from("world");
        s.push_str(h.as_str());
        println!("{}", s);

    }

    // println!("{}", h);
    // println!("{}", h);

    // let s1 = String::from("heello");
    //
    // let bs = s1.as_bytes();
    //
    // let bb = format!("{}", bs[0]);
    //
    // println!("{bb}");
    //
    // let h = s1.chars();
    //
    // println!("{:?}", h);
    //
    // let s11 = String::from("ㄱa");
    // println!("{}", s11.len());
    // for i in s11.into_bytes() {
    //     print!("{} ", i);
    // }

    // println!("{:?}", s11.into_bytes());




    // let s1 = String::from("helo");
    // let s2 = String::from("world");
    //
    // // 잘 생각해보자
    // // add는 String s1(not reference)에 s2(&str / reference임)를 push하고 바로 push된 s1을 반환한 String이다.
    // // 그렇다면 the s1(not reference)은 ownership을 반환해서 자연스럽게 메모리 할당이 해제될 것이다.
    // let s3 = s1.clone().add(s2.as_str());
    //
    // let s4 = format!("{s1}-{s2}-{s3}");
    //
    // println!("{s4}");

    // let t = s2 + "-" + &s1 + "-" + &s3;

    // let mut s = String::from("hello");
    //
    // s.push_str(" sdf");
    //
    // println!("{s}");




    // string slices는 coded 기계어로 변환된 char들의 주소값의 연속된 형태이다.
    // let s = "string liter, string slice";
    //
    // let s = "hello";
    // let ss = "world";
    //
    // for a in ss.chars() {
    //     println!("{}", a);
    // }
    // println!("{}", s[1]);

    //
    // println!("{}", s.to_string());
    //
    // println!("{}", to_string_("this is string listeral"));





    // let v = vec![1,2,3,4,5];
    // for i in &v {
    //     println!("{}", i);
    // }
    //
    // println!("{:?}", v);
    //
    // let mut v = vec![1,2,3,4,5];
    //
    // for i in &mut v {
    //     *i += 20;
    // }
    //
    // println!("{:?}", v);

    // let mut v = vec![1,2,3,4,5];
    //
    // let third: &i32 = &v[3];
    //
    // v.push(6);
    //
    // println!("{:?}", v);

    // {
    //     let v = vec![1, 2, 3, 4, 54];
    // }
    //
    //
    // let mut v = vec![1, 2, 3, 4, 5, 6];
    //
    // let first: &i32 = &v[0];
    //
    // println!("{}", first);
    //
    // // v.push(7);
    //
    // println!("The first element is : {first}");

    // Option 열거형은 값이 있거나 없음을 나타낸다.
    // let x: Option<&str> = Some("hello");
    // let x: Option<i32> = None;
    //
    // println!("{:?}", x);
    //
    // match x {
    //     i32 => println!("32"),
    //     Option => println!("option"),
    //     None => println!("10"),
    // }
    //
    //
    //
    // let mut x: Option<i32> = None;
    //
    // x = Some(32);
    //
    // println!("{:?}", x);
    //
    // // assert_eq!(x.is_some(), false);
    //
    // let answer= practice_option(32);
    //
    // println!("{:?}", answer);
    //
    // let v = vec![1,2,3,4,5];
    //
    // // let does_not_exist: &i32 = &v[100];
    // let does_exist: Option<&i32> = v.get(100);

    //println!("{}", does_not_exist);
    // println!("{:?}", None);


    // println!("{:p}", does_not_exist);
    // println!("{}", does_not_exist);
    // println!("{:p}", &v[1]);
    // println!("{}", &v[1]);


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


    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {s2}");
    //
    // let data = "initial contents";
    // let s = data.to_string();
    //
    // let s = "initial contents".to_string();
    //
    // let s = String::from("initial contents");
    //
    //
    // // Recall that we talked about string liters being stored inside the binary. Now that we know about slices,
    // // we can properly understand string literals;
    //
    // let s = "Hello world";
    //
    // //the type of s here is &str: it's a slice pointing to that specific point of the binary.
    // // This is also why string literals are immutable; &str is an immutale reference.
    //
    //
    // let my_string = String::from("hello world");
    //
    // // first)wird works on slices of String s, whether partial of whole
    // let word = first_word(&my_string[..]);
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
