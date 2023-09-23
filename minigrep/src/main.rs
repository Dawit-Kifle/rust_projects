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

use std::any::Any;
use std::env;

#[derive(Debug)]
struct User {
    name: String,
    age: i32,

}
#[derive(Debug)]
struct Point { x: i32, y: String }

#[derive(Debug)]
struct Color (i32, i32, i32);

struct AlwaysEqual;

#[derive(Debug)]
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


}

fn main() {

    // Unlike functions, methods are defined within the context of a struct
    // and their first parameter is always self, which represents the instace of the
    // struct the method is being called on.
    enum Message {

    }

    let mut x = &Box::new(1);

    println!("{:p}", x);
    println!("{:p}", &x);
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

