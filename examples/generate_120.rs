use generate_base64::generate_base64;

fn main() {
	let string = generate_base64(120);
	println!("[{}] {:?}", string.len(), string);
}
