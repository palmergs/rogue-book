use crate::prelude::*;

#[system]
pub fn map_render(
    #[resource] map: &Map, 
    #[resource] camera: &Camera
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y .. camera.bottom_y {
        for x in camera.left_x .. camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(pt) {
                let idx = Map::to_index(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };

                draw_batch.set(
                    pt - offset, 
                    ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)),
                    glyph);
            }
        }
    }
    draw_batch.submit(0).expect("draw batch error in map_render");
}