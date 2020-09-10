use std::io;

fn main() {
    println!("Enter your weight: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    let mars_weight = calculate_weight_on_mars(weight);
    let mars_weight_in_grams = mars_weight * 1000.0;
    println!(
        "Your weight on Mars would be {} kg or {} gr",
        mars_weight, mars_weight_in_grams
    );
}

fn calculate_weight_on_mars(weight_on_earth: f32) -> f32 {
    (weight_on_earth / 9.81) * 3.711
}
