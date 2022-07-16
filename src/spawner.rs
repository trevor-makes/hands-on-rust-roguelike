use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Name("Player".to_string()),
        Render { color: ColorPair::new(WHITE, BLACK), glyph: to_cp437('@'), },
        Health { current: 3, max: 3 },
        FieldOfView::new(8),
    ));
}

fn goblin() -> (i32, String, FontCharType, i32) {
    (1, "Goblin".to_string(), to_cp437('g'), 6)
}

fn orc() -> (i32, String, FontCharType, i32) {
    (2, "Orc".to_string(), to_cp437('o'), 6)
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph, radius) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(), _ => orc(),
    };
    ecs.push((
        Enemy,
        AIState::ChasingPlayer,
        pos,
        Name(name),
        Render { color: ColorPair::new(WHITE, BLACK), glyph, },
        Health { current: hp, max: hp },
        FieldOfView::new(radius),
    ));
}

pub fn spawn_amulet(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render { color: ColorPair::new(WHITE, BLACK), glyph: to_cp437('|'), },
        Name("Amulet of Yala".to_string()),
    ));
}
