



pub struct Block {
	index: u64,
	timestamp: u64,
	hash: Vec<u8>,
	previous_hash: Vec<u8>,
	payload: Vec<u8>,
}
