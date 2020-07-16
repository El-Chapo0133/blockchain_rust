use std::sync::mpsc::channel;

mod data {
	pub const HASH_LENGTH: i32 = 256;
	pub const NUMBER_THREADS: i32 = 2;
}

pub struct HashBlock {

}

struct DataThread {
	index: i32,
	data: Vec<u8>,
}

impl HashBlock {
	fn generate(input: Vec<u8>) -> Vec<u8> {
		return Vec::new();
	}
}

fn extend(input: Vec<u8>) -> Vec<u8> {
	let mut to_return: Vec<u8> = Vec::new();
	loop {
		push_all(&mut to_return, &input);
		if to_return.len() as i32 >= data::HASH_LENGTH {
			break;
		}
	}
	return to_return[..(data::HASH_LENGTH - 1) as usize].to_vec();
}

fn push_all(data: &mut Vec<u8>, input: &Vec<u8>) {
	for cell in input {
		data.push(*cell);
	}
}

fn calc(input: Vec<u8>) -> Vec<u8> {
	let to_return: Vec<u8> = Vec::new();
	let (sender, receiver) = channel();
	for _ in 0..data::NUMBER_THREADS {
		std::thread::spawn(move || {
			handle_thread(sender.clone());
		});
	}
	
}

fn handle_thread() {

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn assert_push_all() {
		let mut data: Vec<u8> = [6,2,8].to_vec();
		let input: Vec<u8> = [2,6,1,4,7,2,1].to_vec();

		push_all(&mut data, &input);
		assert_eq!(data, [6,2,8,2,6,1,4,7,2,1].to_vec());
	}
	#[test]
	fn assert_extend() {
		let input_1: Vec<u8> = b"test".to_vec();
		let expected_1: Vec<u8> = b"testtesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttes".to_vec();
		let input_2: Vec<u8> = b"Code me daddyy !!".to_vec();
		let expected_2: Vec<u8> = b"Code me daddyy !!Code me daddyy !!Code me daddyy !!Code me daddyy !!Code me daddyy !!Code me daddyy !!Code me daddyy !!Code me daddyy !!Code me daddyy !!Code me daddyy !!Code me daddyy !!Code me daddyy !!Code me daddyy !!Code me daddyy !!Code me daddyy !!".to_vec();

		assert_eq!(extend(input_1), expected_1);
	}
}
