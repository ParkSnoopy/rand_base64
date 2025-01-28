use generate_base64::generate_base64_urlsafe;

fn main() {
	let string = generate_base64_urlsafe(120);
	println!("[{}] {:?}", string.len(), string);
}
