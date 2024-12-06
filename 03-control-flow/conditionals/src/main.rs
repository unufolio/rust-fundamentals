fn main() {
    let proceed = false;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not Proceeding");
    }

    let height = 190;
    if height > 180 {
        println!("Tall");
    } else if height > 170 {
        println!("Average");
    } else {
        println!("Short");
    }
}
