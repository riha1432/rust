use std::io;

fn main() {
    println!("Guess the number!");

    println!("Plesas input your guess.");

    let mut guess = String::new(); // 가변
    // 문자열 이라는 변수 객체 생성
    // 가변으로 사용시 mut 작성
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);
}
