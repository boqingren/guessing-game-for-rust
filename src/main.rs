use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("=== 猜数字游戏 ===");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("秘密号码是: {}", secret_number);

    loop {
        println!("请输入你答案，以回车键结束：");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法从命令行获取有效的输入！");

        let guess: u32 = guess.trim().parse().expect("请输入合法的数字！");

        println!("你输入的数字是: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("结果：太小了！"),
            Ordering::Greater => println!("结果：太大了"),
            Ordering::Equal => println!("结果：恭喜你猜对了，正确的答案是：{}", secret_number)
        }

        println!("====================================");
    }
}
