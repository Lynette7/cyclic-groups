// This is a program to verify Lagrange's theorem that states: the order of any subgroup H of group G should divide the order of group G
// 1. First generate the subgroups of the group then

use std::collections::HashSet;

// Function to generate the subgroup of element g
fn generate_subgroup(g: u32, m: u32) -> HashSet<u32> {
    let mut subgroup = HashSet::new();
    let mut element = 1;
    let mut i = 0;

    while element != 0 {
        element = (g + i) % m;
        subgroup.insert(element);
        i += g;
    }
    subgroup
}

// Function to find all subgroups of the group
fn list_all_subgroups(m: u32) -> Vec<HashSet<u32>> {
    let mut subgroups = Vec::new();

    for g in 1..=m {
        if m % g == 0{
            let subgroup = generate_subgroup(g, m);
            if !subgroups.contains(&subgroup) {
                subgroups.push(subgroup);
            }
        }
    }
    subgroups
}

// Function to verify Lagrange's theorem
fn verify_lagrange_theorem(subgroups: &[HashSet<u32>], group_order: u32) -> bool {
    for subgroup in subgroups {
        let subgroup_order = subgroup.len() as u32;
        if group_order % subgroup_order != 0 {
            return false;
        }
    }
    true
}

fn main() {
    let modulus = 12;
    let subgroups = list_all_subgroups(modulus);

    println!("Subgroups of the additive group Z{} are:", modulus);
    for subgroup in &subgroups {
        println!("{:?}", subgroup);
    }

    let lagrange_verified = verify_lagrange_theorem(&subgroups, modulus);
    if lagrange_verified {
        println!("\nLagrange's theorem holds true!!",);
    } else {
        println!("Error: Lagrange's theorem does not hold.");
    }

    println!("\nThe orders of the subgroups of additive group Z{} are:", modulus);
    for subgroup in &subgroups {
        println!("Subgroup {:?} has order {}", subgroup, subgroup.len());
    }
}