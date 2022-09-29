use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the game");
    let secret_number = rand::thread_rng().gen_range(1..101);

    // infinity loop
    loop {
        println!("please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        println!("You guessed: {}", guess);

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        // trim -> elimina todos os espaços em branco do início e do final da string

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
        // match serve para lidar com o resultado dependendo do resultado que será retornado
        // dependendo do resultado, será chamdo alguma das funcões decladas
    }
}
