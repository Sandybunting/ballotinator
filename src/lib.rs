// use rand::Rng;
// use rand_distr::{Normal, Distribution};
// use polars::prelude::CsvWriter;
use polars::prelude::*;
use polars::prelude::{Result as PolarsResult};
use std::result::Result;
// use std::fs::File;
// use std::ops::Range;

#[derive(Clone, Debug)]
struct Person {
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
    household_preferences: Option<Vec<usize>>, //vector of indices to the accomodation vector of households
    building_preferences: Option<Vec<String>>,
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
    occupants: Vec<Person>,
    building: String,
}
impl Household {
    fn new(name: String, size: u32, building: String) -> Self {
        Self {
            name,
            size,
            building,
            occupants: Vec::new(),
        }
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
        let occupants_num_u32: u32 = u32::try_from(self.occupants.len()).ok().expect("Occupants should be << 2^32");
        let new_group_members_num_u32: u32 = u32::try_from(new_group.members.len()).ok().expect("The number of new group members should be << 2^32");
        return occupants_num_u32 + new_group_members_num_u32 < self.size;
    }
    fn add_group(&mut self, new_group: &Group) {
        // NTS: Should probably refactor to include the can_fit logic and return a Result type
        let mut member_vec = new_group.members.clone();
        self.occupants.append(&mut member_vec);
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
            if let Some(household_preferences) = &group.household_preferences {
                for household_index in household_preferences.iter() {
                    let household = &mut accommodation[*household_index];
                    match household.attempt_to_add_group(&group) {
                        Ok(()) => continue 'group_loop,
                        Err(()) => continue,
                    }
                    
                    // if !(household.can_fit(&group)) {
                    //     continue; // "continue" skips this iteration of the loop and moves to the next one
                    // } else {
                    //     household.add_group(&group);
                    //     continue 'group_loop;
                    // }
                }
            }

            // second, try to fit the group into a household according to the building preference order
            let building_preferences = group
                .building_preferences
                .clone()
                .unwrap_or_else(|| default_building_order.clone());

            for building in building_preferences {
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
            accom.iter().map(|haus| haus.occupants.clone()).collect();
        let occupantsstringvec: Vec<String> = occupantsvecvec
            .iter()
            .map(|ov| {
                String::from(Group {
                    members: ov.clone(),
                    household_preferences: None,
                    building_preferences: None,
                })
            })
            .collect();
        println!("{osv:?}", osv = occupantsstringvec.clone());
        let s4: Series = Series::new("Occupants", occupantsstringvec);

        let df = DataFrame::new(vec![s1, s2, s3, s4]).expect("Couldn't create DataFrame");
        df
    }
}