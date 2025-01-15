use marseille::*;

#[test]
fn alphabet2morse_a() {
    let mid = alphabet2morse(String::from("a"), "~");
    assert_eq!(mid, ".-");
}
#[test]
fn alphabet2morse_asão() {
    let mid = alphabet2morse(String::from("A"), "~");
    assert_eq!(mid, ".-");
}
#[test]
fn alphabet2morse_b() {
    let mid = alphabet2morse(String::from("b"), "~");
    assert_eq!(mid, "-...");
}
#[test]
fn alphabet2morse_bsão() {
    let mid = alphabet2morse(String::from("B"), "~");
    assert_eq!(mid, "-...");
}
#[test]
fn alphabet2morse_c() {
    let mid = alphabet2morse(String::from("c"), "~");
    assert_eq!(mid, "-.-.");
}
#[test]
fn alphabet2morse_csão() {
    let mid = alphabet2morse(String::from("C"), "~");
    assert_eq!(mid, "-.-.");
}
#[test]
fn alphabet2morse_d() {
    let mid = alphabet2morse(String::from("d"), "~");
    assert_eq!(mid, "-..");
}
#[test]
fn alphabet2morse_dsão() {
    let mid = alphabet2morse(String::from("D"), "~");
    assert_eq!(mid, "-..");
}
#[test]
fn alphabet2morse_e() {
    let mid = alphabet2morse(String::from("E"), "~");
    assert_eq!(mid, ".");
    let mid = alphabet2morse(String::from("e"), "~");
    assert_eq!(mid, ".");
}

