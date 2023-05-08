use honggfuzz::fuzz;
use imstr::ImString;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(s) = std::str::from_utf8(data) {
                let imstring = ImString::from(s);
                let imstring_lines: Vec<String> = imstring.lines().map(|line| line.to_string()).collect();
                let str_lines: Vec<&str> = s.lines().collect();
                assert_eq!(imstring_lines, str_lines);
            }
        });
    }
}