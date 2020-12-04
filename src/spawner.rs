use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player, 
            pos, 
            Render{ 
                color: ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)), 
                glyph: to_cp437('@') 
            },
            Health{ current: 20, max: 20 }
        )
    );
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 20) {
        20 => ettin(),
        19 => ogre(),
        14..=18 => orc(),
        _ => goblin(),
    };

    let color = ColorPair::new(RGB::named(WHITE), RGB::named(BLACK));
    ecs.push(
        (
            Enemy,
            pos,
            Render{color, glyph},
            MovingRandomly,
            Health{ current: hp, max: hp },
            Name(name),
        )
    );
}

fn goblin() -> (i32, String, FontCharType) {
    (5, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (10, "Orc".to_string(), to_cp437('o'))
}

fn ogre() -> (i32, String, FontCharType) {
    (30, "Ogre".to_string(), to_cp437('O'))
}

fn ettin() -> (i32, String, FontCharType) {
    (50, "Ettin".to_string(), to_cp437('E'))
}