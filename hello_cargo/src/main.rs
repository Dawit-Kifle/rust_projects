#![allow(unused)]

// mod lib;

use std::borrow::Borrow;
use std::{io, ops, vec};
use std::ops::{Add, AddAssign, Deref};
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
use core::alloc;
use std::fmt::{Debug, format};
use std::thread::scope;
use std::str;
use std::collections::HashMap;
use std::time::SystemTime;
use chrono::{Datelike, DateTime, format, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};

fn main() {


    // let birthday_string = dict.get_item("user_id__birthday").unwrap().
    // extract::<String>().unwrap();
    let birthday_string = "19891006";

    let now_datetime = Local::now();
    let now_date = now_datetime.date_naive();
    let birth = NaiveDate::parse_from_str(&birthday_string, "%Y%m%d").unwrap();

    println!("birth : {}, now : {}", birth, now_date);

    let mut years = now_date.year() - birth.year();


    println!("years : {}", years);

    // if now_date.month() - d.month() == 0 && 1 == 2 {
    //     println!("same");
    // }

    // println!("{} {}", now_date.day().abs_diff(d.day()));

    if !(now_date.month() >= birth.month() && now_date.day() >= birth.day()) {
        years -= 1;
    }

    println!("age : {}", years);
    println!("{} {}", now_date.month(), birth.month());
    // let mut age = now_date.years_since(d.unwrap()).unwrap();






    // let birth = "1989-10-25";
    //
    // let now_datetime = Local::now();
    // let now_date = now_datetime.date_naive();
    // let d = NaiveDate::parse_from_str("1989-10-25", "%Y-%m-%d");
    // println!("{:?}", d.unwrap().years_since(now_date));
    // println!("{:?}", now_date.years_since(d.unwrap()));
    //
    // let birth = "1989-10-25";
    // let now = Local::now().years_since();
    // let today = now.date_naive();
    //
    // println!("now : {now}\n today : {}", today.and_time());

    // println!("{}", dt.naive_utc());

    // dt.naive_utc().timestamp();
    // println!("{}", dt.naive_utc().timestamp());
    // // println!("{:?}", NaiveDate::from_ymd_opt(1989, 10, 25));
    // println!("LOCAL : {:?}", Local::now());
    // println!("UTC : {:?}", Utc::now());
    // let utc_date = Utc.with_ymd_and_hms(1989, 10, 25, 0, 0, 0).unwrap();
    // println!("{:?}", utc_date.timestamp());
    //println!("{:?}", Utc.with_ymd_and_hms(1989, 10, 25).unwrap());

    //let now = ;

    // println!("{:?}", now);

    // let text = "hello world wonderful world";
    //
    // let mut map = HashMap::new();
    //
    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }
    //
    // println!("{:?}", map);
    // let mut scores = HashMap::new();
    //
    // scores.insert(String::from("Blue"), 10);
    // //scores.insert(String::from("Yellow"), 20);
    //
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);
    // // copied를 할 경우 ownership을 가지는 새로운 heap memory struct 반환
    // // let val = scores.get("Blue").copied();
    // // // Hash map의 heap memory 위에 있는 값을 가져옴 -> & 레퍼런스가 있다.
    // // let val = scores.get("Blue");
    //
    // for (key, value) in &scores{
    //     println!("{key} {value}", );
    // }


    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    //
    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    // println!("{:#?}", val);



    // 매개변수로 포인터를 넣어도 여전히 call by value입니다.
    // 애초에 C는 call by value밖에 지원을하지않습니다.
    // (한 language에서 call by value , call by reference를 동시에 지원하는것은 불가능합니다)
    // call by reference처럼 보이는이유는 포인터로 참조를했기 떄문입니다.
    // 정확히는 포인터 그 자체가 call by value로 복사가됐는데,
    // 그 복사된포인터는 기존 포인터와 같은 object를 가르키기때문에 값이 변경되는것입니다.(call by reference이기 떄문에 값이swap되는 것이라고 착각하는일 없으시길 바랍니다.)
    // 하지만 여기에 더해 rust는 pointer에 대한 소유를 반드시 하나만 가질 수 있게끔 강제하고 있다는 점이 추가된 점이라고 생각해야 한다.

    // By deallocating the aliased data, leaving the other variable to point to deallocated memory
    // By mutating the aliased data, invalidating runtime properties expected by the other variable
    // by concurrently mutating the aliased data, causing a data race with nondeterministic behavior for the other variable.
    // string literals라고 하는 것은
    // compile time에 모든 string literal를 모으는 과정에서 중복된 string을 제거하고
    // 재사용 할 리터럴들을 고르고 골라서 preallocated 메모리 상에 할당하고 필요할 때마다 재사용을 하도록 만든다.
    // 그래서 &str은 preallocated 메모리 상에 존재하는 string literal의 주소이다.

    // strin literal의 length와 pointer를 가지고 있다.
    // let aa = "sdfsdf";
    // &str은 length를 계산하여 가지고 있는 형태이고
    // str은 preallocated 영역의 string 그 자체를 가지고 있는 느낌이다.


    // str은 preallocated space에 저장됨
    // preallocated 영역은 read-only memory 영역이고
    // lifetime이 static이다.
    // when the program starts, the string literals are collected and
    // remove duplicated string and check the string literals that we can reuse
    // 그래서 let a = "string"; 을 했을 때, type inference 되는 것이 &str이 나오는 이유는
    // static space에 존재하는 string literals의 주소를 참조하는 것이기 때문이다.

    // fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    //     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
    //     for s in src {
    //         if s.len() > largest.len() {
    //             dst.push(s.clone());
    //         }
    //     }
    // }

    return;


    let name = vec![String::from("Ferris"), String::from("dududu")];

    let first = &name[0];
    // stringify_name_with_title(&name);
    // println!("{}", first);

     fn stringify_name_with_title(name: &Vec<String>) -> String {
        let mut full = name.join(" ");
         println!("ddd {}", full);
        full.push_str(" Esq.");
        full
    }

    println!("{}", stringify_name_with_title(&name));

    // let a = ["hello", "world"];
    //
    // println!("{:?}", a);
    //
    // fn stringify_name_with_title(name: &mut Vec<String>) -> String {
    //     name.push(String::from("Esq."));
    //     let full = name.join(" ");
    //     full
    // }



    // let s = String::from("Hello World");
    // let s_ref = &s;
    //
    // let ss = return_a_string();
    //
    // fn return_a_string() -> String {
    //     let s = String::from("Hello world");
    //     // move ownership
    //     // heap memory의 address를 return 하면 heap memory의 adress가 stack frame
    //     s
    // }
    //
    // // this solution applies if we never intend to change the string, and the a heap
    // // allocation is unnecessary
    // fn return_a_string1() -> &'static str {
    //     "Hello world"
    // }
    //
    // use std::rc::Rc;
    // fn return_a_string2() -> Rc<String> {
    //     // provides shared ownership of a value of type T
    //     // allocated in the heap. Invoking clone on Rc
    //     let s = Rc::new(String::from("Hello world"));
    //     Rc::clone(&s)
    // }
    //
    //
    //
    // let mut sss = String::from("djfkdjfkjd");
    //
    // return_a_string3(&mut sss);
    //
    // println!("{sss}");
    // fn return_a_string3(output: &mut String){
    //     output.replace_range(.., "He");
    //
    // }

    // fn return_a_string2() ->  {
    //     let s: &'static   = String::from("Hello world");
    //     // move ownership
    //     // heap memory의 address를 return 하면 heap memory의 adress가 stack frame
    //     s
    // }


    // let mut x: Box<i32> = Box::new(1);
    //
    // // *x -> 자체가 heap value의 ownership 그 자체다.
    //
    // let a = *x;         // *x reads the heap value, so a = 1
    //
    //
    // // modify heap value
    // // modify heap value
    // // modify heap value
    // // modify heap value
    // *x += 3;                 // *x on the left-side modifies the heap value,
    //                          //     so x points to the value 2
    //
    // let r1: &Box<i32> = &x;
    // let b= **r1;
    //
    // let mut r2 = &mut *x;
    // println!("{r2}");
    // // let zz = **r2;
    // *r2 += 1;
    //
    // println!("{a}");

    return;
