#![allow(unused)]

#[derive(Debug)]
enum ShirtColor {
    Red,
    Blue,
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct UUser {
    active: bool,
    username: String,
    email: &'static str,
    sign_in_count: u64,
}



struct Color(i32, i32, i64);

#[derive(Debug)]
struct Point { x: i32, y: i32}

struct AlwaysEqual;

use std::env::args;

fn print_point(p: &Point){
    println!("{} {}", p.x, p.y);
}

struct Rectangle { width: u32, height: u32}

fn main() {

    let width1 = 30;
    let height1 = 50;

    let rect1 = (30, 50);

    let rect2 = Rectangle {width: 30, height: 560};

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect2)
    );

    fn area(width: u32, height: u32) -> u32{
        width * height
    }

    fn area1(rect: (u32, u32)) -> u32{
        rect.0 * rect.1
    }

    fn area2(rect_struct: Rectangle) -> u32{
        rect_struct.width * rect_struct.height
    }
    // Similar to our discussion in "Different Tuple Fields",
    // Rust's borrow checker will track ownership permissions at both the struct-level and field-level.
    // For example, if we borrow a field x of a Point structure, then both p and p.x temporarily lose their permissions (but not p.y):

    return;
    // x: ro / *x: rw / p.x: r
    // let x = &mut p.x;
    //
    // *x += 123;
    // //*x += 1;
    //
    // println!("{}", p.x);
    //
    // let u = UUser {
    //     active: true,
    //     username: String::from("강용수"),
    //     email: "sdfsdf",
    //     sign_in_count: 23,
    // };
    //
    // println!("{:?}", u);
    //
    // // heap memory는 deallocate 되기 전까지 프로그램 실행시간 동안 메모리가 해제되지 않는다.
    // // &str은? static lifetime preallocated read-only
    // let subject = AlwaysEqual;
    //
    //
    // let black = Color(0, 0, 0);
    //
    //
    // let mut user1 = User {
    //     active: true,
    //     username: String::from("강용수"),
    //     email: String::from("gys1120@naver.com"),
    //     sign_in_count: 23,
    // };
    //
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("gys1120@gmail.com"),
    //     sign_in_count: user1.sign_in_count + 1,
    // };
    //
    // let user3 = User {
    //     ..user2
    // };


    // note that the entire instance must be mutable; Rust doesn't allow us to mark only certain fields as mutable.
    // user1.username = String::from("11");

    // [GUIDELINE] 중요 struct update syntax를 사용하게 되면 ownership이 이동하게 된다.
    // println!("{}", user1.username);




    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
}

