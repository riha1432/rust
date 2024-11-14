use std::io;

fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    // println!("{x}"); 불가능

    // let (x, y, z) = tup;
    // println!("the value of y is:{y}");

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    // 투플 접근 _.0, _.1 _._ 튜플 배열 접근

    println!("the value of five_hundred is:{five_hundred}");
    println!("the value of six_point_four is:{six_point_four}");
    println!("the value of one is:{one}");

    // 배열
    let a = [1,2,3,4,5];
    let months = ["january", "February", "March", "April", "May", "June", "July", "August", "Septemaaaber", "October", "November", "December"];
    let a: [i32; 5] = [1,2,3,4,5]; // int32타입을 배열크기 5
    let a = [3; 5]; // 3으로 채워진 배열크기 5
    // 배열 접근
    let a = [1,2,3,4,5];
    let first = a[0];
    let second = a[1];

    println!("Please endter an array index.");

    let  mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
