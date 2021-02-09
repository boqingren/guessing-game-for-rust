use std::io;
use rand::Rng;

fn main() {
    println!("=== 猜数字游戏 ===");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("秘密号码是: {}", secret_number);

    println!("请输入你猜的数字，以回车键结束：");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法从命令行获取有效的输入！");

    println!("你的猜测的数字: {}", guess);
}
