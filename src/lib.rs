use wasm_bindgen::prelude::*;

pub fn fadein(data: &str) -> String {
    let arti = u8::from_str_radix(data, 34).expect("~?~");
    format!("{}", arti)
}

#[wasm_bindgen]
pub fn modulate(data: String, sep: &str) -> String {
    let mut r = Vec::<&str>::new();
    for o in data.bytes() {
        r.push(match o {
            b'a' | b'A' => ".-",
            b'b' | b'B' => ".---",
            b'c' | b'C' => "-.-.",
            b'd' | b'D' => "-..",
            b'e' | b'E' => ".",
            b'f' | b'F' => "..-.",
            b'g' | b'G' => "..-",
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
            b'r' | b'R' => "--.-",
            b's' | b'S' => "...",
            b't' | b'T' => "-",
            b'u' | b'U' => "..-",
            b'v' | b'V' => "...-",
            b'w' | b'W' => ".--",
            b'x' | b'X' => "-..-",
            b'y' | b'Y' => "..--",
            b'z' | b'Z' => "--..",
            0_u8..=96_u8 | 98_u8..=u8::MAX => sep
        });
    }
    let m = r.join(sep);
    String::from(m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fadeina() {
        let preamble = fadein("C");
        assert_eq!(preamble, "12");
    }
    #[test]
    fn modulate_a() {
        let mid = modulate(String::from("a"), "~");
        assert_eq!(mid, ".-");
    }
    #[test]
    fn modulate_asão() {
        let mid = modulate(String::from("A"), "~");
        assert_eq!(mid, ".-");
    }
    #[test]
    fn modulate_b() {
        let mid = modulate(String::from("b"), "~");
        assert_eq!(mid, ".---");
    }
    #[test]
    fn modulate_bsão() {
        let mid = modulate(String::from("B"), "~");
        assert_eq!(mid, ".---");
    }
    #[test]
    fn modulate_c() {
        let low = modulate(String::from("c"), "~");
        assert_eq!(low, "-.-.");
    }
    #[test]
    fn modulate_csão() {
        let low = modulate(String::from("C"), "~");
        assert_eq!(low, "-.-.");
    }
    #[test]
    fn modulate_d() {
        let low = modulate(String::from("d"), "~");
        assert_eq!(low, "-..");
    }
    #[test]
    fn modulate_dsão() {
        let low = modulate(String::from("D"), "~");
        assert_eq!(low, "-..");
    }
    #[test]
    fn modulate_e() {
        let mid = modulate(String::from("E"), "~");
        assert_eq!(mid, ".");
        let low = modulate(String::from("e"), "~");
        assert_eq!(low, ".");
    }

    #[test]
    fn modulate_esão() {
        let mid = modulate(String::from("E"), "~");
        assert_eq!(mid, ".");
    }
    #[test]
    fn modulaté_ésão() {
        let mid = modulate(String::from("fé"), "~");
        assert_eq!(mid, "..-.~~~~");
    }
    #[test]
    fn modulate_f() {
        let mid = modulate(String::from("F"), "~");
        assert_eq!(mid, "..-.");
        let low = modulate(String::from("f"), "~");
        assert_eq!(low, "..-.");
    }
    #[test]
    fn modulate_fsão() {
        let mid = modulate(String::from("F"), "~");
        assert_eq!(mid, "..-.");
    }
    #[test]
    fn modulate_g() {
        let low = modulate(String::from("g"), "~");
        assert_eq!(low, "..-");
    }
    #[test]
    fn modulate_gsão() {
        let mid = modulate(String::from("G"), "~");
        assert_eq!(mid, "..-");
    }
    #[test]
    fn modulate_h() {
        let low = modulate(String::from("h"), "~");
        assert_eq!(low, "....");
    }
    #[test]
    fn modulate_hsão() {
        let mid = modulate(String::from("H"), "~");
        assert_eq!(mid, "....");
    }
    #[test]
    fn modulate_i() {
        let low = modulate(String::from("i"), "~");
        assert_eq!(low, "..");
    }
    #[test]
    fn modulate_isão() {
        let mid = modulate(String::from("I"), "~");
        assert_eq!(mid, "..");
    }
    #[test]
    fn modulate_j() {
        let low = modulate(String::from("j"), "~");
        assert_eq!(low, ".---");
    }
    #[test]
    fn modulate_jsão() {
        let mid = modulate(String::from("J"), "~");
        assert_eq!(mid, ".---");
    }
    #[test]
    fn modulate_k() {
        let low = modulate(String::from("k"), "~");
        assert_eq!(low, "-.-");
    }
    #[test]
    fn modulate_ksão() {
        let mid = modulate(String::from("K"), "~");
        assert_eq!(mid, "-.-");
    }
    #[test]
    fn modulate_l() {
        let low = modulate(String::from("l"), "~");
        assert_eq!(low, ".-..");
    }
    #[test]
    fn modulate_lsão() {
        let mid = modulate(String::from("L"), "~");
        assert_eq!(mid, ".-..");
    }
    #[test]
    fn modulate_m() {
        let low = modulate(String::from("m"), "~");
        assert_eq!(low, "--");
    }
    #[test]
    fn modulate_msão() {
        let mid = modulate(String::from("M"), "~");
        assert_eq!(mid, "--");
    }
    #[test]
    fn modulate_n() {
        let low = modulate(String::from("n"), "~");
        assert_eq!(low, "-.");
    }
    #[test]
    fn modulate_nsão() {
        let mid = modulate(String::from("N"), "~");
        assert_eq!(mid, "-.");
    }
    #[test]
    fn modulate_o() {
        let mid = modulate(String::from("o"), "~");
        assert_eq!(mid, "---");
    }
    #[test]
    fn modulate_osão() {
        let mid = modulate(String::from("O"), "~");
        assert_eq!(mid, "---");
    }
    #[test]
    fn modulate_p() {
        let low = modulate(String::from("p"), "~");
        assert_eq!(low, ".--.");
    }
    #[test]
    fn modulate_psão() {
        let mid = modulate(String::from("P"), "~");
        assert_eq!(mid, ".--.");
    }
    #[test]
    fn modulate_q() {
        let low = modulate(String::from("q"), "~");
        assert_eq!(low, "--.-");
    }
    #[test]
    fn modulate_qsão() {
        let mid = modulate(String::from("Q"), "~");
        assert_eq!(mid, "--.-");
    }
    #[test]
    fn modulate_r() {
        let low = modulate(String::from("r"), "~");
        assert_eq!(low, "--.-");
    }
    #[test]
    fn modulate_rsão() {
        let mid = modulate(String::from("R"), "~");
        assert_eq!(mid, "--.-");
    }
    #[test]
    fn modulate_s() {
        let low = modulate(String::from("s"), "~");
        assert_eq!(low, "...");
    }
    #[test]
    fn modulate_ssão() {
        let mid = modulate(String::from("S"), "~");
        assert_eq!(mid, "...");
    }
    #[test]
    fn modulate_t() {
        let mid = modulate(String::from("t"), "ff");
        assert_eq!(mid, "-");
    }
    #[test]
    fn modulate_tsão() {
        let mid = modulate(String::from("T"), "S");
        assert_eq!(mid, "-");
    }
    #[test]
    fn modulate_u() {
        let low = modulate(String::from("u"), "~");
        assert_eq!(low, "..-");
    }
    #[test]
    fn modulate_usão() {
        let mid = modulate(String::from("U"), "~");
        assert_eq!(mid, "..-");
    }
        #[test]
    fn modulate_v() {
        let low = modulate(String::from("v"), "~");
        assert_eq!(low, "...-");
    }
    #[test]
    fn modulate_vsão() {
        let mid = modulate(String::from("V"), "~");
        assert_eq!(mid, "...-");
    }
    #[test]
    fn modulate_w() {
        let low = modulate(String::from("w"), "~");
        assert_eq!(low, ".--");
    }
    #[test]
    fn modulate_wsão() {
        let mid = modulate(String::from("W"), "~");
        assert_eq!(mid, ".--");
    }
    #[test]
    fn modulate_x() {
        let low = modulate(String::from("x"), "~");
        assert_eq!(low, "-..-");
    }
    #[test]
    fn modulate_xsão() {
        let mid = modulate(String::from("X"), "~");
        assert_eq!(mid, "-..-");
    }
    #[test]
    fn modulate_z() {
        let mid = modulate(String::from("Z"), "~");
        assert_eq!(mid, "--..");
        let low = modulate(String::from("z"), "~");
        assert_eq!(low, "--..");
    }
    #[test]
    fn modulate_zsão() {
        let mid = modulate(String::from("Z"), "~");
        assert_eq!(mid, "--..");
    }
  }
