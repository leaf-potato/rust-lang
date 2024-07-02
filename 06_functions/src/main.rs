fn main() {
    // 1. 定义函数
    //  - 在函数签名中, 必须包含每个参数的类型.
    //  - 函数和变量名使用snake case规范格式.
    //  - 不关心函数定义所在位置, 作用域中可见.
    //  - 函数返回值等同于函数体最后1个表达式值.
    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {value}{unit_label}");
    }
    print_labeled_measurement(4, 'a'); // 调用函数

    // 2. 语句和表达式
    //  - 语句: 执行一些操作, 但不返回值的指令.
    //  - 表达式: 计算并产生一个值, 可以是语句一部分.
    // 
    // 例子: 代码块作为表达式赋值给expr
    let expr = {
        let x = 3;
        x + 1
    };
    println!("the value of expr is: {}", expr);

    // 表达式作为函数的返回值
    fn plus_one(x: i32) -> i32 {
        x + 1
    }
    println!("the value of plus_one is: {}", plus_one(5));
}
