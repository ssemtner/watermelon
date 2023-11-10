use bevy::prelude::*;

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Playing,
    Lost,
}
