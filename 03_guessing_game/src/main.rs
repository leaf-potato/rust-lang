// 将io库引入当前作用域
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // 使用println!宏来输出
    println!("Guess the number!");

    // start..=end范围表达式作为参数
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("secret_number:{}", secret_number);

    loop {
        // String::new: 声明&定义可变的字符串
        println!("Please input your guess.");
        let mut guess = String::new();

        // read_line返回io::Result类型(枚举)
        // read_line返回io::Result类型(枚举)
        // Result::Ok: 操作成功 & 对应的结果值.
        // Result::Err: 操作失败 & 失败的原因.
        // expect():
        //     1. Err: 中断当前的程序, 并将传入的字符串信息显示
        //     2. Ok: 提取Ok中附加的值, 并将值作为结果返回给用户
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // 遇到Err直接panic

        // 类型转换: string => u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed:{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // 终止循环
            }
        }
    }

}
