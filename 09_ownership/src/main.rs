fn main() {
    // 1. 所有权概念
    // 所有程序都要管理运行时计算机内存
    //  - 垃圾回收机制自动释放内存.
    //  - 程序员必须亲自分配和释放内存.
    //  - rust通过所有权系统管理内存.

    // 2. 所有权规则
    //  - rust中每个值都有1个所有者.
    //  - 值任何时刻有且只有1个所有者.
    //  - 当所有者离开作用域, 值将被丢弃.

    // 3. String类型
    //  - 必须在运行时向内存分配器(memory allocator)请求内存.
    //  - 需要一个当我们处理完String时将内存返回给分配器的方法.
    {
        let mut s = String::from("hello");      // s是有效的
        s.push_str(", world!");

        println!("the string value is: {}", s);
    }                                                   // 作用域结束
                                                        // s不再有效
    // {
    //     let s1 = String::from("hello");
    //     let s2 = s1;
    //     println!("the s1 string value is: {}", s1);  // s1被移动了
    // }

    {
        let s1 = String::from("hello");
        let _s2 = s1.clone();
        println!("the s1 string value is: {}", s1);     // s1被拷贝了
    }

    // 5. 函数所有权
    // 参数 & 返回值: 非引用会转移所有权.
    let str = String::from("hello world!");
    fn takes_ownership(str: String) -> String {         // 返回参数所有权
        return str;
    }
    let str_back = takes_ownership(str);        // 拿到str所有权
    println!("the str_back value is: {}", str_back);

    // 5. 引用和借用
    //  - 作用: 在参数传递过程中不转移所有权.
    //  - 可变引用: 在函数内可修改引用的参数.
    //  - 限制:
    //      - 变量已有1个可变引用, 不能再创建可变引用.
    //      - 不能在拥有不可变引用的同时拥有可变引用.
    //      - 引用必须总是有效的, 不能出现悬垂引用.
    let str = String::from("hello");
    fn calculate_len(str: &String) -> usize {           // 引用返回字符串长度
        return str.len();
    }
    println!("the str len is: {}", calculate_len(&str));

    let mut str = String::from("hello");
    fn append_str(str: &mut String)  {                  // 可变引用修改字符串
        str.push_str(", world!");
    }
    append_str(&mut str);
    println!("the str after modify is: {}", str);

    // 6. Slice
    //  - 引用集合中一段连续的元素序列.
    //  - 本质上是引用, 没有所有权问题.
    fn first_world(str: &str) -> &str { // 函数返回切片
        let bytes = str.as_bytes();
        for (idx, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &str[..idx];
            }
        }
        return &str[..];
    }
    let word = first_world("hello world");
    println!("the first_world value is: {}", word);
}