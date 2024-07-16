use borrow_me_the_reference::{delete_and_backspace, do_operations};

fn main() {
	let mut a = String::from("bpp--o+er+++sskroi-++lcw");
	let mut b: Vec<String> = vec!["2+2".to_string(), "3+2".to_string(), "10-3".to_string(), "5+5".to_string()];
	// let mut b: Vec<String> = vec!["2+2", "3+2", "10-3", "5+5"];

	delete_and_backspace(&mut a);
	do_operations(&mut b);

	println!("{:?}", (a, b));
}