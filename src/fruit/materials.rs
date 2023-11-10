use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::fruit::fruit_type::FruitType;

pub struct FruitMaterialsPlugin;

impl Plugin for FruitMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FruitMaterials(HashMap::new()))
            .add_systems(Startup, setup_materials);
    }
}

#[derive(Resource, Deref)]
pub struct FruitMaterials(pub(crate) HashMap<FruitType, Handle<ColorMaterial>>);

fn setup_materials(mut materials: ResMut<Assets<ColorMaterial>>, mut fruit_materials_resource: ResMut<FruitMaterials>) {
    let mut fruit_materials: HashMap<FruitType, Handle<ColorMaterial>> = HashMap::new();

    fruit_materials.insert(FruitType::Blueberry, materials.add(Color::hex("#3D4A84").unwrap().into()));
    fruit_materials.insert(FruitType::Strawberry, materials.add(Color::hex("#E70333").unwrap().into()));
    fruit_materials.insert(FruitType::Grapes, materials.add(Color::hex("#8E50DC").unwrap().into()));
    fruit_materials.insert(FruitType::Lemon, materials.add(Color::hex("#FFDA45").unwrap().into()));
    fruit_materials.insert(FruitType::Coconut, materials.add(Color::hex("#6D3F0C").unwrap().into()));
    fruit_materials.insert(FruitType::Apple, materials.add(Color::hex("#77BA00").unwrap().into()));
    fruit_materials.insert(FruitType::Orange, materials.add(Color::hex("#F96719").unwrap().into()));
    fruit_materials.insert(FruitType::Pomegranate, materials.add(Color::hex("#9F1E44").unwrap().into()));
    fruit_materials.insert(FruitType::Peach, materials.add(Color::hex("#FCB5A7").unwrap().into()));
    fruit_materials.insert(FruitType::Pineapple, materials.add(Color::hex("#F6DF0D").unwrap().into()));
    fruit_materials.insert(FruitType::Melon, materials.add(Color::hex("#8CB925").unwrap().into()));
    fruit_materials.insert(FruitType::Watermelon, materials.add(Color::hex("#6CCD15").unwrap().into()));

    fruit_materials_resource.0 = fruit_materials;
}