use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    if map.can_enter(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(want_move.entity, fov.clone_dirty());
                if ecs.entry_ref(want_move.entity).unwrap().get_component::<Player>().is_ok() {
                    camera.on_play_move(want_move.destination);
                    fov.visible_tiles.iter().for_each(|pt| {
                        map.revealed[Map::to_index(pt.x, pt.y)] = true;
                    });
                }
            }
        }


        commands.remove(*entity);
    }
}