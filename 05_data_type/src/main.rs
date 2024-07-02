fn main() {
    // 1. 整型溢出
    //  - debug会溢出会panic
    //  - release对最大值取模
    let mut num = 255u8;
    num = num + 1; // 导致溢出
    println!("the value of num is: {}", num);

    // 2. 浮点类型
    //  - 默认是f64类型.
    //  - IEEE-754标准.
    let f64 = 64.0;
    let f32: f32 = 32.0;
    println!("the value of f64 is: {}", f64);
    println!("the value of f32 is: {}", f32);

    // 3. 字符类型
    // Unicode -> 4个字节
    let ch = '赵';
    println!("char type size:{}", ch.len_utf8());

    // 4. 布尔类型
    let flag = true;
    println!("the value of flag is: {}", flag);

    // 5. 元组类型
    //  - 将多个类型值组合成1个复合类型.
    //  - 元组在声明后长度固定, 不可变.
    //  - 每个位置有个类型, 类型可以不同.
    // 
    // 访问元素:
    // - 模式匹配解构获取元素值.
    // - 使用.后跟索引位来访问.
    let tup = (32, 2.0, 'A');
    println!("the value of tuple is ({}, {}, {})",
            tup.0, tup.1, tup.2); 

    // 模式匹配来解构获取元组中的值
    let (x, y, z) = tup;
    println!("the value of tuple is ({}, {}, {})",
            x, y, z); 

    // 6. 数组类型
    // - 数组长度固定, 且元素类型必须相同.
    // - 数组元素都是存储在栈上, 不会涉及堆.
    let arr:[u32; 5] = [1, 2, 3, 4, 5]; // 指定类型
    println!("the value of arr is: {:?}", arr);

    let arr = [3; 4];         // 创建相同值的数组
    println!("the value of arr is: {:?}", arr);
    println!("the first value of arr is: {}", arr[0]);  // 访问数组元素
}
