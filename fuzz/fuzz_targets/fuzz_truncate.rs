use honggfuzz::fuzz;
use imstr::ImString;
use std::str::FromStr;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(s) = std::str::from_utf8(data) {
                let mut imstring = ImString::from_str(s).unwrap();
                for i in 0..=s.len() {
                    imstring.truncate(i);
                    assert_eq!(imstring.as_str(), &s[..i]);
                }
            }
        });
    }
}