use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(delta) = key.map(|key| match key {
        VirtualKeyCode::Left => Some(Point::new(-1, 0)),
        VirtualKeyCode::Right => Some(Point::new(1, 0)),
        VirtualKeyCode::Up => Some(Point::new(0, -1)),
        VirtualKeyCode::Down => Some(Point::new(0, 1)),
        _ => None,})
            .flatten() {
        let mut players = <&mut Point>::query()
            .filter(component::<Player>());
        players.iter_mut(ecs).for_each(|pos| {
            let destination = *pos + delta;
            if map.can_enter_tile(destination) {
                *pos = destination;
                camera.on_player_move(destination);
            }
        });
    }
}