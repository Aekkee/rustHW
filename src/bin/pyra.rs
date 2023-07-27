fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1) == None {
        
    }
    else    {
    let x: i32 = args[1].parse().unwrap_or(0);
    for i in 1..=2 * x.abs() - 1 {
        if i <= x {
                println!("{}", "*".repeat(i.try_into().unwrap()));
        } else {
                println!("{}", "*".repeat((2 * x - i).try_into().unwrap()));
        }
    }
    }
}
