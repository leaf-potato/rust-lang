fn main() {
    // 1. 声明&定义可变变量
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6; // 重新赋值
    println!("The value of x is: {}", x);

    // 2. 声明&定义常量
    //  - 单词之间使用全大写加下划线.
    //  - 常量默认不可变&总是不可变.
    //  - 必须显示注明值的类型, u32等.
    //  - 常量只能被设置为常量表达式.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("seconds: {}s", THREE_HOURS_IN_SECONDS);

    // 3. 变量隐藏
    // shadow与mut的区别:
    //  - shadow新建的变量默认也是不可变的.
    //  - shadow实际创建新变量, 支持改变值类型.
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}
