// Function to generate the elements of an additive group
fn additive_group(modulus: u32) -> Vec<u32> {
    let mut group = Vec::new();
    for i in 0..modulus {
        group.push(i);
    }

    group
}

fn elemental_order(g: u32, m: u32) -> u32 {
    let set = additive_group(m);
    let mut result = 1;
    let mut n = 0;

    if !set.contains(&g) {
        println!("The integer {} is not an element of the group Z+{}!", g, m);
    } else {
        while result != 0 {
            n += 1;
            let multiple = g * n;
            result = multiple % m;
        }
        println!{"The order of element {} of the group Z+{} is {}!", g, m, n};
    }

    n
}

fn main() {
    let modulus = 7;
    let element = 5;
    elemental_order(element, modulus);
}

