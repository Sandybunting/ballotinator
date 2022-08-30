use rand::Rng;
use rand_distr::{Normal, Distribution};
use polars::prelude::CsvWriter;
use polars::prelude::*;
use polars::prelude::{Result as PolarsResult};
use std::result::Result;
use std::fs::File;
use std::ops::Range;

const HUGHS_BUILDINGS: [&str; 6] = [
    "Wolfson",
    "MGA",
    "RTB",
    "MTB",
    "85 Banbury Rd",
    "85 Woodstock Rd",
];
// const BUILDINGS: [String; 6] = ["Wolfson".to_string(), "MGA".to_string(), "RTB".to_string(), "MTB".to_string(), "85 Banbury Rd".to_string(), "85 Woodstock Rd".to_string()];

fn main() {
    test_run(15, 10, 3);
}

fn test_run(group_number: u32, household_number: u32, building_number: u32) {
    let mut sample_ballot: Ballot = generate_sample_ballot(group_number, household_number, building_number);
    println!("{sb:?}", sb = sample_ballot.clone());
    let sizes: String = sample_ballot.excess_groups.clone().iter().map(|g| g.size().to_string()).collect::<Vec<String>>().join(", ");
    println!("Group sizes: {sizes}");
    // let default_building_order: Vec<String> =
    // HUGHS_BUILDINGS.iter().map(|s| s.to_string()).collect();
    let default_building_order: Vec<String> = sample_ballot.buildings.clone();
    println!("Default building order: {default_building_order:?}");
    sample_ballot.allocate_rooms(&default_building_order);
    println!("\n");
    println!("{sb:?}", sb = sample_ballot.clone());
    let excess_groups = sample_ballot.excess_groups.clone();
    println!("Excess groups: {excess_groups:?}");
    let mut df = DataFrame::from(sample_ballot);
    let mut file = File::create("ballot.csv").expect("could not create file");
    let result = CsvWriter::new(&mut file)
        .has_header(true)
        .with_delimiter(b',')
        .finish(&mut df);
    if let Err(e) = result {
        println!("Error writing CSV: {e}")
    }
}

fn generate_sample_ballot(
    group_number: u32,
    household_number: u32,
    building_number: u32,
) -> Ballot {
    fn building_from_number(num: u32) -> String {
        format!("Building {num}")
    }

    fn normal_u32_random_number(mean: f64, std_dev: f64) -> u32 {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(mean, std_dev).unwrap();
        let floatsample = normal.sample(&mut rng);
        match floatsample > 0.0 {
            true => floatsample as u32,
            false => 0,
        }
    }

    let mut rng = rand::thread_rng();

    // A ballot consissts of a list of buildings, groups, and accommodation
    let buildings: Vec<String> = (1..=building_number)
        .map(|i| building_from_number(i).to_string())
        .collect();

    // Generate groups:
    let mut groups: Vec<Group> = Vec::new();
    for i in 1..group_number {
        let groupsize = normal_u32_random_number(4.0, 2.0)+1;
        let mut members: Vec<Person> = Vec::new();
        for j in 1..=groupsize {
            members.push(Person {
                name: format!("Person {j} in group {i}").to_string(),
                score: rng.gen_range(1..=200)
            })
        }
        groups.push(Group {
            members,
            household_preferences: None,
            building_preferences: None,
        })
    }

    // Generate accommodation:
    let mut accommodation: Vec<Household> = Vec::new();
    for i in 1..household_number {
        // Generate household

        // let size = rng.gen_range(1..=10);
        let size = normal_u32_random_number(12.0, 4.0);
        let name = format!("Household {i}");
        let i = rng.gen_range(1..=building_number);
        let building = building_from_number(i);
        // Old code from when generate_sample_ballot used Hughs buildings
        // let building_index = rng.gen_range(0..HUGHS_BUILDINGS.len());
        // let building = HUGHS_BUILDINGS
        //     .get(building_index)
        //     .expect("Index should be in BUILDINGS")
        //     .to_string();
        accommodation.push(Household::new(name, size, building));
    } // left off here

    // let buildings: Vec<String> = BUILDINGS.iter().map(|s| s.to_string()).collect();

    Ballot {
        buildings,
        accommodation,
        excess_groups: groups,
    }
}

fn are_permutations<T>(v1: &Vec<T>, v2: &Vec<T>) -> bool
where
    T: Eq,
{
    v1.iter().all(|item| v2.contains(item))
}



// #[derive(Clone, Debug, Copy, PartialEq, EnumIter)]
// enum Building {
//     Wolfson,
//     MGA,
//     RTB,
//     MTB,
//     Banbury82,
//     Woodstock,
// }

// struct Building {
//     name: String,
//     households: Vec<Household>,
// }
// impl Building {
//     fn new(name: String, households: Vec<Household>) -> Self {
//         Self { name, households }
//     }
// }
