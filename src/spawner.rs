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
            Health{ current: 100, max: 100 },
            FieldOfView::new(8),
        )
    );
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item{},
        AmuletOfYala{},
        pos,
        Render{color: ColorPair::new(WHITE, BLACK), glyph: to_cp437('&')},
        Name("Amulet of Yala".to_string()),
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 20) {
        20 => ettin(),
        19 => ogre(),
        14..=18 => orc(),
        _ => goblin(),
    };

    let color = ColorPair::new(RGB::named(WHITE), RGB::named(BLACK));
    match rng.roll_dice(1, 4) {
        1 => ecs.push((
            Enemy{},
            pos,
            Render{color, glyph},
            Health{ current: hp, max: hp },
            Name(name),
            MovingRandomly{},
            FieldOfView::new(6),
        )),
        _ => ecs.push((
            Enemy{},
            pos,
            Render{color, glyph},
            Health{ current: hp, max: hp },
            Name(name),
            ChasingPlayer{},
            FieldOfView::new(6),
        ))
    };
}

fn goblin() -> (i32, String, FontCharType) {
    (5, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (8, "Orc".to_string(), to_cp437('o'))
}

fn ogre() -> (i32, String, FontCharType) {
    (15, "Ogre".to_string(), to_cp437('O'))
}

fn ettin() -> (i32, String, FontCharType) {
    (25, "Ettin".to_string(), to_cp437('E'))
}
