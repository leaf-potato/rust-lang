fn main() {
    // 1. if表达式
    //  - 作用: 根据条件执行不同的代码分支
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 2. else if处理多重条件
    let number = 11;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 3. let语句中使用if表达式
    //  - rust无三目运算符, 功能替代.
    //  - if...else表达式类型相同.
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("the value of num is: {}", num);
}
