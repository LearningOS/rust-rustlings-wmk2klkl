// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a hint.



fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: String) -> bool {
    //对象类型不一致 都返回false
    // attempt == "green" || attempt == "blue" || attempt == "red"
    // let attempt = attempt.to_string();
    println!("{}|{}|{}",attempt == "green" ,attempt == "blue" ,attempt == "red");
    attempt == "green" || attempt == "blue" || attempt == "red"

}
