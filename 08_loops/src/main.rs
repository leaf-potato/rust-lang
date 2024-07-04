fn main() {
    // 1. loop循环
    //  - 循环返回值
    //      - 使用场景: 重试可能失败的操作.
    //      - 将返回值用于循环的break表达式.
    //  - 循环标签
    //      - break和continue默认作用域最内层循环.
    //      - break和continue可作用于指定循环标签.
    let mut count = 0;
    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2; // 循环返回值
        }
    };
    println!("the value of count is: {}", result);

    let mut count = 0; 
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);         
            if remaining == 9 {
                break; 
            }
            if count == 2 { 
                break 'counting_up; // 指定循环标签
            }
            remaining -= 1;
        }
        count += 1; 
    }
    println!("end count = {}", count);

    // 2. while循环
    //  - 作用: 条件不为true中终止循环.
    //  - 类似loop + if, 结构更加简单.
    let mut number = 3;
    while number != 0 {
        println!("number = {}", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // 3. for循环
    //  - 作用: 对数组元素的遍历, 防止越界.
    //  - 可以使用range类型指定范围来遍历.
    let elems = [10, 20, 30, 40, 50];
    for elem in elems {
        println!("the value is: {}", elem);
    }

    for (idx, elem) in elems.iter().enumerate() {
        println!("the idx: {}'s value is: {}", idx, elem);
    }

    for idx in (0..4).rev() {
        println!("the for number: {}", idx);
    }
}
