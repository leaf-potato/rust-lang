// 定义枚举类型
#[derive(Debug)]
enum IpAddrKind {
    V4, // ipv4
    V6, // ipv6
}

#[derive(Debug)]
#[allow(dead_code)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 枚举类型绑定值
#[derive(Debug)]
#[allow(dead_code)]
enum IpAddr2 {
    V4(String),
    V6(String), 
}

// 枚举绑定不同类型值
#[derive(Debug)]
#[allow(dead_code)]
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String), 
}

fn main() {
    // 1. 枚举类型
    //  - 声明某个值是一个集合中的一员.
    //  - 枚举成员位于标识符命名空间中.
    //  - 枚举成员名字 -> 构建枚举实例的函数.
    //  - 与结构体相同, 可以在impl中定义方法.
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four value is: {:?}", four);
    println!("six value is: {:?}", six);

    let home = IpAddr { // 枚举类型和值绑定
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("home address value is: {:?}", home);
    println!("loopback address value is: {:?}", loopback);

    // 将数据附加到枚举的每个成员上
    let home = IpAddr2::V4("127.0.0.1".to_string()); 
    let loopback = IpAddr2::V6("::1".to_string());

    println!("home address value is: {:?}", home);
    println!("loopback address value is: {:?}", loopback);

    // 枚举成员的类型值不同
    let home = IpAddr3::V4(127, 0, 0, 0); 
    let loopback = IpAddr3::V6("::1".to_string());

    println!("home address value is: {:?}", home);
    println!("loopback address value is: {:?}", loopback);

    // 2. Option
    //  - Some: 存在某个值, 值类型为T
    //  - None: 不存在值, 变量指定类型.
    //  作用:
    //  - 捕获到假设值不为空, 实际为空的情况.
    //  - 增加代码的安全性, 确保总是访问有效值.
    let some_number = Some(5);
    let some_char = Some('e');
    let _absent_number: Option<i32> = None;     // 空值

    println!(
        "number:{}, char:{}",
        some_number.unwrap(), some_char.unwrap());

    // 3. match表达式
    //  - 将值与模式相比较, 根据相匹配的模式执行代码.
    //  - 具备穷尽性, 编译器要求必须处理所有可能情况.
    #[allow(dead_code)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter, 
    }

    fn value_in_cents(coin: &Coin) -> u8 {
        match coin { // match表达式并返回值
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,  
        }
    }
    println!("coin in cents is: {}", value_in_cents(&Coin::Quarter));

    #[derive(Debug)]
    #[allow(dead_code)]
    enum UsState { // 定义州名称
        Alabama,
        Alaska,
        // --snip--
    }

    #[allow(dead_code)]
    enum Coin2 {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState), 
    }

    fn value_in_cents2(coin: &Coin2) -> u8 {
        match coin {
            Coin2::Penny => 1,
            Coin2::Nickel => 5,
            Coin2::Dime => 10,
            Coin2::Quarter(state) => { // 绑定值模式
                println!("State quarter from {state:?}!");
                25
            }
        }
    }
    println!("coin2 in cents is: {}", 
            value_in_cents2(&Coin2::Quarter(UsState::Alabama)));

    // 4. if let
    //  - 作用: match需要处理所有分支, if let处理一种情况.
    //  - 依赖特定环境, 增加简洁度和失去穷尽性检查的权衡取舍.
    let config_max = Some(3u8);
    match config_max { // 只处理Some的情况
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => () // 满足match穷尽性
    };

    if let Some(max) = config_max { // match的语法糖
        println!("The maximum is configured to be {max}");
    }
}