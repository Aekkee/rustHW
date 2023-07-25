fn main(){
    let args: Vec<String> = std::env::args().collect();  
    let x: i64 = args[1].parse().unwrap_or(0);  
match x {
    0 ..= 49 => println!("Failed with F"),
    50 ..= 60 => println!("bad"),
    61 => println!("ad"),
    _ => println!("gg")
}
}