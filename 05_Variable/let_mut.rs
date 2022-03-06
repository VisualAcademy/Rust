fn main() {
    let num1 = 10; // 변수(상수, immutable)
    println!("num1 = {}", num1);
    // num1 = 20; // 컴파일 에러 
    
    let mut num2 = 1234; // 변수(mutable)
    println!("num2 = {}", num2);
    num2 = 3456;
    println!("num2 = {}", num2);
}

// num1 = 10
// num2 = 1234
// num2 = 3456
