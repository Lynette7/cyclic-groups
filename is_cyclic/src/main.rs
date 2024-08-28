fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn is_prime(n: u32) -> bool {
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

fn prime_factors(mut num: u32) -> Vec<u32> {
    let mut p = 2;
    let mut factors: Vec<u32> = Vec::new();

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

fn find_order(m: u32) -> u32 {
    let factors: Vec<u32> = prime_factors(m);
    let prime = is_prime(m);
    // let mut order = 0;

    if prime {
        let order = m - 1;
        return order;
    } else {
        // Calculate Euler's Totient function, φ(m)
        let mut phi = m;
        for &factor in &factors {
            phi = phi * (factor - 1) / factor;
        }
        let order = phi; // The order is φ(m)
        return order;
    }

    // order
}

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

// fn additive_group(modulus: u32) -> Vec<u32> {
//     let mut group = Vec::new();
//     for i in 0..modulus {
//         group.push(i);
//     }

//     group
// }

fn generate_group(g: u32, order: u32, m: u32) -> Vec<u32> {
    let mut set: Vec<u32> = Vec::new();
    for j in 1..order+1 {
        let element = u32::pow(g, j);
        let newelement = element % m;
        set.push(newelement);
    }
    set
}

fn is_cyclic(m: u32) -> bool {
    let order = find_order(m);
    println!("The order of this multiplicative group is {}", order);
    let set = multiplicative_group(m);
    println!("The elements of the group are {:?}", &set);
    for &g in &set {
        let subgroup = generate_group(g, order, m);
        // for i in set {
        //     for j in subgroup {
        //         if i == j{
        //             return true;
        //         }
        //     }
        // }
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
