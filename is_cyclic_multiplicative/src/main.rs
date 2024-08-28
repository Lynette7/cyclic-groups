// The logical steps for this code are:
// 1. Find the order of the group using φ(m) - find_order(m: u128)
// 2. Find the members of the multiplicative group i.e integers that are relatively prime to the mod - multiplicative_group(modulus)
// 3. Determine if the group is cyclic by generating the subgroups for each element in the group mod m and comparing it to the group - is_cyclic(m)
// ** The is_cyclic function uses generate_group(g, order, m) to generate the groups for each element

// Function to find the gcd of two integers recursively
fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Function to determine if an integer is prime
fn is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }
    true
}

// Function to find the prime factors of an integer
fn prime_factors(mut num: u128) -> Vec<u128> {
    let mut p = 2;
    let mut factors: Vec<u128> = Vec::new();

    while num >= p * p {
        if num % p == 0 {
            if factors.last() != Some(&p) {
                factors.push(p);
            }
            num /= p;
        } else {
            p += 1;
        }
    }
    
    if num > 1 && factors.last() != Some(&num) {
        factors.push(num);
    }

    return factors;
}

// Function to find the order of a group using Euler Phi
fn find_order(m: u128) -> u128 {
    let factors: Vec<u128> = prime_factors(m);
    let prime = is_prime(m);

    if prime {
        let order = m - 1;
        return order;
    } else {
        // Calculate Euler's Totient function, φ(m)
        let mut phi = m;
        for &factor in &factors {
            phi = phi * (factor - 1) / factor;
        }
        let order = phi;
        return order;
    }
}

// Generate the group of the Z mod m passed in main
fn multiplicative_group(modulus: u128) -> Vec<u128> {
    let mut group = Vec::new();
    for j in 1..modulus {
        let gcd = gcd(j, modulus);
        if gcd == 1 {
            group.push(j);
        }
    }
    group
}

// Generate the subgroups for each element in the group
fn generate_group(g: u128, order: u128, m: u128) -> Vec<u128> {
    let mut set: Vec<u128> = Vec::new();
    for j in 1..order+1 {
        let element = u128::pow(g, j as u32);
        let newelement = element % m;
        set.push(newelement);
    }
    set
}

// Determine if the multiplicative group is cyclic
fn is_cyclic(m: u128) -> bool {
    let order = find_order(m);
    println!("The order of this multiplicative group is {}", order);
    let set = multiplicative_group(m);
    println!("The elements of the group are {:?}", &set);
    for &g in &set {
        let subgroup = generate_group(g, order, m);
        if set.len() == subgroup.len() && set.iter().all(|x| subgroup.contains(x)) {
            return true;
        }
    }
    false
}

fn main() {
    let modulus = 4900;
    let cyclic = is_cyclic(modulus);
    if cyclic == true {
        println!("The multipilcative group Z*{} is cyclic!", modulus);
    } else {
        println!("The multiplicative group Z*{} is not cyclic!", modulus);
    }
}
