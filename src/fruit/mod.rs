pub mod fruit_type;
pub mod materials;
pub mod collisions;

use bevy::prelude::*;

pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(materials::FruitMaterialsPlugin)
            .add_systems(FixedUpdate, collisions::handle_collisions);
    }
}
