use std::fs::File;
use std::io::{self, Read, BufRead};
use std::path::Path;
use std::env;




fn main() {

	//get console arguments
	let args: Vec<String> = env::args().collect();


	//this is thse setup you need to access a file
	let path = Path::new(&args[1]);
	let file = File::open(&path).unwrap();
	let reader = io::BufReader::new(file);


	let mut diff_sum : i32 = 0;
	
	let mut left_vec : Vec::<i32> = vec![];
	let mut right_vec : Vec::<i32> = vec![];


	//now lets read it line by line
	for line in reader.lines() {
	
		let line = String::from(line.unwrap()); //actually get the string value

		//println!("The line is:{}",line);

		//now we must separate into two string based on the space in the midde
		let numbers : Vec<&str> = line.split_whitespace().collect();


		let n1 : i32 = numbers[0].parse::<i32>().unwrap();
		let n2 : i32 = numbers[1].parse::<i32>().unwrap();

		left_vec.push(n1);
		right_vec.push(n2);

	}

		left_vec.sort();
		right_vec.sort();

		let length = left_vec.len();

		for i in 0..length{

			diff_sum += (left_vec[i] - right_vec[i]).abs();

		}


	println!("The final sum is:{}",diff_sum);


}
