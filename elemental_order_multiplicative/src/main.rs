fn elemental_order(g: u32, m: u32) -> u32 {
    let mut result = 0;
    let mut power = 0;

    while result != 1 {
        power += 1;
        let multiple = u32::pow(g, power);
        result = multiple % m;
    }
    power
}

fn main() {
    let modulus = 10;
    let element = 9;
    let order = elemental_order(element, modulus);

    println!("The order of element {} from group Z*{} is {}!", element, modulus, order);
}
