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

#[derive(Clone, Debug)]
pub struct Person {
    name: String,
    score: u32,
}

impl From<Person> for String {
    fn from(item: Person) -> String {
        let name = item.name;
        let score = item.score;
        format!("{name} [{score}]")
    }
}

#[derive(Clone, Debug)]
struct Group {
    members: Vec<Person>,
    household_preferences: Vec<Option<String>>,
    building_preferences: Option<Vec<String>>,
    splitting_allowed: bool
}

impl From<Group> for String {
    fn from(item: Group) -> String {
        let person_string_list: Vec<String> = item
            .members
            .iter()
            .map(|p| String::from(p.clone()))
            .collect();
        let group_string: String = person_string_list.join(", ");
        group_string
    }
}
impl Group {
    fn score(&self) -> u32 {
        self.members.iter().map(|m| m.score).sum()
    }
    fn size(&self) -> u32 {
        self.members.len() as u32
    }
    fn avg_score(&self) -> f64 {
        let score_float = self.score() as f64;
        let size_float = self.size() as f64;
        score_float / size_float
    }
}

#[derive(Clone, Debug)]
struct Household {
    name: String,
    size: u32,
    groups: Vec<Group>,
    building: String,
}
impl Household {
    fn new(name: String, size: u32, building: String) -> Self {
        Self {
            name,
            size,
            building,
            groups: Vec::new(),
        }
    }
    fn occupants(&self) -> Vec<Person> {
        self.groups.iter().map(|group| group.members.clone()).flatten().collect()
    }
    fn attempt_to_add_group(&mut self, new_group: &Group) -> Result<(), ()> {
        if self.can_fit(new_group) {
            self.add_group(new_group);
            return Ok(())
        } else {
            return Err(())
        }
    }
    fn can_fit(&self, new_group: &Group) -> bool {
        // A household can fit a group if its current occupants combined with the group's members is not bigger than its size
        // use the "as" type cast expression to coerce usizes into u32s
        let occupants_num: u32 = self.groups.iter().map(|g| g.size()).sum();
        let new_group_members_num: u32 = u32::try_from(new_group.members.len()).ok().expect("The number of new group members should be << 2^32");
        return occupants_num + new_group_members_num < self.size;
    }
    fn add_group(&mut self, new_group: &Group) {
        self.groups.push(new_group.clone());
    }
}

#[derive(Debug, Clone)]
struct Ballot {
    buildings: Vec<String>,
    accommodation: Vec<Household>,
    excess_groups: Vec<Group>,
}

impl Ballot {
    fn allocate_rooms(&mut self, default_building_order: &Vec<String>) {
        // Check that the default building order is a permutation of the actual buildings avaialable
        if !are_permutations(&self.buildings, &default_building_order) {
            let ballotbuildings = self.buildings.clone();
            println!("Ballot buildings available: {ballotbuildings:?}");
            println!("Default building order: {default_building_order:?}");
            panic!("The available ballot buildings vector is NOT a permutation of the default building order vector!")
        }
        let accommodation = &mut self.accommodation;
        let old_excess_groups = &mut self.excess_groups;
        let mut new_excess_groups: Vec<Group> = Vec::new();

        old_excess_groups.sort_unstable_by(|a, b| {
            a.avg_score()
                .partial_cmp(&b.avg_score())
                .expect("Average score is NaN—is a group size 0?")
        }); // Since a, b are f64s, they do not have the Ord (total order) trait as f64s can be NaN, so you must implement a comparator function manually.
        'group_loop: for group in old_excess_groups.iter() {
            // first, try to satisfy the specific household preferences
            for opt in group.household_preferences.iter() {
                if let Some(preferred_household_name) = opt {
                    for household in accommodation.iter_mut() {
                        if household.name == *preferred_household_name {
                            match household.attempt_to_add_group(&group) {
                                Ok(()) => continue 'group_loop,
                                Err(()) => continue,
                            }
                        }
                    }
                }
            }

            // second, try to fit the group into a household according to the building preference order
            let building_preferences = group
                .building_preferences
                .clone()
                .unwrap_or_else(|| default_building_order.clone());

            for building in building_preferences {
                if !self.buildings.contains(&building) {
                    panic!("The building {building} is not in the building list of the ballot!")
                }

                for household in accommodation.iter_mut() {
                    if !self.buildings.contains(&household.building) {
                        let hb = &household.building;
                        let hn = &household.name;
                        panic!("The household {hn} is listed as being in building {hb}, but this is not in the building list of the ballot!")
                    }
                    if household.building == building {
                        match household.attempt_to_add_group(&group) {
                            Ok(()) => continue 'group_loop,
                            Err(()) => continue,
                        }
                    }
                }
            }

            // third, if the group cannot be fit into a household, we add them to the excess_groups vec which will be returned
            new_excess_groups.push(group.clone())
        }
        self.excess_groups = new_excess_groups;
    }
}

impl From<Ballot> for DataFrame {
    fn from(item: Ballot) -> Self {
        let accom: Vec<Household> = item.accommodation;
        let s1: Series = Series::new(
            "Household name",
            accom
                .iter()
                .map(|haus| haus.name.clone())
                .collect::<Vec<String>>(),
        );
        let s2: Series = Series::new(
            "Building",
            accom
                .iter()
                .map(|haus| haus.building.clone())
                .collect::<Vec<String>>(),
        );
        let s3: Series = Series::new(
            "Size",
            accom.iter().map(|haus| haus.size).collect::<Vec<u32>>(),
        );
        let occupantsvecvec: Vec<Vec<Person>> =
            accom.iter().map(|haus| haus.occupants().clone()).collect();
        let occupantsstringvec: Vec<String> = occupantsvecvec
            .iter()
            .map(|ov| {
                String::from(Group {
                    members: ov.clone(),
                    household_preferences: Vec::new(),
                    building_preferences: None,
                    splitting_allowed: false
                })
            })
            .collect();
        println!("{osv:?}", osv = occupantsstringvec.clone());
        let s4: Series = Series::new("Occupants", occupantsstringvec);

        let df = DataFrame::new(vec![s1, s2, s3, s4]).expect("Couldn't create DataFrame");
        df
    }
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
            household_preferences: Vec::new(),
            building_preferences: None,
            splitting_allowed: true,
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
