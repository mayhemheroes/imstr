use honggfuzz::fuzz;
use imstr::ImString;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(s) = std::str::from_utf8(data) {
                let mut imstring = ImString::from("placeholder");
                for (i, c) in s.char_indices() {
                    let index = i % (imstring.len() + 1);
                    imstring.insert(index, c);
                    imstring.insert_str(index, &c.to_string());
                }
            }
        });
    }
}