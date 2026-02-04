

use rand::Rng;

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess < secret {
        -1
    }
    else if guess > secret {
        1
    }
    else {
        0
    }
}


fn main() {
    
    let secret : i32 = rand::thread_rng().gen_range(1..=100);
    let mut guess: i32 = rand::thread_rng().gen_range(1..=100);
    let mut v2 : i32 = 100;
    let mut v1 : i32 = 0;
    let mut rev : i32 = 0;
    let mut i = true;
    let mut counter = 0;

    while i {
        if check_guess(guess, secret) == -1 {
            println!("Guess ({}) was too low. ", guess);
            v1 = guess;
            rev = guess;
            guess = guess + ((v2-v1)/2);
            counter += 1;
        }
        
        else if check_guess(guess, secret) == 1 {
            println!("Guess ({}) was too high. ", guess);
            if v2 < guess {
                guess = rev ;
                v1 = guess;
                rev = guess;
                guess = guess + ((v2-v1)/2);
                counter += 1;
            }
            else if v2 > guess{
                v2 = guess;
                guess = rev;
                v1 = guess;
                rev = guess;
                guess = guess + ((v2-v1)/2);
                counter += 1;
            }
            }
        
        else if check_guess(guess, secret) == 0 {
            counter += 1;
            println!("Guess ({}) was correct! Secret number was also {}", guess, secret);
            i = false;
        }
    }

    println!("The total amount of guesses was: {}." , counter);
}

