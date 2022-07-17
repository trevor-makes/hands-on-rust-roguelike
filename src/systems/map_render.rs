use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(ecs: &SubWorld, #[resource] map: &Map, #[resource] camera: &Camera) {
    let player_fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next().unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y .. camera.bottom_y {
        for x in camera.left_x .. camera.right_x {
            let pos = Point::new(x, y);
            if map.in_bounds(pos) && map.revealed.contains(&pos) {
                let tint = if player_fov.visible.contains(&pos) { WHITE } else { GRAY };
                let idx = map_idx(x, y);
                let offset = Point::new(camera.left_x, camera.top_y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };
                draw_batch.set(pos - offset, ColorPair::new(tint, BLACK), glyph);
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
