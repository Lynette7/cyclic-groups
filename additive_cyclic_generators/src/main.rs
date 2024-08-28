// Function to find the gcd of two integers recursively
fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn additive_group(modulus: u128) -> Vec<u128> {
    let mut group = Vec::new();
    for i in 0..modulus {
        group.push(i);
    }

    group
}

// fn generate_group(g: u128, order: u128, m: u128) -> Vec<u128> {
//     let mut set: Vec<u128> = Vec::new();
//     for j in 1..order+1 {
//         let element = u128::pow(g, j as u32);
//         let newelement = element % m;
//         set.push(newelement);
//     }
//     set
// }

fn generators(m: u128) -> Vec<u128> {
    // The order of additive group Zn is n
    println!("The order of this additive group is {}", m);
    let set = additive_group(m);
    println!("The elements of the group are {:?}", &set);
    let mut generators = Vec::new();
    for &g in &set {
        let gcd = gcd(g, m);
        if gcd == 1 {
            generators.push(g);
        }
    }
    generators
}

fn main() {
    let modulus = 10;
    println!("The additive group Z{} is cyclic!", modulus);
    let generators = generators(modulus);
    println!("The generators for additive group Z{} are: {:?}", modulus, generators);
}

