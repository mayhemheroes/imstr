use honggfuzz::fuzz;
use imstr::ImString;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            match ImString::from_utf8(data.to_vec()) {
                Ok(imstring) => {
                    let utf8_str = std::str::from_utf8(data).unwrap();
                    assert_eq!(imstring.as_str(), utf8_str);
                }
                Err(_err) => {
                    // Panic if valid UTF-8
                    let _ = std::str::from_utf8(data).unwrap();
                }
            }
        });
    }
}