use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(FieldOfView)]
#[read_component(Player)]
pub fn fov(ecs: &mut SubWorld, #[resource] map: &mut Map) {
    let &player = <Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next().unwrap();
    <(Entity, &Point, &mut FieldOfView)>::query()
        .iter_mut(ecs)
        .filter(|(_, _, fov)| fov.is_dirty)
        .for_each(|(&entity, &pos, mut fov)| {
            fov.visible = field_of_view_set(pos, fov.radius, map);
            if entity == player {
                map.revealed.extend(&fov.visible);
            }
            fov.is_dirty = false;
        });
}
