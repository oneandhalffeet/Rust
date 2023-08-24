use std::io;
fn main() {
    println!("Please Enter weight on Earth in kilos:");
    let mut user_weight = String::new();

    io::stdin().read_line(&mut user_weight).unwrap();

    let weight_earth: f32 = user_weight.trim().parse().unwrap();

    // borrow_string(&user_weight);
    // own_string(user_weight);
    dbg!(weight_earth);
    dbg!(user_weight);
    // println!("Input = {}, {}", user_weight, weight);
    let weight = calc_weight_on_mars(weight_earth);
    // weight = weight*1000.0;
    println!("My weight is {}Kg on Mars", weight);
}

fn calc_weight_on_mars(weight: f32) -> f32 {
    weight*3.711/9.81
}

// fn borrow_string(s: &String){
//     println!("{} borrow", s);
// }

// fn own_string(s: String){
//     println!("{} own", s);
// }