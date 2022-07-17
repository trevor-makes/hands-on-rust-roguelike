pub use crate::prelude::*;

use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIState {
    MovingRandomly,
    ChasingPlayer,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove(pub Point);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlockedBy(pub Entity);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack(pub Entity);

#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self { visible: HashSet::new(), radius, is_dirty: true, }
    }

    pub fn clone_dirty(&self) -> Self {
        Self { visible: HashSet::new(), radius: self.radius, is_dirty: true, }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala;
