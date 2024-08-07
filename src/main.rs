#![allow(unused)]
#![allow(dead_code)]

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let _y = 10;

    // 字符串类型
    let spaces = "   ";
    // usize数值类型
    let spaces = spaces.len();

    let guess: i32 = "42".parse().expect("Not a number!");

}
