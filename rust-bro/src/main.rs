fn main() {
    let mut i = 0;

    loop {
        match i {
            2 => {
                break
                println!("Yes")
            },
            _ => println!("No"),
        }
        i += 1;
    }
}
