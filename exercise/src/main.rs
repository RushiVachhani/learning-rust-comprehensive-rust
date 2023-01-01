fn main() {
    let mut x: i32 = 6; // mutable variable binding
    print!("{x}"); //Macro for printing like printf
    while x != 1 {
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = 3 * x + 1
        }
        print!("-> {x}");
    }
    println!();
}
