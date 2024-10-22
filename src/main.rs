use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100); // Tạo biến secret_number ngẫu nhiên bằng Rng trong phạm vi từ 1 - 100
    println!("The secret number is: {secret_number}"); //In ra màn hình secret_number
    
    loop {
        println!("Please input your guess");
        let mut guess = String::new(); //Tạo một biến mới - kiểu String có khả năng thay đổi giá trị
        io::stdin() //gọi phương thức io
            .read_line(&mut guess)//gọi phương thức này để lấy giá trị nhập từ bàn phím
            .expect("Failed to read line"); //nếu .read_line trả về result:ok thì .expect sẽ thực thi.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
    };
    
        println!("You guessed: {guess}");// in ra màn hình , với {} là một dạng giữ chỗ

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}