#[test]
fn alphabet2morse_esão() {
    let mid = alphabet2morse(String::from("E"), "~");
    assert_eq!(mid, ".");
}
#[test]
fn modulaté_ésão() {
    let mid = alphabet2morse(String::from("fé"), "~");
    assert_eq!(mid, "..-.~~~~");
}
#[test]
fn alphabet2morse_f() {
    let mid = alphabet2morse(String::from("F"), "~");
    assert_eq!(mid, "..-.");
    let mid = alphabet2morse(String::from("f"), "~");
    assert_eq!(mid, "..-.");
}
#[test]
fn alphabet2morse_fsão() {
    let mid = alphabet2morse(String::from("F"), "~");
    assert_eq!(mid, "..-.");
}
#[test]
fn alphabet2morse_g() {
    let mid = alphabet2morse(String::from("g"), "~");
    assert_eq!(mid, "--.");
}
#[test]
fn alphabet2morse_gsão() {
    let mid = alphabet2morse(String::from("G"), "~");
    assert_eq!(mid, "--.");
}
#[test]
fn alphabet2morse_h() {
    let mid = alphabet2morse(String::from("h"), "~");
    assert_eq!(mid, "....");
}
#[test]
fn alphabet2morse_hsão() {
    let mid = alphabet2morse(String::from("H"), "~");
    assert_eq!(mid, "....");
}
#[test]
fn alphabet2morse_i() {
    let mid = alphabet2morse(String::from("i"), "~");
    assert_eq!(mid, "..");
}
#[test]
fn alphabet2morse_isão() {
    let mid = alphabet2morse(String::from("I"), "~");
    assert_eq!(mid, "..");
}
#[test]
fn alphabet2morse_j() {
    let mid = alphabet2morse(String::from("j"), "~");
    assert_eq!(mid, ".---");
}
#[test]
fn alphabet2morse_jsão() {
    let mid = alphabet2morse(String::from("J"), "~");
    assert_eq!(mid, ".---");
}
#[test]
fn alphabet2morse_k() {
    let mid = alphabet2morse(String::from("k"), "~");
    assert_eq!(mid, "-.-");
}
#[test]
fn alphabet2morse_ksão() {
    let mid = alphabet2morse(String::from("K"), "~");
    assert_eq!(mid, "-.-");
}
#[test]
fn alphabet2morse_l() {
    let mid = alphabet2morse(String::from("l"), "~");
    assert_eq!(mid, ".-..");
}
#[test]
fn alphabet2morse_lsão() {
    let mid = alphabet2morse(String::from("L"), "~");
    assert_eq!(mid, ".-..");
}
#[test]
fn alphabet2morse_m() {
    let mid = alphabet2morse(String::from("m"), "~");
    assert_eq!(mid, "--");
}
#[test]
fn alphabet2morse_msão() {
    let mid = alphabet2morse(String::from("M"), "~");
    assert_eq!(mid, "--");
}
#[test]
fn alphabet2morse_n() {
    let mid = alphabet2morse(String::from("n"), "~");
    assert_eq!(mid, "-.");
}
#[test]
fn alphabet2morse_nsão() {
    let mid = alphabet2morse(String::from("N"), "~");
    assert_eq!(mid, "-.");
}
#[test]
fn alphabet2morse_o() {
    let mid = alphabet2morse(String::from("o"), "~");
    assert_eq!(mid, "---");
}
#[test]
fn alphabet2morse_osão() {
    let mid = alphabet2morse(String::from("O"), "~");
    assert_eq!(mid, "---");
}
#[test]
fn alphabet2morse_p() {
    let mid = alphabet2morse(String::from("p"), "~");
    assert_eq!(mid, ".--.");
}
#[test]
fn alphabet2morse_psão() {
    let mid = alphabet2morse(String::from("P"), "~");
    assert_eq!(mid, ".--.");
}
#[test]
fn alphabet2morse_q() {
    let mid = alphabet2morse(String::from("q"), "~");
    assert_eq!(mid, "--.-");
}
#[test]
fn alphabet2morse_qsão() {
    let mid = alphabet2morse(String::from("Q"), "~");
    assert_eq!(mid, "--.-");
}
#[test]
fn alphabet2morse_r() {
    let mid = alphabet2morse(String::from("r"), "~");
    assert_eq!(mid, ".-.");
}
#[test]
fn alphabet2morse_rsão() {
    let mid = alphabet2morse(String::from("R"), "~");
    assert_eq!(mid, ".-.");
}
#[test]
fn alphabet2morse_s() {
    let mid = alphabet2morse(String::from("s"), "~");
    assert_eq!(mid, "...");
}
#[test]
fn alphabet2morse_ssão() {
    let mid = alphabet2morse(String::from("S"), "~");
    assert_eq!(mid, "...");
}
#[test]
fn alphabet2morse_t() {
    let mid = alphabet2morse(String::from("t"), "ff");
    assert_eq!(mid, "-");
}
#[test]
fn alphabet2morse_tsão() {
    let mid = alphabet2morse(String::from("T"), "S");
    assert_eq!(mid, "-");
}
#[test]
fn alphabet2morse_u() {
    let mid = alphabet2morse(String::from("u"), "~");
    assert_eq!(mid, "..-");
}
#[test]
fn alphabet2morse_usão() {
    let mid = alphabet2morse(String::from("U"), "~");
    assert_eq!(mid, "..-");
}
#[test]
fn alphabet2morse_v() {
    let mid = alphabet2morse(String::from("v"), "~");
    assert_eq!(mid, "...-");
}
#[test]
fn alphabet2morse_vsão() {
    let mid = alphabet2morse(String::from("V"), "~");
    assert_eq!(mid, "...-");
}
#[test]
fn alphabet2morse_w() {
    let mid = alphabet2morse(String::from("w"), "~");
    assert_eq!(mid, ".--");
}
#[test]
fn alphabet2morse_wsão() {
    let mid = alphabet2morse(String::from("W"), "~");
    assert_eq!(mid, ".--");
}
#[test]
fn alphabet2morse_x() {
    let mid = alphabet2morse(String::from("x"), "~");
    assert_eq!(mid, "-..-");
}
#[test]
fn alphabet2morse_xsão() {
    let mid = alphabet2morse(String::from("X"), "~");
    assert_eq!(mid, "-..-");
}
#[test]
fn alphabet2morse_z() {
    let mid = alphabet2morse(String::from("Z"), "~");
    assert_eq!(mid, "--..");
    let mid = alphabet2morse(String::from("z"), "~");
    assert_eq!(mid, "--..");
}
#[test]
fn alphabet2morse_zsão() {
    let mid = alphabet2morse(String::from("Z"), "~");
    assert_eq!(mid, "--..");
}
