use rand::Rng;
use std::io;

fn main() {
    println!("Guess the name");

    let names: Vec<&str> = vec!["Erick", "Sabes", "Holland", "Falcon"];
    let name_value: &str = names[(rand::thread_rng().gen_range(0..names.len()))];

    loop {
        let mut guess: String = String::new();

        println!("Secret name is: {}", name_value);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed");

        if check_guess_not_string(&mut guess) {
            if guess.trim() == name_value {
                println!("You win");
                break;
            } else {
                println!("Try again");
                println!("Input got: {}", guess);
            }
        } else {
            println!("Please type a string");
        }
    }
}

fn check_guess_not_string(x: &String) -> bool{
    return match x.trim().parse::<f64>() {
        Ok(_) => {
            false
        }
        Err(_) => {
            true
        }
    }
}