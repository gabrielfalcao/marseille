use marseille::alphabet2morse;

const SS: &str = " ";
pub fn main() {
    let erst = "The first rule of the Fight Club is you do not talk about it";
    let stanza: String = String::from(erst);
    println!("{}", alphabet2morse(stanza, &SS))
}
