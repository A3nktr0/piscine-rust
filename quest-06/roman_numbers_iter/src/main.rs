use roman_numbers_iter::RomanNumber;

fn main() {
	let mut number = RomanNumber::from(2149);

	println!("{:?}", number);
	println!("{:?}", number.next());
}