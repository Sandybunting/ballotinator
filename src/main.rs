use rand::Rng;
use std::fmt;
use std::io;
use strum::{EnumCount, IntoEnumIterator, VariantNames};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter, IntoStaticStr, EnumVariantNames};

fn main() {}

fn generate_sample_ballot(
    group_number: u32,
    household_number: u32,
    building_number: u32,
) -> Ballot {
    // A ballot consissts of a list of buildings, groups, and accommodation
    let buildings = Building::;

    // Generate groups:
    let mut groups: Vec<Group> = Vec::new();
    for i in 1..group_number {
        let mut members: Vec<Person> = Vec::new();
        for j in 1..i * 2 {
            members.push(Person {
                name: format!("Person {j} in group {i}").to_string(),
                score: i * 10 + j * 4,
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
        let mut rng = rand::thread_rng();
        let size = (8i32 + rng.gen_range::<i32>(-8..8)) as u32;
        Household {
            name: format!("Household {i}"),
            size:
        }
    } // left off here
}

struct Ballot {
    buildings: [Building; Building::COUNT],
    accommodation: Vec<Household>,
    groups: Vec<Group>,
}
impl Ballot {
    fn allocate_rooms(
        &mut self,
        default_building_order: &[Building; Building::COUNT],
    ) -> Vec<Group> {
        let mut accommodation = &mut self.accommodation;
        let mut groups = &mut self.groups;
        let mut excess_groups: Vec<Group> = Vec::new();

        // let mut groups = groups.clone(); // if you don't want to mutate the original groups passed in
        groups.sort_unstable_by(|a, b| {
            a.avg_score()
                .partial_cmp(&b.avg_score())
                .expect("Not a NaN")
        }); // Since a, b are f64s, they do not have the Ord (total order) trait as f64s can be NaN, so you must implement a comparator function manually.
        'group_loop: for group in groups.iter() {
            // first, try to satisfy the specific household preferences
            if let Some(household_preferences) = &group.household_preferences {
                for household_index in household_preferences.iter() {
                    let household = &mut accommodation[*household_index];
                    if !(household.can_fit(&group)) {
                        continue; // "continue" skips this iteration of the loop and moves to the next one
                    } else {
                        household.add_group(&group);
                        continue 'group_loop;
                    }
                }
            }

            // second, try to fit the group into a household according to the building preference order
            let building_preferences = group
                .building_preferences
                .clone()
                .unwrap_or_else(|| *default_building_order);

            for &building in &building_preferences {
                // let relevant_households: Vec<Household> =
                //     Vec::from_iter(accommodation.iter().filter(|h| match h.building {
                //         building => true,
                //         _ => false,
                //     }));
                // println!("{building:?}");

                // let relevant_households: Vec<Household> = accommodation
                //     .iter()
                //     .filter(|h| match h.building {
                //         building => true,
                //         _ => false,
                //     })
                //     .collect();
                // let test: i32 = &accommodation;

                for household in accommodation.iter_mut() {
                    // if !self.buildings.contains(&household.building) {
                    //     let hb = &household.building;
                    //     let hn = &household.name;
                    //     panic!("The household {hn} is listed as being in building {hb}, but this is not in the building list of the ballot!")
                    // }
                    if !household.can_fit(&group) || household.building != building {
                        continue;
                    } else {
                        household.add_group(&group);
                        continue 'group_loop;
                    }
                }
            }

            // third, if the group cannot be fit into a household, we add them to the excess_groups vec which will be returned
            excess_groups.push(group.clone())
        }
        return excess_groups;
    }
}

#[derive(Clone)]
struct Person {
    name: String,
    score: u32,
}

#[derive(Clone)]
struct Group {
    members: Vec<Person>,
    household_preferences: Option<Vec<usize>>, //vector of indices to the accomodation vector of households
    building_preferences: Option<[Building; Building::COUNT]>,
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

#[derive(Clone)]
struct Household {
    name: String,
    size: u32,
    occupants: Vec<Person>,
    building: Building,
}
impl Household {
    fn new(name: String, size: u32, building: Building) -> Self {
        Self {
            name,
            size,
            building,
            occupants: Vec::new(),
        }
    }
    fn can_fit(&self, new_group: &Group) -> bool {
        // A household can fit a group if its current occupants combined with the group's members is not bigger than its size

        // use the "as" type cast expression to coerce usizes into u32s
        return self.occupants.len() as u32 + new_group.members.len() as u32 > self.size;
    }
    fn add_group(&mut self, new_group: &Group) {
        // NTS: Should probably refactor to include the can_fit logic and return a Result type
        if self.can_fit(new_group) {
            let mut member_vec = new_group.members.clone();
            self.occupants.append(&mut member_vec);
        } else {
            panic!();
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, EnumIter, IntoStaticStr, EnumCountMacro, EnumVariantNames)]
enum Building {
    Wolfson,
    MGA,
    RTB,
    MTB,
    Banbury85,
    Woodstock82,
}

impl fmt::Display for Building {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let building_name: &'static str = self.into();
        write!(f, "{building_name}")
    }
}
