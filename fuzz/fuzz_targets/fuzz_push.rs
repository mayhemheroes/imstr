use honggfuzz::fuzz;
use imstr::ImString;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(s) = std::str::from_utf8(data) {
                let mut imstring = ImString::new();
                for c in s.chars() {
                    imstring.push(c);
                    imstring.push_str(&c.to_string());
                }
                assert_eq!(imstring.as_str(), s.repeat(2));
            }
        });
    }
}