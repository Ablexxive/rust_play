use ron;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Debug)]
struct Wall {
    wall_x: f32,
    wall_y: f32,
    wall_height: f32,
    wall_width: f32,
}

#[derive(Deserialize, Debug)]
struct Walls {
    walls: Vec<Wall>,
}

fn main() {
    let ron_file = fs::read_to_string("tmp.ron").unwrap();
    dbg!(&ron_file);
    let seralized: Walls = ron::de::from_str(&ron_file).unwrap();
    dbg!(seralized);
}
