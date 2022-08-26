use std::convert::TryFrom;

fn main() {}

fn allocate_rooms(
    accommodation: &mut Vec<Household>,
    groups: &mut Vec<Group>,
    default_building_order: &Vec<Building>,
) {
    let mut groups = groups.clone();
    groups.sort_unstable_by_key(|g| g.group_score());
    for group in groups.iter() {
        if let Some(household_preferences) = &group.household_preferences {
            for household in household_preferences.iter() {
                if household.is_full() {
                    continue;
                }
            }
        }
    }
}

fn attempt_to_join_household(group: &Group, household: &mut Household) {}

fn usize_to_u32(usize_value: &usize) -> u32 {
    let u32_version = u32::try_from(*usize_value);
    match u32_version {
        Ok(n) => n,
        Err(error) => panic!("{error}"),
    }
}

// fn generate_sample_ballot() -> Vec<Building> {
//     // let wolfson = Building {
//     //     name: String::from("Wolfson")
//     //     id: 1

//     // };
//     let b1 = Building {
//         name: "Building 1".to_string(),
//         households: vec![
//             Household::new(name="Floor".to_string(), 8),
//             Household::new(name="Floor 2".to_string(), 8)
//         ]
//     };
//     let v = vec![];
// }

struct Ballot {
    accommodation: Vec<Household>,
}

#[derive(Clone)]
struct Person {
    name: String,
    score: u32,
}

#[derive(Clone)]
struct Group {
    members: Vec<Person>,
    household_preferences: Option<Vec<Household>>,
    building_preferences: Option<Vec<Building>>,
}
impl Group {
    fn group_score(&self) -> u32 {
        self.members.iter().map(|m| m.score).sum()
    }
    // fn generate_test_group() -> Group {
    //     Group {
    //         members: vec![
    //             Person {
    //                 name: "John".to_string(),
    //                 score: 32,
    //             },
    //             Person {
    //                 name: "Jane".to_string(),
    //                 score: 30,
    //             },
    //         ],
    //     }
    // }
}

#[derive(Clone)]
struct Household {
    name: String,
    size: u32,
    occupants: Vec<Person>,
    building: Building,
}
impl Household {
    fn can_fit(&self, new_group: &Group) -> bool {
        // let num_occupants = u32::try_from(self.occupants.len());
        // let num_occupants = match num_occupants {
        //     Ok(n) => n,
        //     Err(error) => {
        //         let name = &self.name;
        //         let occupants = &self.occupants.len();
        //         let size = &self.size;
        //         panic!("Household {name} is unbelievably full—it has {occupants} occupants when its size is only {size}. Error: {error}");
        //     }
        // };

        return num_occupants + usize_to_u32(new_group.size) > self.size;
    }
}

#[derive(Clone)]
enum Building {
    Wolfson,
    MGA,
    RTB,
    MTB,
    Banbury82,
    Woodstock,
}

// struct Building {
//     name: String,
//     households: Vec<Household>,
// }
// impl Building {
//     fn new(name: String, households: Vec<Household>) -> Self {
//         Self { name, households }
//     }
// }
