extern crate rand;

// 导入标准库
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // 带有感叹号的调用是宏
    println!("Guess the number!");

    // 获取1~100之间的随机值
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // 来个无限循环
    loop {
        println!("Please input your guess.");

        // let创建变量绑定，默认绑定是不可变的
        // mut让一个绑定可变
        // ::是调用静态方法（与String关联的函数）
        let mut guess = String::new();

        // read_line应该是个实例方法
        // &mut 是可变引用的写法
        // expect是泛型Result的方法
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // 将字符串转化为数字
        // 这里的guess会覆盖上面的guess
        // 添加match，防止程序退出
        let guess: u32 =  match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}", guess);

        // 使用match语句来匹配cmp的返回值
        // 为每个可能的值创建一个分支，Ordering是个枚举
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                // 获取胜利后，跳出循环
                break;
            }
        }
    }
}
