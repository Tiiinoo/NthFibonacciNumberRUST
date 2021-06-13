use std::io;

fn fibonacci(n: u32) -> u128 {
		match n {
				0 => 1,
				1 => 1,
				_ => fibonacci(n - 1) + fibonacci(n - 2),
		}
}

fn main() {
		println!("You want a fibo?");
		println!("When you want to quit write stop");

		loop {
			println!("Write the number you want to fibonnaize");

			let mut n = String::new();

			io::stdin()
						.read_line(&mut n)
						.expect("Failed to read the line");

			if n.trim() == "stop" {
					break;
			}

			let n: u32 = match n.trim().parse() {
							Ok(num) => num,
							Err(_) => continue,
					};
			
			println!("The corresponding number in the fibo sequence is {}", fibonacci(n));
			
		}
}
