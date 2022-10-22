const STARTING_MISSILES: i32 = 8; 
const READY_TO_LAUNCH: i32 = 2; 

fn main() {
    let (mut missiles, ready) = (STARTING_MISSILES, READY_TO_LAUNCH); 

    let _y = 10 ;

    println!("Firing {} of my {} missiles.", ready, missiles);

    missiles = missiles - ready;
    println!("{} missiles left", missiles);

    println!("Mul of 10, 30 is {}", multiply_stuff(10.0, 30.0));
}


fn multiply_stuff(num1: f64, num2: f64) -> f64 {
    num1 * num2
}
