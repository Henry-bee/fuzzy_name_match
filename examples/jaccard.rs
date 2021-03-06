extern crate name_match;

use name_match::jaccard::compare;
use name_match::prelude::*;
use std::fmt;

fn main(){
    let name_1 = "James Bay";
    let name_2 = "James Sancho Adam";

    let name_matcher =
        compare::JaccardMatcher::default();
    let score = name_matcher.get_score(name_1, name_2);
    println!("Jaccard Similarity = {}", score);
}