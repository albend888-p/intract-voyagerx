use std::io;
use random_word::Lang;

fn main(){
    println!("Guess the Word!");
	
	let mut result = Vec::new();
	let mut not_contains = Vec::new();
	let secret_word = random_word::gen(Lang::En);
	let count = secret_word.chars().count();		

	println!("Secret word is: {} ({} lenght / lang EN)", "*".repeat(count), count);		
	println!("Write your variant:");		
	
	loop {         
		let mut guess = String::new();
		io::stdin().read_line(&mut guess)
		.expect("Failed to read line"); 

		if guess.trim() == secret_word {
			println!("Congratulations, it's incredibly difficult to solve a word in such a boring game!");			
		}
		else if guess.trim().chars().count() ==  count {
			result.clear();
			println!("your variant is {}", guess);	
			for (i, ch) in guess.trim().to_lowercase().char_indices() {	
				if secret_word.contains(ch) == false {
					not_contains.push(ch.to_string());
				}
				let input = ch;
				let secret = secret_word.chars().nth(i).unwrap();
				if input == secret {
					result.push(input.to_string());			
				}
				else {
					result.push('*'.to_string());	
				}
			}
			println!("Secret word is: {} ({} lenght / lang EN / not containts letters: {})", result.clone().into_iter().collect::<String>(), count, not_contains.clone().into_iter().collect::<String>());			
		}
		else {
			println!("You wrote wrong letters count word!");	
		}				
	}	 
}
