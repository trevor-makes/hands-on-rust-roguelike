use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let player_fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next().unwrap();
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    let screen_pos = *mouse_pos * 4;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    <(Entity, &Point, &Name)>::query()
        .iter(ecs)
        .filter(|(_, &pos, _)| pos == map_pos && player_fov.visible.contains(&pos))
        .for_each(|(entity, _, name)| {
            let display = match ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                Ok(health) => format!("{} : {} hp", &name.0, health.current),
                _ => name.0.clone(),
            };
            draw_batch.print(screen_pos, &display);
        });
    draw_batch.submit(10100).expect("Batch error");
}
