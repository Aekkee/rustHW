fn main() {
    let args: Vec<String> = std::env::args().collect();
    let x: i64 = args[1].parse().unwrap_or(0);
    match x {
        0..=49 => println!("Failed with F"),
        50..=60 => println!("D"),
        61..=70 => println!("C"),
        71..=80 => println!("B"),
        81..=94 => println!("A"),
        95..=100 => println!("Excellent with A+"),
        _ => println!("Invalid score"),
    }
}
