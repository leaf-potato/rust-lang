fn main() {
    // 1. 定义结构体并访问
    //  - 不同于元组, 结构体需要命名各数据.
    //  - 整个实例可变, 不支持部分字段可变.
    #[derive(Debug)]            // 实现debug trait
    #[allow(dead_code)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64
    }

    let username = String::from("someone");
    let mut user1 = User {
        active: false,
        username,               // 语法简写
        email: String::from("someone@gmail.com"),
        sign_in_count: 32
    };

    println!("user1 value is: {:?}", user1);
    user1.active = true;        // 修改字段
    println!("user1 value is: {:?}", user1);

    // 2. 更新语法
    //  - 作用: 使用1个实例初始化另一个实例, 只改变其中部分字段的值.
    //  - 结构中的String等类型, 在赋值后数据会被移动, 之后不能访问.
    let user2 = User {
        active: true,
        ..user1
    };
    println!("user2 value is: {:?}", user2);

    // println!("user1.username: {}", user1.username); // 不能访问

    // 3. 元组结构体
    //  - 没有具体的字段名称, 只有字段的类型.
    //  - 元组中字段类型相同, 也是不同的类型.
    //  - 字段访问和元组相同, 没有任何的区别.
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Color(i32, i32, u32);

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Point(i32, i32, i32);

    let color = Color(0, 2, 3);
    let point = Point(0, 1, 2);

    println!("color value is: {:?}", color);
    println!("point value is: {:?}", point);

    // 4. 类单元体结构
    //  - 概念: 没有任何字段的结构体.
    //  - 作用: 通常用于使用某种trait.
    #[derive(Debug)]
    struct AlwaysEqual;

    let subject = AlwaysEqual;
    println!("subject value is: {:?}", subject);
}
