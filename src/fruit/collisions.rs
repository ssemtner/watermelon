use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::fruit::fruit_type::Fruit;
use crate::fruit::materials::FruitMaterials;

#[derive(Component)]
pub struct HasCollided;

pub fn handle_collisions(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    fruit_materials: Res<FruitMaterials>,
    fruit_entity_query: Query<Entity, With<Fruit>>,
    mut fruit_query: Query<(&mut Fruit, &mut Transform, &mut Handle<ColorMaterial>)>,
    has_collided_query: Query<&HasCollided>,
    // sound: Res<MergeSound>,
) {
    // might need to make this func faster or blocking or something
    for entity1 in fruit_entity_query.iter() {
        for entity2 in fruit_entity_query.iter() {
            let contact_pair = rapier_context.contact_pair(entity1, entity2);
            if contact_pair.is_none() {
                continue;
            }

            if !contact_pair.unwrap().has_any_active_contacts() {
                continue;
            }
            println!("{:?} contacting {:?}", entity1, entity2);

            if let Some(fruit1) = fruit_query.get_component::<Fruit>(entity1).ok() {
                if let Some(fruit2) = fruit_query.get_component::<Fruit>(entity2).ok() {
                    if has_collided_query.get(entity1).is_err() {
                        commands.entity(entity1).try_insert(HasCollided);
                    }

                    if has_collided_query.get(entity2).is_err() {
                        commands.entity(entity2).try_insert(HasCollided);
                    }

                    if fruit1.0 == fruit2.0 {
                        let new_fruit_type = fruit1.0.merge();

                        if new_fruit_type.is_none() {
                            continue;
                        }

                        let new_fruit_type = new_fruit_type.unwrap();

                        commands.entity(entity2).despawn();

                        if let Some((mut fruit, mut transform, mut material)) =
                            fruit_query.get_mut(entity1).ok()
                        {
                            fruit.0 = new_fruit_type.clone();
                            transform.scale = Vec3::splat(new_fruit_type.size());
                            *material = fruit_materials.0[&new_fruit_type].clone();
                        }

                        // commands.spawn(AudioBundle {
                        //     source: sound.0.choose(&mut rand::thread_rng()).unwrap().clone(),
                        //     settings: PlaybackSettings::DESPAWN,
                        // });
                    }
                }
            }
        }
    }
}