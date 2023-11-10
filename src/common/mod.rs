mod app_state;
mod sound;

use bevy::prelude::*;
use crate::common::app_state::AppState;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>();
    }
}