//     // a는 box의 1을 가리킴
//     // call by value -> ro
//     // call by value call by value call by value call by value
//     // primitive type int가 call by value됨
//
//     // let a: i32 = &x;
//     // // rw
//     // *x += 1;
//     // let z = &x;
//     // let zz = &*x;
//     // println!("{}", zz+1);
//     // let q = x.as_ref();
//     // let a = x.as_mut();
//     // a.add_assign(123);
//     // println!("{}", x);
//
//     // call by value
//
//
//     // let r1: &Box<i32> = &x;
//     // let b: i32 = **r1;
//     //
//     // let r2: &i32 = &*x;
//     // let c: i32 = *r2;
//
//     // let m1 = String::from("Hello");
//     // let m2 = String::from("world");
//     // greet(&m1, &m2); // note the ampersands
//     // let s = format!("{} {}", m1, m2);
//     //
//     // fn greet(g1: &String, g2: &String) { // note the ampersands
//     //     println!("{} {}!", g1, g2);
//     // }
//
//
//     // let mut x = "hey";
//     // // str 그 자체를 참조하고 있으면 str은 preallocated 영역에서 unsized이기 때문에 컴파일 타임에 사이즈를 알 수 없다.
//     // let a = *x;
//     //
//     // println!("{a}");
//     //
//     // let mut x: &'static [u8] = b"hey";
//
//
//     // string literal가 unsized이며 반드시 length를 알아야 하는 이유는
//     // string 자체가 ascii 코드는 1byte이고 utf-8은 3byte이기 때문에
//     // int floating number boolean char와 같이 명확하게 type의 사이즈를 알 수 없다는 특징을 가지고 있다.
//     // 그래서 unsized type인 str은 반드시 length를 알아야 한다.
//
//
//     // string literal을 heap memory에 올리고
//     // 그 시작 pointer를 가지고 있으면 len과 capacity가 stack frame에 저장되어 있다.
//     // let noodles = "noodles".to_string();
//     //
//     // // String의 주소값을 가지고 있는 stack의 noodles의 메모리를 가지고 있는 oodles
//     // // ownership이 oodles로 넘어감.
//     // let oodles = &noodles;
//     //
//     // // string slice이며 noodles의 1번 인덱스부터 끝까지를 가지고 있는 oodles1이고 len만 가지고 있다.
//     // let oodles1 = &noodles[1..];
//     //
//     // // static [u8]이며 preallocated 영역의 값을 참조하고 있는 변수이다.
//     // // len만 가지고 있다.
//     // let poodles = "poodles";
//     //
//     // let s: &'static [u8] = b"hey";
//     //
//     // let mut x: &'static [u8] = b"hey";
//
//
//
//
//
//
//
//
//     let strings: Vec<String> = vec![];
//     let default = String::from("default");
//
//     // println!("{:?}",strings);
//
//     // fn first_or(strings: &Vec<String>, default: &String) -> &String {
//     //
//     //     if strings.len() > 0 {
//     //         &strings[0]
//     //     } else {
//     //         default
//     //     }
//     //
//     // }
//
//     return;
//
//
//     // [GUIDELINE] 중요!!! -> String이 index로 바로 접근하지 못하게 하는 이유 =>  because UTF-8 is a variable-length encoding
//     //   If you are certain that your strings contain ascii characters only, you can yse
//     //   as_bytes() method on &str which returns a byte slice, and then index into this slice
//     let s = String::from("Hello world");
//     // vec<i32>의 경우는 i32이라는 size를 명확히 알고 있기 때문에 index로 access하고
//     // 추가적로으 접근하는 것(이유는 어차피 4bytes씩 이동하면 다음 index로 접근할 수 있으니까)
//     // 하지만 String은 아스키코드냐 utf-8이냐에 따라서 이동해야 하는 byte가 달라지기 때문에 쉽게 접근할 수 없게 만들어 놓은 것으로 추측된다.
//     let s_ref = &s;
//
//     println!("{s_ref}");
//     fn first(strings: &Vec<String>) -> &String {
//         let s_ref = &strings[0];
//         s_ref
//     }
//
//
//     return;
//     // fn ascii_capitalize(v: &mut Vec<char>) {
//     //     // v: rwo
//     //
//     //     // c: ro
//     //     // v: r
//     //     // c*: r
//     //     let c = &v[0];
//     //
//     //     if c.is_ascii_lowercase(){
//     //         let up = c.to_ascii_uppercase();
//     //
//     //         v[0] = up;
//     //
//     //         println!("{:?}", v);
//     //     }else{
//     //         println!("Already capitalized: {:?}", v);
//     //
//     //     }
//     // }
//     //
//     // let mut str = Vec::new();
//     //
//     // str.push('c');
//     //
//     // ascii_capitalize(&mut str);
//
//     return;
//
//
//
//
//
//
//     // x : rwo
//     // let mut x = 1;
//     // // y: ro | x: r | *x: r
//     // let y = &x;
//     // // z는 1일 것인데 call by value로 1의 값이 복사된 듯하다.
//     // // z: ro
//     // let z = *y;
//     // x += z;
//     //
//     // println!("{x}");
//
//     return;
//     let mut v = vec![1,2,3];
//     // num has ro permission
//     // *num has read and write permission
//     let num = &mut v[2];
//
//     let num2 = &*num;
//
//     println!("{} {}", *num, *num2);
//     return;
//     // [GUIDELINE] 중요
//     //  Creating a reference to data causes that data to be temporarily read-only until the reference is
//     //  no longer used.
//
//     let mut v = vec![1,2,3];
//     // num gets +ro permissions
//
//     // 전체 벡터의 index 일부만 borrow가 일어나도 소유권은 완전히 move된다.
//     let num = &mut v[1];
//
//     // *num gets read and write permissions
//     // num = &mut v[0];
//     *num += 11;
//
//
//     println!("{num}");
//
//     println!("{:?}", v);
//     return;
//     let mut v = vec![1,2,3];
//     let num = &v[2];
//
//     let a = *num;
//     println!("{a}");
//
//     v.push(23);
//     return;
//
//     let mut v = vec![1,2,3];
//     let num = &v[2];
//
//     println!("{}", *num);
//     println!("{}", *num);
//
//     v.push(22);
//     return;
//
//     // paths
//     // Variable, like a.
//     // Dereferences of paths, like *a
//     // Array acdesses of paths, like a[0]
//     // Fields of paths, like a.0 for tuples or a.field for structs
//     // Any combination of the above like *((*a)[0].1).
//
//     let mut x = 0;
//     let mut x_ref = &mut x;
//
//     *x_ref +=11;
//     println!("{}", x_ref);
//     return;
//
//     // 하지만 string이라는 것은 결국 char[u8]의 array에 불과하고 string은 정확하게
//     // we don't know how many bytes has on compile time
//     let mut v = vec![1,2,3];
//     // heap memory의 1번 인덱스의 2 address를 참조하고 있다.
//     // owner is num
//     // the data in v has been borrowed by num.
//     let mut num = &v[1];
//
//     println!("{:?}", v);
//
//     println!("{}", num);
//     v[0] = 11;
//     // no longer num is used, so the ownership is moved to v
//     // println!("{}", *num);
//
//     v.push(3);
//     println!("{:?}", v);
//     return;
//
//     let mut v = vec![1,2,3]; // length, capacity, pointer
//     // [GUIDELINE] 중요 vec의 element에 대해서 하나의 address의 ownership만 넘길 수 없다.
//     //   이 의미인 것 같다.
//     //   heap의 2의 주소값을 가지고 있는 것이 맞다.
//     let num = &v[1]; // length, pointer
//
//     // [GUIDELINE] 중요 num은 현재 stack 1 메모리를 참조하는 중
//     //  엄밀히 말하면 stack 1의 메모리에
//     //  진짜 중요!!!!!!!!!!!!!!!
//     //     size = 1 + < 4*(index + 1) > 만큼을 stack 1메모리에서 이동을 여하
//     //   2 value를 가지고 있게 될건데 결국에는 stack 메모리를 참조하고 있었지만
//     //   push에 의해서 stack1이 참조하고 있는 heap 메모리의 주소가 변경이 될 때
//     //   num이 가리키던 heap이 변경이 일어나서 undefined behavior가 일어난다.
//
//     let mut v = vec![1,2,3];
//     // heap 메모리의 1-index의 address를 참조하고 있는 중
//     let h = &v[1];
//
//     // [GUIDELINE] 중요 push가 일어났을 때, capacity가 length와 같으면 새로운 space로 이사가고 기존의 주소는
//     //  파기된다. 그렇다면 h는 undefined behavior이 일어나게 된다.
//     v.push(23);
//     println!("{}", *num);
//
//     // println!("{:?}", v);
//
//
//     // let mut s = String::from("hello");
//     //
//     // let ss = &mut s;
//
//     // this means string literal is stored on preallocated memory
//     // and preallocated memory space is located on read-only data segment
//     // read-only code segment collect all string literals when this program runs
//     // and remove duplicates and check literals that we can reuse
//     // &str is memory and length
//     // let element = " world";
//
//     // ss.push_str(element);
//     // println!("{}", ss);
//     // println!("{}", s);
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//     // note: all local variables must have a statically known size
//     // help: unsized locals are gated as an unstable feature
//     // note: all local variables must have a statically know size
//     // all local variables must have a statically known size means that
//     // int have a 4 bytes size and boolean has 1 byte size
//     // and char has a 1 byte size and floating number has a 8 bytes size
//
//     // 우리는 컴파일 타임에 스택 변수들의 사이즈를 알고 있어야 한다.
//     // 최소 length or type 그 자체가 가질 수 있는 memory bytes
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//     let x = Box::new(-1);
//     let x_abs1 = i32::abs(*x);
//     let x_abs2 = x.abs();
//
//     // [GUIDELINE] 증요 -> str을 사용하면 안됨
//     // note all local variables must have a statically known size
//     // help: unsized locals are gated as an unstable features
//     // note: all local variables must have a statically know size
//
//     let r = &x;
//     let r_abs1 = i32::abs(**r);
//     // [GUIDELINE] 증요 -> stack을 받은 것이기 때문에 스택을 가지고 할 수있는 것들을 그냥 하면 된다.
//     //   그리고 heap의 소유권은 없이 그냥 stack을 사용하고 있는 것뿐.
//     let r_abs2 = r.abs();
//
//     let s = String::from("hello");
//     let s_len1 = str::len(s.as_str());
//     let s_len2 = s.len();
//
//     // println!("{x_abs1} {x_abs2}");
//     // println!("{r_abs1} {r_abs2}");
//     // println!("{s_len1} {s_len2}");
//
//     // reads the heap value directly
//     // and Box
//
//     // println!("{}", a);
//     // return;
//     // let mut x = Box::new(1);
//     // let a = *x;
//     // *x += a;
//     //
//     // // x stack memory의 address
//     // let r1 = &x;
//     // let b = **r1;
//     //
//     // *x += b;
//     //
//     // // Box memory를 가리키는 것이 아니라 stack 메모리를 가리킨다.
//     // // Box 메모리를 가리키는 것이 아니라 스택 메모리를 가리킨다.
//     // // Box 메모리를 가리키는 것이 아니라 스택 메모리를 가리킨다.
//     // // 명시적으로 Box와 같이 표시가 되지 않으면 그것은 borrow 즉 스택 메모리를 가리키는 것이다.
//     //
//     // let r2 = &*x;
//     // // let c = *r2;
//     // drop(r2);
//     //
//     // println!("{}", r2);
//     // println!("{}", x);
//     // println!("{x}");
//     // println!("{c}");
//     // let first = String::from("Fe");
//     // let full = add_suffix(first);
//     //
//     // fn add_suffix(mut name: String) -> String{
//     //     name.push_str(" Jr.");
//     //     name
//     // }
//     //
//     // let a = "Ferris";
//     // let b = &"Ferrisa";
//     // let c = &&"Ferrisa";
//     // println!("{:p}", a);
//     // println!("{:p}", b);
//     // println!("{:p}", c);
//     // println!("{:p}", "Ferris");
//
//     // hey string literals is allocated to preallocated memory space(code segment)
//     // and x have a pointer to hey string literals'address and length on stack frame
//     let mut x = "hey";
//     // world is allocated on rumtime(execution of program when the program starts
//     // the world sting literal and all of the string literals have been allocated to preallocated memory space)
//     x = "world";
//
//     // how does that work that there already is a string "world" in read-only memory available at the start
//     // of the program? answer: during compilation, the compiler collects all string literals appearing in your entire program source
//     // possibly skipping the ones that are known to be never used, and possibly removing duplicates, and smae data in the same place in read-only
//     // memory can be re-used again.
//
//     //loaded into memory together with the program code when you start
//
//     // they are there for the whole duration of the program and they stay
//     // there forever.
//
//     // let mut a = 5;
//     // let b = a;
//     // a += 1;
//     //
//     // println!("{a}");
//     //
//     // let a_num = 4;
//     // make_and_drop();
//     //
//     // fn make_and_drop(){
//     //     let a_box = Box::new(5);
//     // }
//
//     // of the box from a to b
//     // // to avoid this situation, we finally arrive at ownership
//     // When a is bound to Box::new; we say that a owns the box. The
//     // statement let b = a moves ownership
//     // let a = Box::new([0; 10]);
//     // let b = &a;
//     //
//     // println!("{:?}", a);
//     // println!("{:?}", b);
//
//     // let n = 5;
//     // let y = plus_one(n);
//     // println!("The value of y is: {y}");
//     //
//     // fn plus_one(x: i32) -> i32 {
//     //     x + 1
//     // }
//
//
//
//
//
// //
// // let mut v: Vec<i32> = vec![1, 2, 3];
// // let num: &mut i32 = &mut v[2];
// //
// // println!("Third element is {}", *num);
// // println!("Again, the third element is {}", *num);
// // v.push(4);
//
//
//
//     // Creating a reference to data causes that data to be temporarily read-only until the refernece is nolonger used.
//     // creating a reference to data causes that data to be temporarily read-only until the reference is no longer used.
//
//     // the vector has to create a new allocation with larger capacity, copy all the elements over, and deallocate the original heap array.
//
//     //
//     // return;
//     // //
//     // // let s = String::from("Hello");
//     // // let ln1 = s.len();
//     // //
//     // // let aaaa = &s[1..3];
//     // // let aaaa11 = &s;
//     // //
//     // // println!("{}", aaaa);
//     //
//     //
//     // // let a = *s;
//     // let a = &s;
//     // let a = &s;
//     // println!("{:?}", s);
//     // // println!("{:?}", str::len(*s));
//     // println!("{:?}", str::len(s.deref()));
//     // println!("-=--------------------");
//     //
//     //
//     // //let ln2 = str::len();
//     //
//     //
//     // let x = Box::new(-1);
//     // let x_abs1 = i32::abs(*x);
//     // let x_abs2 = x.abs();
//     //
//     // assert_eq!(x_abs1, x_abs2);
//     //
//     // let r = &x;
//     // // double reference means i get real value of heap memory
//     // let r_abs1 = i32::abs(**r);// explicit dereference
//     // // r 그자체가 stack의 메모리이기 때문에
//     // let r_abs2 = r.abs();
//     //
//     // println!("{:p},,,, {:p}", &x, r);
//     // println!("{:p}", &*x);
//     //
//     //
//     //
//     //
//     //
//     // let mut x: Box<i32> = Box::new(1);
//     // // call by value
//     // // call by value란 매개변수에 값을 넘길 때, 넘기는 값의 메모리 주소가 아닌
//     // // 실제 값을 복사해서 넘기는 것을 말한다.
//     // // 즉 reference의 value를 stack frame에 직적 copy하는 것을 의미한다.
//     // let a = *x;
//     //
//     // *x +=1;
//     //
//     //
//     // println!("{x}");
//     //
//     // // r1 is pointer of x(heap memory[Box]) in other words
//     // // r1 is memory address
//     //
//     // // r1 points to x on the stack
//     // // 스택에 있는 주소를 받았다고 생각하면 됨
//     // // r1 points to x on the stack
//     // // r1 points to x on the stack
//     // // r1 points to x on the stack
//     // // r1 points to x on the stack
//     // // r1 points to x on the stack
//     // // r1 points to x on the stack
//     // // r1 points to x on the stack
//     //
//     // let r1 = &x;
//     // // r1 is heap memory reference and if we wanna get the value of heap memory
//     // // ** double deference
//     // let b = **r1;
//     //
//     //
//     // // r2 points to the heap value directly
//     // // this means call by value
//     // // x가 가리키는 box의 힙 메모리에 있는 value를 현재의 main stack에 복사해온 값
//     // let r2 = *x;
//     // // x의 deferencing 즉 heap 메모리의 2값을 가리키고 그 2의 메모리& address를 직접 참조하게
//     // // 만드는 코드
//     // let r2 = &*x;
//     //
//     // println!("{r2}");
//     //
//     // // println!("{r1}, {b}, {r2}, {c}");
//     //
//     //
//     // // let mut x: i32 = x.deref() + 1;
//     // //
//     // // fn de(reff: Box<i32>) -> i32 {
//     // //
//     // //     unsafe {
//     // //         *reff + 2
//     // //     }
//     // // }
//     // //
//     // // let mut x: Box<i32> = Box::new(1);
//     // //
//     // // let a = de(x);
//     // //
//     // //
//     // //
//     // //
//     // // println!("{a}");
//     // // //println!("{x}");
//     //
//     // //
//     // // let m1 = String::from("Hello");
//     // // let m2 = String::from("World");
//     // //
//     // // // reference를 parameter로 넘긴다는 것은 stack 메
//     // // greet(&m1, &m2);
//     // //
//     // // let s = format!("{} {}", m1, m2);
//     // // fn greet(g1: &String, g2: &String) {
//     // //     println!("{} {}", g1, g2);
//     // //
//     // // }
//     //
//     //
//     //
//     //
//     // // let s = String::from("h");
//     // //
//     // //
//     // // let mut s1: String;
//     // // // s1 = &s;
//     // // // let s2 = String::from("12312321");
//     // // // because of this allocation the original String s is dropped
//     // // s1 = String::from("12312321");
//     // // s1 = String::from("sdfsdfsdfsdfsdf");
//     // //
//     // // println!("{s1}");
//     // // println!("{s}");
//     //
//     //
//     // // [GUIDELINE] 중요!!!!!!!!!
//     // // since the original String isn't saved to a variable anywhere
//     // // since the original String isn't saved to a variable anywhere
//     // // it would be immediately freed and the reference would be invalid.
//     // // it would be immediately freed and the reference would be invalid
//     // // If you don't put value into a variable, it gets dropped
//     // // If you don't put value into a variable, it gets dropped
//     // // But if you put result of to_string into some temprrary variable, you might
//     // // But if you put result of to_string into some temporary variable, you might
//     // // in your case second error that did_config outlvies that
//     // // in your case second error that did_confg
//     // // dbg!(temoporary)Simplest solution would be to make did_config a String.
//     // // If you don't put value into a variable, it gets dropped
//     //
//     //
//     //
//     // // 1. vector 자료구조는 힙메모리를 사용한다.
//     // // 2. 벡터는 고정된 메모리 주소 크기만큼 사용하고 메모리 사이즈를 넘어서 사용하게 되면
//     // // 돟적으로 할당할 수 있는 메모리 주소의 크기를 늘리는 형태의 자료조구이다.
//     // // 3. 이 과정에서(메모리 주소의 크기를 늘려야 할 때) 메모리 전체를 새로운 주소 공간(?!)으로 옮긴다.
//     // // 새로운 메모리 주소 공간으로 이동(move)하게 된다면 만약 벡터의 주소를 가지고 있는 기존의 다른 변수들이
//     // // mut이면 상관이 없지만 const ptr인 경우에는 옮긴 주소로 변경해줄 수 없다.!!!!!
//     // // 위의 내용이 가장 핵심적인 부분
//     //
//     // //Rather than indexing using [] with a single number, you can use [] with a range to create a string slice
//     // // containing particualr bytes
//     //
//     //
//     // // 1. 벡터의 자료구조는 어떻게 생겨먹었냐?
//     // // 2. slice 타입에 대해서 배움
//     // // 3. str과 reference &str이 뭔 차이가 있는지
//     // // 가장 핵심적인 부분은 결국 reference를 어떻게 다루고 어떻게 러스트에서 바라보고 있는건가를 이해하는 점
//     //
//     // // because integers are simple values with a known, fixed size, and these two 5 values
//     // // are pushed onto the stack.
//     //
//     //
//     // // str -> string literals -> read-only data segment
//     // // String -> heap allocated string -> heap memory
//     // //
//     // // String -> heap memory and Vector<u8> 따라서
//     // // buffer, capacity, length를 가지고 있다.
//     //
//     // // but literal string은 buffer, len을 가지고 있다.
//     //
//     // // A reference is a nonowning pointer type that references another value in memory.
//     // // &
//     //
//     // // string slices is a reference to a substring of that data and therefore also  points at the memory on the heap
//     // // string slices는 heap memory에 이미 할당된 Vec의 reference를 가지고 오는 것이기 때문에 반드시 length를 명시하고 가져와야 한다.
//     //
//     // // The boxed array has now been bound to both a and b. by our almost correct principle
//     // // rust would try to free the box's heap memory twice on behalf of both variables.
//     // // That's undefined behavior too.
//     // // To avoid this situation, we finally arrive at ownership
//     // // When a is bound to Box::new we say that a owns the box. the statement let b = a moves ownership of the box
//     // // from a to b.
//     //
//     // // fn main() {
//     // //
//     // //     let first = String::from("Ferris"); first is stack memory and String::from("Ferris") is converted string literals to heap memory
//     // //     and first own the "Ferris" string memory address
//     // //
//     // //     new stack frame add_suffix is created in stack memory
//     // //     let full = add_suffix(first);
//     // //     full owns the "Ferris Jr." string memory address
//     // //     println!("{full}");
//     // // }
//     // //               parameter name is defined mutably and has String. it means name own the String memory address
//     // // fn add_suffix(mut name: String) -> String {
//     // //
//     // //     name.push_str(" Jr.");
//     // //     name
//     // //     then name owns the "Ferris Jr." string memory address
//     // //     and when add_suffix stack frame is deallocated on stack
//     // //     the name variable is deallocated too on stack so name variable is dropped
//     // // }
//     //
//     // // unsafe {
//     // //     let my_num: Box<i32> = Box::new(10);
//     // //     let my_num_ptr = my_num;
//     // //
//     // //     println!("{:?}", my_num_ptr);
//     // //     println!("{:?}", *my_num_ptr);
//     // //     println!("{}", my_num);
//     // // }
//     //
//     //
//     //
//     //
//     // // let mut first = String::from("Ferris");
//     // // first.as_ptr();
//     // //
//     // // println!("{first}");
//     // // first = String::from("Ferris Jr.");
//     // // println!("{first}");
//     //
//     // // let full = add_suffix(first);
//     // // println!("{full} ");
//     // //
//     // // fn add_suffix(mut name: String) -> String {
//     // //     name.push_str(" Jr.");
//     // //     name
//     // // }
//     // //
//     // // All heap data must be owned by exactly one variable.
//     // // Rust deallocates heap data once its owner goes out of scope
//     // // Ownership can be transferred by moves, which happen on assignment
//     // // Heap data can only be accessed through its current owner
//     //
//     // // let s2: i32;
//     // // let s = String::from("hello");
//     // //
//     // //
//     // // // heap memory
//     // // // Ferris Jr.
//     // //
//     // //
//     // //
//     // //
//     // //
//     // //
//     // // let mut a = 32;
//     // // // !!!값 복사가 일어남!!! 값 복사가 일어남 값 복사가 일어남 값 복사가 일어남
//     // // let b = a;
//     // //
//     // // println!("{}", b);
//     // //
//     // // a = 11;
//     // //
//     // // println!("{}", b);
//     // //
//     // //
//     // // let my_name = &"Pascal Precht".to_string();
//     // //
//     // // println!("{}", my_name);
//     // // // 주소값을 그대로 사용할순 없다. 주소값을 가지고 와서 다시 slice 해서 사용해야 한다.
//     // // let last_name = &my_name[7..];
//     // //
//     // // let s = "sdfsdf";
//     // //
//     // //
//     // //
//     // // // & reference는 결국 시작 주소값을 가지고 와서 stack frame에 저장하는 것이다.
//     // //
//     // //
//     // //
//     // //
//     // // let story = "Once upon a time...";
//     // //
//     // // let ptr = story.as_ptr();
//     // // let len = story.len();
//     // //
//     // // println!("{:p}, {}", ptr, len);
//     // // println!("{:p}", story);
//     //
//     // // unsafe {
//     // //     let x = 5;
//     // //     println!("{:p}", x);
//     // //     let raw: *const i32 = &x;
//     // //     println!("{:p}", raw);
//     // //
//     // //     let y = x;
//     // //     println!("{:p}", &y);
//     // //
//     // // }
//     //
//     // // this is indeed what is happening, because integers are simple values with a known, fixed size,
//     // // and these two 5 values are pushed onto the stack.
//     //
//     // // but let x = String::from("hellow"); statement는 다르다.
//     // // 왜냐하면 String은 heap memory에 올라가기 때문에 그리고 String은 flexible한 size를 가지고 있기 때문이다.
//     //
//     //
//     // // let s = String::from("dsf");
//     // //
//     // // // s = String::from("sdf");
//     // //
//     // // println!("{s}");
//     // //
//     // //
//     // //
//     // // struct User {
//     // //     name: String,
//     // //     age: u32,
//     // //     email: String
//     // // }
//     // //
//     // // let mut user2 = User {
//     // //     name: "강정수".to_string(),
//     // //     age: 29,
//     // //     email: "kysdf@gmail.com".to_string()
//     // // };
//     // //
//     // // user2.email = "aaaaa@naver.com".to_string();
//     // //
//     // // println!("{:?}", user2.email);
//     // //
//     // //
//     // // let user1 = User {
//     // //     name: "강용수".to_string(),
//     // //     age: 33,
//     // //     email: String::from("asdf")
//     // // };
//     // //
//     // // println!("{}", user1.name);
//     // //
//     // // fn build_user(email: String, name: String) -> User {
//     // //     // parameter name과 struct의 filed name이 같을 때는,
//     // //     // 할당하는 과정 생략 가능
//     // //     User {
//     // //         email,
//     // //         name,
//     // //         age: 32
//     // //     }
//     // // }
//     // //
//     // //
//     // //
//     // // let s= String::from("hellsso"); // s is a new String
//     // //
//     // //
//     // // // u8 type의 벡터
//     // // let v: Vec<u8> = Vec::new();
//     // // let v = vec![13,4,3,3,];
//     // //
//     // // println!("{:?}", v[0]);
//     // //
//     // // let num = v.len();
//     // // let pp = v.as_ptr();
//     // //
//     // // unsafe {
//     // //     println!("{:p}", pp.add(2));
//     // //
//     // // }
//     //
//     // // println!("{}", v.capacity());
//     // // println!("{num}");
//     //
//     // // fn dangle() -> String { // dangle returns a reference to a String
//     // //
//     // //     "asd".to_string()
//     // // }
//     // //
//     // //
//     // //
//     //
//     // //
//     // // // s = String::from("hello")의 memory address 그자체
//     // //
//     // // // &s는 String::from("hello")의 memory address를 담고 있는 stack 변수 s 그자체의 memory address
//     // //
//     // // println!("{}", s.to_string());
//     // //
//     // // println!("{:p}", &s);
//     // //
//     // // println!("=========================================");
//     // // println!("\n");
//     // //
//     // // // let reference = dangle();
//     // // //
//     // // // println!("{reference}");
//     // //
//     // //
//     // //
//     // //
//     // //
//     // //
//     // // fn calculate_length(s: &String) -> usize {
//     // //     // These ampersands represent references, and they allow you to refer to some value without taking ownership of it. Figure 4-5 depicts this concept.
//     // //     // references를 사용한다는 것은 ownershiop을 이전하지 않고 사용하는 것을 의미하는 것 같다.
//     // //     // 그래서 main 함수에서 재사용 가능
//     // //     let length = s.len(); // len() returns the length of a String
//     // //
//     // //     length
//     // // }
//     // //
//     // // let mut s: String = String::from("hee");
//     // //
//     // // let r1 = &s;
//     // // let r2 = &s;
//     // //
//     // // // The scopes of the immutable references r1 and r2 end after the println! where they are last used
//     // // println!("{}, {}", r1, r2);
//     // //
//     // // // 하고자 하는 말은 이와 같은 것 같다.
//     // // // mutable reference가 있는 이하의 코드 section부터는 mutable 변수들이 쓰이자 않으면 된다.
//     // // let r3 = &mut s;
//     // //
//     // // println!("{r3}");
//     //
//     // // 핵심적인 문제는 immutable하게 참조하고 있는 r1, r2가 만약에
//     // // mutable하게 바뀔 수 있는 r3에서 실제로 바뀌게 된다면 문제가 발생한다.
//     //
//     //
//     //
//     //
//     // // mutable variable의 reference룰 여러개 만들 때의 문제는
//     // // 하나의 변수에서의 수정에 대한 synchronize가 되지 않는 문제를 야기하기 때문에
//     // // 개발자가 이런 부분을 훌륭하게 통제할 수 있다면 문제가 되지 않겠지만 러스트에서는 이런 상황을 미연에 방지하고자 하는 것 같다.
//     //
//     // // but immutable한 reference는 여러개를 만들어도 상관없다. 어차피 reference가 가리키는 real value의 값이 변하지 않기 때문
//     //
//     // // {
//     // //     let r1 = &mut s;
//     // // }
//     // //
//     // // let r2 = &mut s;
//     // //
//     // //
//     // // println!("{}, {}", 1, r2);
//     //
//     // // let len:usize = calculate_length(&s1);
//     // //
//     // // println!("The length of '{}' is {}.", s1, len);
//     //
//     //
//     //
//     //
//     //
//     //
//     // //
//     // //
//     // // fn first_word(s: &String) -> &str {
//     // //
//     // //     let bytes = s.as_bytes();
//     // //
//     // //     for (i, &b) in bytes.iter().enumerate(){
//     // //         if b == b' '{
//     // //             return &s[0..i];
//     // //         }
//     // //     }
//     // //
//     // //     &s[..]
//     // // }
//     // //
//     // // let s = String::from("hello- world:");
//     // //
//     // // println!("{}", first_word(&s));
//     // //
//     // // // let index = Range {start: 1, end: 34 };
//     // //
//     // //
//     // //
//     // //
//     // // let hello = &s[0..5];
//     // // let world = &s[6..11];
//     //
//     // // println!("{}", &h);
//     //
//     // // {
//     // //     let h = String::from("hello");
//     // //     let mut s:String = String::from("world");
//     // //     s.push_str(h.as_str());
//     // //     println!("{}", s);
//     // //
//     // // }
//     //
//     // // println!("{}", h);
//     // // println!("{}", h);
//     //
//     // // let s1 = String::from("heello");
//     // //
//     // // let bs = s1.as_bytes();
//     // //
//     // // let bb = format!("{}", bs[0]);
//     // //
//     // // println!("{bb}");
//     // //
//     // // let h = s1.chars();
//     // //
//     // // println!("{:?}", h);
//     // //
//     // // let s11 = String::from("ㄱa");
//     // // println!("{}", s11.len());
//     // // for i in s11.into_bytes() {
//     // //     print!("{} ", i);
//     // // }
//     //
//     // // println!("{:?}", s11.into_bytes());
//     //
//     //
//     //
//     //
//     // // let s1 = String::from("helo");
//     // // let s2 = String::from("world");
//     // //
//     // // // 잘 생각해보자
//     // // // add는 String s1(not reference)에 s2(&str / reference임)를 push하고 바로 push된 s1을 반환한 String이다.
//     // // // 그렇다면 the s1(not reference)은 ownership을 반환해서 자연스럽게 메모리 할당이 해제될 것이다.
//     // // let s3 = s1.clone().add(s2.as_str());
//     // //
//     // // let s4 = format!("{s1}-{s2}-{s3}");
//     // //
//     // // println!("{s4}");
//     //
//     // // let t = s2 + "-" + &s1 + "-" + &s3;
//     //
//     // // let mut s = String::from("hello");
//     // //
//     // // s.push_str(" sdf");
//     // //
//     // // println!("{s}");
//     //
//     //
//     //
//     //
//     // // string slices는 coded 기계어로 변환된 char들의 주소값의 연속된 형태이다.
//     // // let s = "string liter, string slice";
//     // //
//     // // let s = "hello";
//     // // let ss = "world";
//     // //
//     // // for a in ss.chars() {
//     // //     println!("{}", a);
//     // // }
//     // // println!("{}", s[1]);
//     //
//     // //
//     // // println!("{}", s.to_string());
//     // //
//     // // println!("{}", to_string_("this is string listeral"));
//     //
//     //
//     //
//     //
//     //
//     // // let v = vec![1,2,3,4,5];
//     // // for i in &v {
//     // //     println!("{}", i);
//     // // }
//     // //
//     // // println!("{:?}", v);
//     // //
//     // // let mut v = vec![1,2,3,4,5];
//     // //
//     // // for i in &mut v {
//     // //     *i += 20;
//     // // }
//     // //
//     // // println!("{:?}", v);
//     //
//     // // let mut v = vec![1,2,3,4,5];
//     // //
//     // // let third: &i32 = &v[3];
//     // //
//     // // v.push(6);
//     // //
//     // // println!("{:?}", v);
//     //
//     // // {
//     // //     let v = vec![1, 2, 3, 4, 54];
//     // // }
//     // //
//     // //
//     // // let mut v = vec![1, 2, 3, 4, 5, 6];
//     // //
//     // // let first: &i32 = &v[0];
//     // //
//     // // println!("{}", first);
//     // //
//     // // // v.push(7);
//     // //
//     // // println!("The first element is : {first}");
//     //
//     // // Option 열거형은 값이 있거나 없음을 나타낸다.
//     // // let x: Option<&str> = Some("hello");
//     // // let x: Option<i32> = None;
//     // //
//     // // println!("{:?}", x);
//     // //
//     // // match x {
//     // //     i32 => println!("32"),
//     // //     Option => println!("option"),
//     // //     None => println!("10"),
//     // // }
//     // //
//     // //
//     // //
//     // // let mut x: Option<i32> = None;
//     // //
//     // // x = Some(32);
//     // //
//     // // println!("{:?}", x);
//     // //
//     // // // assert_eq!(x.is_some(), false);
//     // //
//     // // let answer= practice_option(32);
//     // //
//     // // println!("{:?}", answer);
//     // //
//     // // let v = vec![1,2,3,4,5];
//     // //
//     // // // let does_not_exist: &i32 = &v[100];
//     // // let does_exist: Option<&i32> = v.get(100);
//     //
//     // //println!("{}", does_not_exist);
//     // // println!("{:?}", None);
//     //
//     //
//     // // println!("{:p}", does_not_exist);
//     // // println!("{}", does_not_exist);
//     // // println!("{:p}", &v[1]);
//     // // println!("{}", &v[1]);
//     //
//     //
//     // // //let v: Vec<i32>  = Vec::new();
//     // //
//     // // fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
//     // //     let s1 = sum_vec(v1);
//     // //     let s2 = sum_vec(v2);
//     // //
//     // //     s1 + s2
//     // // }
//     // //
//     // // fn sum_vec(v: &Vec<i32>) -> i32 {
//     // //     v.iter().fold(0, |a, &b| a + b)
//     // // }
//     // //
//     // // let v1 = vec![1, 2, 3];
//     // // let v2 = vec![4, 5, 6];
//     // //
//     // // let v3 = sum_vec(&v1);
//     // //
//     // // let answer = foo(&v1, &v2);
//     // //
//     // // println!("{}", answer);
//     //
//     // // println!("{:?}, {:?}, {}", v3, v4, answer);
//     // //
//     // // println!("{:?}", v1);
//     //
//     // // let v = vec![1,2,3,4,5];
//     // //
//     // // let third = &v[2];
//     // // println!("{}", third);
//     // // println!("{:?}", v);
//     //
//     //
//     // //
//     // // let v = vec![1, 2, 3, 4, 5];
//     // // let a = &v[0];
//     // //
//     // // let third: Option<&i32> = v.get(2);
//     // //
//     // // println!("{}", a);
//     // // println!("{}", v[1]);
//     // //
//     // //
//     // //
//     // // let s = "sdfasd".to_string();
//     // //
//     // // let ss = &s;
//     // //
//     // // let s12 = &s[..];
//     // //
//     // //
//     // // let s:u8 = 1;
//     // //
//     // // let s = "강용";
//     // //
//     // // println!("{}", (&s));
//     //
//     //
//     // // let mut s1 = String::from("hellow");
//     // // let s2 = String::from("world");
//     // //
//     // // let a = s2.as_str();
//     // // let s3 = s1 + &s2;
//     // //
//     // // let s3 = s1 + a;
//     // //
//     // // println!("{s3}");
//     //
//     //
//     // // let mut s1 = String::from("foo");
//     // // let s2 = "bar";
//     // // s1.push_str(s2);
//     // // println!("s2 is {s2}");
//     // //
//     // // let data = "initial contents";
//     // // let s = data.to_string();
//     // //
//     // // let s = "initial contents".to_string();
//     // //
//     // // let s = String::from("initial contents");
//     // //
//     // //
//     // // // Recall that we talked about string liters being stored inside the binary. Now that we know about slices,
//     // // // we can properly understand string literals;
//     // //
//     // // let s = "Hello world";
//     // //
//     // // //the type of s here is &str: it's a slice pointing to that specific point of the binary.
//     // // // This is also why string literals are immutable; &str is an immutale reference.
//     // //
//     // //
//     // // let my_string = String::from("hello world");
//     // //
//     // // // first)wird works on slices of String s, whether partial of whole
//     // // let word = first_word(&my_string[..]);
//     // // let s: String = String::from("hello world");
//     // // first_word(&s);
//     // //
//     // //
//     // // println!("{:p}", &s);
//     // // // print!("{:?}", [0,7]);
//     // // let z = &s;
//     // //
//     // //
//     // // println!("{:p}", z);
//     // //
//     // // println!("{}", *z)
//     //
//     // // let mut s = String::from("hello world");
//     // //
//     // // let word = first_word(&s); // word will get the value 5
//     // // println!("{word}");
//     // // s.clear();
//     // //
//     // // println!("{word}");
//     // // println!("{reference}");
//     // //
//     // //
//     // // let mut s = String::from("hello");
//     // //
//     // // let r1 = &s;
//     // // let r2 = &s;
//     // //
//     // // println!("{}, {}", r1, r2);
//     // //
//     // // let r3 = &mut s; // no problem
//     // //
//     // // println!("{s} sdfsdfsd");
//     // //
//     // //
//     // // // println!("{}, {}", r1, r2);
//     // //
//     // // // runtime allocated variable
//     // // // let mut s1 = String::from("hello");
//     // // //
//     // // // let s2 = &mut s1;
//     // // // let s3 = &mut s1;
//     // // //
//     // // // println!("{}, {}", s2, s3);
//     // // //
//     // // // // We call the action of creating a reference borrowing.
//     // // // // let len = calculate_length(&s1);
//     // // //
//     // // // let s = change(&mut s1);
//     // // //
//     // // // println!("{s}");
//     // //
//     // //
//     // // // println!("The length of '{}' is {}.", len.0, len.1);
//     // // //
//     // // // println!("{s1}");
//     // //
//     // //
//     // // // shadowing은 type이 달라도 상관없다.
//     // // // 하지만 mut은 최초에 선언한 타입과 다른 타입의 값을 할당할 시 에러가 발생한다.
//     // // let spaces = "   ";
//     // // let spaces = spaces.len();
//     // //
//     // // println!("{spaces}");
//     // // let x = 5;
//     // // println!("The first of x is: {x}, {:p}", &x);
//     // //
//     // // //let x = x + 1;
//     // //
//     // // {
//     // //     let x = x * 2;
//     // //     println!("The value of x in the inner scope is: {x}, {:p}", &x);
//     // //
//     // // }
//     // //
//     // // println!("The value of x is: {x}, {:p}", &x);
//     // //
//     // // let x = 6;
//     // // println!("The value of x is: {x}, {:p}", &x);
//     // //
//     // // println!("____________________________\n
//     // // ______________________________");
//     // //
//     // //
//     // // // 1. const는 shadowing이 안된다.
//     // // // 2. const는 타입 명시를 반드시 해줘야 한다. -> 결국엔 상수는 타입 추론이 아니라 컴파일 시에 명시적으로 알 수 있어야 하기 때문인것 같다.
//     // // const b: isize = 300;
//     // //
//     // //
//     // //
//     // //
//     // // let s = "sadfsf";
//     // // println!("first s : {:p}, {s}", &s);
//     // //
//     // // let ss = String::from(s);
//     // //
//     // // println!("  ss : {:p}, {ss}", &ss);
//     // //
//     // // let s = "sadf";
//     // // println!("second s : {:p}, {s}", &s);
//     // //
//     // //
//     // // println!("{s}");
//     // // println!("new {ss}");
//     // //
//     // //
//     // // let a = [10, 20, 30, 40, 50];
//     // //
//     // // for number in (1..4).rev() {
//     // //     println!("{number}");
//     // //
//     // // }
//     // //
//     // //
//     // // // let mut count = 0;
//     // // //
//     // // // 'counting_up: loop
//     // // // {
//     // // //     println!("count = {count}");
//     // // //
//     // // //     let mut remaining = 10;
//     // // //
//     // // //     loop
//     // // //     {
//     // // //         println!("remaining = {remaining}");
//     // // //
//     // // //         if remaining == 9 {
//     // // //             break;
//     // // //         }
//     // // //         if count == 2 {
//     // // //             break 'counting_up;
//     // // //         }
//     // // //         remaining -= 1;
//     // // //
//     // // //     }
//     // // //     count += 1;
//     // // // }
//     // // //
//     // // // println!("end count {count}");
//     // //
//     // //
//     // //
//     // //
//     // // let mut counter = 0;
//     // //
//     // // let result_ = loop {
//     // //     counter += 1;
//     // //
//     // //     if counter == 10 {
//     // //         break counter * 30;
//     // //     }
//     // // };
//     // //
//     // // println!("{result_} is  ");
//     // //
//     // //
//     // // let condition = false;
//     // // let number = if condition { "5" } else { "sdfsdf" };
//     // //
//     // // println!("{number}");
//     // //
//     // // fn five(num: u32) -> u32 {
//     // //     num + 5
//     // // }
//     // //
//     // // let a = five(1);
//     // // println!("{a}");
//     // //
//     // //
//     // // // Because Rust is an expression-based language
//     // // let c = {
//     // //     let cc = 11;
//     // //     cc + 12
//     // // };
//     // //
//     // // println!("ccccccc     {c}");
//     // //
//     // // let y = {
//     // //     let x = 3;
//     // //     x + 1
//     // // };
//     // //
//     // // println!("the value is {y}");
//     // //
//     // // let mut guess = String::new();
//     // //
//     // // let a: [u32; 5] = [1, 2, 3, 4, 5];
//     // //
//     // // // array should access brackets
//     // // let a = [3; 5];
//     // //
//     // // println!("{}", a[1]);
//     // //
//     // // let mut index = String::new();
//     // // // io::stdin()
//     // // //     .read_line(&mut index)
//     // // //     .expect("Failed to read line");
//     // // //
//     // // // let index: i32 = index.trim().parse().expect("Index entered was not a number");
//     // // // println!("{}", index);
//     // //
//     // // // for i in a {
//     // // //     println!("{}", i);
//     // // // }
//     // //
//     // // // tuple should access dot .
//     // // let x: (i32, f64, u8) = (500, 6.4, 1);
//     // // println!("{}", x.1);
//     // // // io::stdin().read_line(&mut guess).expect("Failed to read line");
//     // //
//     // // println!("{}", &mut guess);
//     // // // Constants can be declared in any scope, including the global scope,
//     // // // which makes them useful for values that many parts of code need to know about.
//     // //
//     // // // const A: usize = 1;
//     // //
//     // // // shadowing
//     // // let x = 1;
//     // // // let x = x +1;
//     // // let x = x + 1;
//     // // // println!("{x}")
//     // //
//     // // // let spaces = "   ";
//     // // // let spaces = spaces.len();
//     // //
//     // // let z = "42".parse::<usize>().expect("Not a number!");
//     // // println!("{}", z)
}
