use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess The Number!");

	let secret_number = rand::thread_rng().gen_range(1, 101);

//	println!("The secret number is: {}", secret_number);

	let mut num_guesses = 0;
	loop {
	    println!("Please input your guess (0-100):");

	    let mut guess = String::new();

	    io::stdin()
	    	.read_line(&mut guess)
	    	.expect("Failed to read line.");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
	    
//	    println!("You guessed: {}", guess);

	    match guess.cmp(&secret_number) {
	    	Ordering::Less => println!("Too small!"),
	    	Ordering::Greater => println!("Too big!"),
	    	Ordering::Equal => {
	    		println!("You win!");
	    		println!("It took you {} guesses to win!", num_guesses);
				break;
	    	}
	    }
	    num_guesses = num_guesses + 1;
	}
    println!("Press enter to exit.");
    let mut voids = String::new();
	io::stdin()
		    	.read_line(&mut voids)
		    	.expect("Failed to read line.");
}
