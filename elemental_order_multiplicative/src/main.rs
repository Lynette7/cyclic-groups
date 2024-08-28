// Function to find the gcd of two integers recursively
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Generate the group of the Z mod m passed in main
fn multiplicative_group(modulus: u32) -> Vec<u32> {
    let mut group = Vec::new();
    for j in 1..modulus {
        let gcd = gcd(j, modulus);
        if gcd == 1 {
            group.push(j);
        }
    }
    group
}

fn elemental_order(g: u32, m: u32) -> u32 {
    let set = multiplicative_group(m);
    let mut result = 0;
    let mut power = 0;

    if !set.contains(&g) {
        println!("The integer {} is not an element of the group Z*{}!", g, m);
    } else {
        while result != 1 {
            power += 1;
            let multiple = u32::pow(g, power);
            result = multiple % m;
        }
        println!{"The order of element {} of the group Z*{} is {}!", g, m, power};
    }

    power
}

fn main() {
    let modulus = 10;
    let element = 7;
    elemental_order(element, modulus);
}
