fn main() {
    println!("Hello, world!");
    let we = calc_weight_on_mars(100.0);
    println!("My weight is {}", we);
}

fn calc_weight_on_mars(weight: f32) -> f32 {
    weight*3.711/9.81
}