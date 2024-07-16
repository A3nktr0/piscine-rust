use std::io;

fn main() {
    let mut n: i32 = 0;
    loop {
        n += 1;
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        if guess.trim() == "The letter e" {
            println!("Number of trials: {}", n);
            break;
        }
    }
}
