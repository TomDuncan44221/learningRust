use std::io;


fn main() 
{
    println!("Guess the number, go on! Guess!");

    println!("Gimme a number, c'mon!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line. Sad soup.");

    println!("You wot? You guessed {} Alright then.", guess);
}
