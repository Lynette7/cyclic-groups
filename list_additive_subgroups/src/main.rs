use std::collections::HashSet;

// Function to generate the subgroup of element g
fn subgroup(g: u32, m: u32) -> HashSet<u32> {
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
fn all_subgroups(m: u32) -> Vec<HashSet<u32>> {
    let mut subgroups = Vec::new();

    for g in 1..=m {
        if m % g == 0{
            let subgroup = subgroup(g, m);
            if !subgroups.contains(&subgroup) {
                subgroups.push(subgroup);
            }
        }
    }
    subgroups
}

fn main() {
    let modulus = 12;
    let subgroups = all_subgroups(modulus);

    println!("Subgroups of the additive group Z{} are:", modulus);
    for subgroup in subgroups {
        println!("{:?}", subgroup);
    }
}
