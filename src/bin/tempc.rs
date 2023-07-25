#[allow(non_snake_case)]
fn main(){
    let args: Vec<String> = std::env::args().collect();  
    let strP: i32 = args[1].parse().unwrap_or(0);  
    let endP: i32 = args[2].parse().unwrap_or(0); 
    let d: i32 = args[3].parse().unwrap_or(0); 
    println!("{}{}","Fahr","\tCelcius");
    if strP<endP{
    for i in 0..= (strP-endP).abs()/d {
        println!("{}{}{:.2}", strP+(i)*d,"\t", 5.0/9.0*((strP as f32 +i as f32 *d as f32)-32.0));
        }
    }
    else{
    for i in 0..= (strP-endP).abs()/d {
        println!("{}{}{:.2}", strP-(i)*d,"\t", 5.0/9.0*((strP as f32 -i as f32 *d as f32)-32.0));
        }
    }
}