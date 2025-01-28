use itertools::{ Itertools as _ };
use rand::{ Rng as _ };

use pyo3::prelude::*;

const BASE64_CHARS: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

#[pyfunction]
fn generate(amount: usize) -> PyResult<String> {
    let mut rng = rand::rng();

    Ok(std::iter::repeat(())
        .take(amount)
        .map(|_| BASE64_CHARS[rng.random_range(0..=63)])
        .join(""))
}

#[pyfunction]
fn generate_urlsafe(amount: usize) -> PyResult<String> {
    let mut rng = rand::rng();

    Ok(std::iter::repeat(())
        .take(amount)
        .map(|_| match rng.random_range(0..=63) {
            62 => '-',
            63 => '_',
            idx => BASE64_CHARS[idx],
        })
        .join(""))
}

#[pymodule]
fn rand_base64(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate, m)?)?;
    m.add_function(wrap_pyfunction!(generate_urlsafe, m)?)?;
    Ok(())
}
