fn main() {
use std::io;
	
	let mut numofdiv: i32 = 0;
	let number: i32;
	
	println!("Enter a number");
	let mut input = String::new();
	io::stdin()
	 .read_line(&mut input)
	 .expect("Cannot read");

	number = input.trim().parse().expect("Not a number");

	for i in 1..=number {
		if number % i == 0 {
			numofdiv = numofdiv + 1
		};
	};
	println!("{} has {} divisors", number, numofdiv);

	if numofdiv == 2 {
		println!("{} is prime", number)
	} else {
		println!("{} is not prime", number)
	}
}
