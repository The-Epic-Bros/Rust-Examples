fn is_prime(number: i64) -> bool {
	let upper_limit = (number as f64).sqrt() as i64 + 1;

	for i in 2..upper_limit + 1 {

		if number % i == 0 {
			return false;
		}
	}

	return true;
}

fn main() {
	// Tests
	// println!("{}", is_prime(100014437)); -> true
	// println!("{}", is_prime(100000000000000)); -> false
}
