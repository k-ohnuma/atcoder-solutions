use std::{cmp::Reverse, collections::BTreeMap};

use domain::model::problem::Problem;

pub struct ContestGroupCollection(pub BTreeMap<Reverse<String>, Vec<Problem>>);

pub struct ContestGroupPage {
    pub items: ContestGroupCollection,
    pub has_more: bool,
    pub total_contest_count: usize,
}

impl From<Vec<Problem>> for ContestGroupCollection {
    fn from(value: Vec<Problem>) -> Self {
        let mut map = BTreeMap::new();
        for p in value {
            let contest = p.contest_code.to_owned();
            map.entry(Reverse(contest)).or_insert(Vec::new()).push(p);
        }
        Self(map)
    }
}
