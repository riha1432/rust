fn main() {
    let mut text = 5; // 가변 변수
    
    const MAX_POINTS: u32 = 100000; // uint32_t 상수

    println!("{}",MAX_POINTS);
    println!("The value of x is: {}", text);
    text= 6;
    println!("The value of x is: {}", text);

    let x = 5; // 불변 
    let x = x + 1; // 불변에 같은 이름 사용하여 shadows 가능
    let x = x * 2; // 
    println!("the value of x is : {}", x);

    let spaces = "   "; // 불변 
    let spaces = spaces.len(); // 불변에 동일한 이름을 ㄱ진 새롭게 정의 돈 숫자 가능
    println!("the value of x is : {}", spaces);

    // let mut spaces = "   "; // 가변
    // let spaces = spaces.len(); // 가변 변수에 동일한 이름을 가진 불변 변수 정의시 컴파일 애러
    // println!("the value of x is : {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // arch	isize	usize
    println!("{}", guess);
}
