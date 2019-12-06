use std::env;
use std::fs;

fn fuel_for_mass(mass: f32) -> f32
{
    (mass as f32 / 3.0).floor() - 2.0
}

fn fuel_for_mass_and_fuel(mass: f32) -> f32
{
    let fuel = (mass as f32 / 3.0).floor() - 2.0;
    if fuel > 0.0
    {
        let fuel_for_fuel = fuel_for_mass_and_fuel(fuel);
        return fuel + fuel_for_fuel;
    }
    else
    {
        return 0.0;
    }
}

fn main() {
    // --snip--
    let data = fs::read_to_string("data/1")
        .expect("Something went wrong reading the file");
    let module_masses : Vec<i32> = data.split("\n")
                      .map(|mass_str| mass_str.parse::<i32>().unwrap())
                      .collect();
    let module_masses_2 = module_masses.clone();
    let needed_fuel_1 : f32 = module_masses.into_iter().map(|mass_int| fuel_for_mass(mass_int as f32)) 
                      .sum();
    let needed_fuel_2 : f32 = module_masses_2.into_iter().map(|mass_int| fuel_for_mass_and_fuel(mass_int as f32)) 
                      .sum();

    println!("1: {}", needed_fuel_1);
    println!("2: {}", needed_fuel_2);
}