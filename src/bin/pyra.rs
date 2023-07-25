fn main(){
    let args: Vec<String> = std::env::args().collect();  
    let x: i32 = args[1].parse().unwrap_or(0);  
    if x >= 0{
    for i in 1..=2*x.abs()-1 {
        if i<=x{
        println!("{}", "*".repeat(i.try_into().unwrap()));
        } else{
        println!("{}", "*".repeat((2*x-i).try_into().unwrap()));    
        }
       }
    }else{
    for i in 1..=2*x.abs()-1 {
        if i<=x.abs(){
            println!("{}{}"," ".repeat((x.abs()-i).try_into().unwrap()), "*".repeat(i.try_into().unwrap()));
            } else{
                println!("{}{}"," ".repeat((i-x.abs()).try_into().unwrap()), "*".repeat((2*x.abs()-i).try_into().unwrap()));
            }
        }    
    }
}