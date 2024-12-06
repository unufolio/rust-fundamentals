fn main() {
    // the difference between let mut message = String::from(""); and let mut message = "";
    // The first one creates a String (heap-allocated, mutable content).
    // The latter creates a &str (static, immutable content, rebindable with mut).
    let mut message = String::from("Name: Alfredo, Height: ");
    // this changes the String's value
    message.clear();
    let mut height = 190;
    height = 189;
    println!("{}{}", message, height);
}
