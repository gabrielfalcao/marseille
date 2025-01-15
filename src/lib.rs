use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn alphabet2morse(data: String, sep: &str) -> String {
    let mut r = Vec::<&str>::new();
    for o in data.bytes() {
        r.push(match o {
            b'a' | b'A' => ".-",
            b'b' | b'B' => "-...",
            b'c' | b'C' => "-.-.",
            b'd' | b'D' => "-..",
            b'e' | b'E' => ".",
            b'f' | b'F' => "..-.",
            b'g' | b'G' => "--.",
            b'h' | b'H' => "....",
            b'i' | b'I' => "..",
            b'j' | b'J' => ".---",
            b'k' | b'K' => "-.-",
            b'l' | b'L' => ".-..",
            b'm' | b'M' => "--",
            b'n' | b'N' => "-.",
            b'o' | b'O' => "---",
            b'p' | b'P' => ".--.",
            b'q' | b'Q' => "--.-",
            b'r' | b'R' => ".-.",
            b's' | b'S' => "...",
            b't' | b'T' => "-",
            b'u' | b'U' => "..-",
            b'v' | b'V' => "...-",
            b'w' | b'W' => ".--",
            b'x' | b'X' => "-..-",
            b'y' | b'Y' => "-.--",
            b'z' | b'Z' => "--..",
            0_u8..=96_u8 | 98_u8..=u8::MAX => sep,
        });
    }
    let m = r.join(sep);
    String::from(m)
}
