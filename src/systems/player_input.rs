use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Point)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if turn_state.clone() != TurnState::AwaitingInput { 
        return
    }

    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };

        if delta.x != 0 || delta.y != 0 {
            let mut players = <(Entity, &mut Point)>::query().filter(component::<Player>());
            players.iter_mut(ecs).for_each(|(entity, pos)| {
                let dest = *pos + delta;
                commands.push(((), WantsToMove{ entity: *entity, destination: dest }));
            });
        }

        *turn_state = TurnState::PlayerTurn;
    }
}