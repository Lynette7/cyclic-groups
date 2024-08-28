// The logical steps for this code are:
// 1. Find the order of the group using φ(m) - find_order(m: usize)
// 2. Find the members of the multiplicative group i.e integers that are relatively prime to the mod - multiplicative_group(modulus)
// 3. Find the generators and print them.

// use num_bigint::BigUint;
// use num_traits::One;

// Function to find the gcd of two integers recursively
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Function to determine if an integer is prime
fn is_prime(n: usize) -> bool {
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
fn prime_factors(mut num: usize) -> Vec<usize> {
    let mut p = 2;
    let mut factors: Vec<usize> = Vec::new();

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
fn find_order(m: usize) -> usize {
    let factors: Vec<usize> = prime_factors(m);
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
fn multiplicative_group(modulus: usize) -> Vec<usize> {
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
fn generate_group(g: usize, order: usize, m: usize) -> Vec<usize> {
    let mut set: Vec<usize> = Vec::new();
    for j in 1..order+1 {
        let element = usize::pow(g, j as u32);
        let newelement = element % m;
        set.push(newelement);
    }
    set
}

// Function to find the generators of the group and print them out
fn generators(m: usize) -> Vec<usize> {
    let order = find_order(m);
    let set = multiplicative_group(m);
    println!("The elements of the group Z*{} are {:?}", m, &set);
    let mut generators = Vec::new();

    for &g in &set {
        let subgroup = generate_group(g, order, m);
        if set.len() == subgroup.len() && set.iter().all(|x| subgroup.contains(x)) {
            generators.push(g);
        }
    }

    if generators.is_empty() {
        println!("No generators found for Z*{}!", m);
    } else {
        println!("Generators for Z*{}: {:?}", m, generators);
    }

    generators
}

fn main() {
    let modulus = 4900;
    generators(modulus);
}
