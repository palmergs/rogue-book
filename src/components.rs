pub use crate::prelude::*;

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Player;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Enemy;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Item;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AmuletOfYala;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MovingRandomly;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChasingPlayer;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32
}

#[derive(Debug, Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct FieldOfView{
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self{ visible_tiles: HashSet::new(), radius, is_dirty: true }
    }

    pub fn clone_dirty(&self) -> Self {
        let mut cloned = self.clone();
        cloned.is_dirty = true;
        cloned
    }
}