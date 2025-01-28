use rand::{ Rng as _ };
use itertools::{ Itertools as _ };

//const BASE64_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const BASE64_CHARS: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

pub fn generate_base64(amount: usize) -> String {
    let mut rng = rand::rng();

    std::iter::repeat(())
        .take(amount)
        .map(|_| BASE64_CHARS[ rng.random_range(0..=63) ])
        .join("")
}

pub fn generate_base64_urlsafe(amount: usize) -> String {
    let mut rng = rand::rng();

    std::iter::repeat(())
        .take(amount)
        .map(|_| match rng.random_range(0..=63) {
            62  => '-',
            63  => '_',
            idx => BASE64_CHARS[ idx ],
        })
        .join("")
}
