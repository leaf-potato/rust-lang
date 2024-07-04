// 定义结构体
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 实现方法(关联函数)
impl Rectangle {
    // &self等同于self: &Self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 判断是否容纳另一个长方形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 静态方法: 构造正方形
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 构造1个正方形
    let rect4 = Rectangle::square(3);
    println!(
        "The area of the square is {} square pixels.",
        rect4.area());
}