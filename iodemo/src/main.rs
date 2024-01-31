use std::io;
use rand::Rng;
use std::cmp::Ordering;

// use std::io;
fn main() {
    println!("猜数游戏");
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("secret = {}", secret);
    loop {
        println!("guess number");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("read error");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("you guess number : {}", guess);
        match guess.cmp(&secret) {
            Ordering::Less => {
                println!("less secret")
            }
            Ordering::Equal => {
                println!("equal secret");
                break;
            }
            Ordering::Greater => {
                println!("Greater secret")
            }
        }
    }
}
