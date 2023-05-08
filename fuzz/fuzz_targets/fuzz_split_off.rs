use honggfuzz::fuzz;
use imstr::ImString;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(s) = std::str::from_utf8(data) {
                if s.len() > 2 {
                    let mut imstring = ImString::from(s);
                    let split_index = s.len() / 2;
                    let split_str = imstring.split_off(split_index);
                    assert_eq!(imstring.as_str(), &s[..split_index]);
                    assert_eq!(split_str.as_str(), &s[split_index..]);
                }
            }
        });
    }
}