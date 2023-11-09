use bevy::ecs::query::QuerySingleError;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::petgraph::visit::Walker;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use rand::seq::SliceRandom;

const GRAVITY: f32 = -10.0;
const LEFT_WALL: f32 = -400.0;
const RIGHT_WALL: f32 = 400.0;
const BOTTOM_WALL: f32 = -300.0;
const TOP_WALL: f32 = 300.0;
const WALL_THICKNESS: f32 = 10.0;
const WALL_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

const FRUIT_SPAWN_HEIGHT: f32 = 200.0;
const LOSE_HEIGHT: f32 = 200.0;

const DROP_COOLDOWN: f32 = 0.5;

#[derive(Component)]
struct Fruit(FruitType);

#[derive(Debug, PartialEq, Clone)]
enum FruitType {
    Blueberry,
    Strawberry,
    Grapes,
    Lemon,
    Coconut,
    Apple,
    Orange,
    Pomegranate,
    Peach,
    Pineapple,
    Melon,
    Watermelon,
}

#[derive(Resource)]
pub struct FruitMaterialHandles {
    pub blueberry: Handle<ColorMaterial>,
    pub strawberry: Handle<ColorMaterial>,
    pub grapes: Handle<ColorMaterial>,
    pub lemon: Handle<ColorMaterial>,
    pub coconut: Handle<ColorMaterial>,
    pub apple: Handle<ColorMaterial>,
    pub orange: Handle<ColorMaterial>,
    pub pomegranate: Handle<ColorMaterial>,
    pub peach: Handle<ColorMaterial>,
    pub pineapple: Handle<ColorMaterial>,
    pub melon: Handle<ColorMaterial>,
    pub watermelon: Handle<ColorMaterial>,
}

struct FruitMaterialsPlugin;

impl Plugin for FruitMaterialsPlugin {
    fn build(&self, app: &mut App) {
        let mut materials = app
            .world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .unwrap();

        // FRUIT COLORS DEFINED HERE
        let blueberry = materials.add(Color::hex("#3D4A84").unwrap().into());
        let strawberry = materials.add(Color::hex("#E70333").unwrap().into());
        let grapes = materials.add(Color::hex("#8E50DC").unwrap().into());
        let lemon = materials.add(Color::hex("#FFDA45").unwrap().into());
        let coconut = materials.add(Color::hex("#6D3F0C").unwrap().into());
        let apple = materials.add(Color::hex("#77BA00").unwrap().into());
        let orange = materials.add(Color::hex("#F96719").unwrap().into());
        let pomegranate = materials.add(Color::hex("#9F1E44").unwrap().into());
        let peach = materials.add(Color::hex("#FCB5A7").unwrap().into());
        let pineapple = materials.add(Color::hex("#F6DF0D").unwrap().into());
        let melon = materials.add(Color::hex("#8CB925").unwrap().into());
        let watermelon = materials.add(Color::hex("#6CCD15").unwrap().into());

        app.insert_resource(FruitMaterialHandles {
            blueberry,
            strawberry,
            grapes,
            lemon,
            coconut,
            apple,
            orange,
            pomegranate,
            peach,
            pineapple,
            melon,
            watermelon,
        });
    }
}

impl FruitType {
    fn random() -> FruitType {
        match rand::random::<u8>() % 5 {
            0 => FruitType::Blueberry,
            1 => FruitType::Strawberry,
            2 => FruitType::Grapes,
            3 => FruitType::Lemon,
            4 => FruitType::Coconut,
            _ => unreachable!(),
        }
    }

    fn size(&self) -> f32 {
        // FRUIT SIZES DEFINED HERE (in pixels)
        match self {
            FruitType::Blueberry => 30.0,
            FruitType::Strawberry => 50.0,
            FruitType::Grapes => 70.0,
            FruitType::Lemon => 100.0,
            FruitType::Coconut => 120.0,
            FruitType::Apple => 150.0,
            FruitType::Orange => 180.0,
            FruitType::Pomegranate => 210.0,
            FruitType::Peach => 240.0,
            FruitType::Pineapple => 270.0,
            FruitType::Melon => 300.0,
            FruitType::Watermelon => 330.0,
        }
    }

    fn material(&self, fruit_materials: &Res<FruitMaterialHandles>) -> Handle<ColorMaterial> {
        match self {
            FruitType::Blueberry => fruit_materials.blueberry.clone(),
            FruitType::Strawberry => fruit_materials.strawberry.clone(),
            FruitType::Grapes => fruit_materials.grapes.clone(),
            FruitType::Lemon => fruit_materials.lemon.clone(),
            FruitType::Coconut => fruit_materials.coconut.clone(),
            FruitType::Apple => fruit_materials.apple.clone(),
            FruitType::Orange => fruit_materials.orange.clone(),
            FruitType::Pomegranate => fruit_materials.pomegranate.clone(),
            FruitType::Peach => fruit_materials.peach.clone(),
            FruitType::Pineapple => fruit_materials.pineapple.clone(),
            FruitType::Melon => fruit_materials.melon.clone(),
            FruitType::Watermelon => fruit_materials.watermelon.clone(),
        }
    }

    fn merge(&self) -> Option<FruitType> {
        match self {
            FruitType::Blueberry => Some(FruitType::Strawberry),
            FruitType::Strawberry => Some(FruitType::Grapes),
            FruitType::Grapes => Some(FruitType::Lemon),
            FruitType::Lemon => Some(FruitType::Coconut),
            FruitType::Coconut => Some(FruitType::Apple),
            FruitType::Apple => Some(FruitType::Orange),
            FruitType::Orange => Some(FruitType::Pomegranate),
            FruitType::Pomegranate => Some(FruitType::Peach),
            FruitType::Peach => Some(FruitType::Pineapple),
            FruitType::Pineapple => Some(FruitType::Melon),
            FruitType::Melon => Some(FruitType::Watermelon),
            FruitType::Watermelon => None,
        }
    }
}

#[derive(Component)]
struct Level();

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.0),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.0),
            WallLocation::Bottom => Vec2::new(0.0, BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0.0, TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;

        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => Vec2::new(WALL_THICKNESS, arena_height),
            WallLocation::Bottom | WallLocation::Top => Vec2::new(arena_width, WALL_THICKNESS),
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider::cuboid(0.5, 0.5),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum State {
    #[default]
    Playing,
    Lost,
}

#[derive(Resource)]
struct DropTimer(Timer);

#[derive(Resource)]
struct MergeSound(Vec<Handle<AudioSource>>);

#[derive(Resource)]
struct SpawnSound(Handle<AudioSource>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let merge_sound_0 = asset_server.load("sounds/pop0.ogg");
    let merge_sound_1 = asset_server.load("sounds/pop1.ogg");
    let merge_sound_2 = asset_server.load("sounds/pop2.ogg");
    let merge_sound_3 = asset_server.load("sounds/pop3.ogg");
    let merge_sound_4 = asset_server.load("sounds/pop4.ogg");
    let merge_sound_5 = asset_server.load("sounds/pop5.ogg");
    let merge_sound_6 = asset_server.load("sounds/pop6.ogg");
    let merge_sound_7 = asset_server.load("sounds/pop7.ogg");
    let merge_sound_8 = asset_server.load("sounds/pop8.ogg");
    let merge_sound_9 = asset_server.load("sounds/pop9.ogg");
    let merge_sound_10 = asset_server.load("sounds/pop10.ogg");
    commands.insert_resource(MergeSound(vec![
        merge_sound_0,
        merge_sound_1,
        merge_sound_2,
        merge_sound_3,
        merge_sound_4,
        merge_sound_5,
        merge_sound_6,
        merge_sound_7,
        merge_sound_8,
        merge_sound_9,
        merge_sound_10,
    ]));

    let spawn_sound = asset_server.load("sounds/click.ogg");
    commands.insert_resource(SpawnSound(spawn_sound));

    commands.spawn(Camera2dBundle::default());

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));

    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, LOSE_HEIGHT, 0.0),
            scale: Vec3::new(RIGHT_WALL - LEFT_WALL, 5.0, 1.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(0.5, 0.5, 0.5),
            ..default()
        },
        ..default()
    });
}

fn clamp_to_bounds(n: f32, fruit_type: &FruitType) -> f32 {
    let left_bound = LEFT_WALL + WALL_THICKNESS + (fruit_type.size() * 0.5) + 10.0;
    let right_bound = RIGHT_WALL - WALL_THICKNESS - (fruit_type.size() * 0.5) - 10.0;
    n.clamp(left_bound, right_bound)
}

#[derive(Resource)]
struct CurrentFruitType(FruitType);

#[derive(Component)]
struct FruitPreview;

fn drop_preview(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    fruit_materials: Res<FruitMaterialHandles>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    current_fruit_type: Res<CurrentFruitType>,
    mut fruit_preview_query: Query<&mut Transform, With<FruitPreview>>,
) {
    let (camera, camera_transform) = camera_query.single();

    let window = window_query.single();

    if let Some(position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let x = clamp_to_bounds(position.x, &current_fruit_type.0);

        match fruit_preview_query.get_single_mut() {
            Ok(mut transform) => {
                transform.translation.x = x;
            }
            Err(QuerySingleError::NoEntities(_)) => {
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::default().into()).into(),
                        material: current_fruit_type.0.material(&fruit_materials),
                        transform: Transform::from_translation(Vec3::new(
                            x,
                            FRUIT_SPAWN_HEIGHT,
                            2.0,
                        ))
                        .with_scale(Vec3::splat(current_fruit_type.0.size())),
                        ..default()
                    },
                    Fruit(current_fruit_type.0.clone()),
                    FruitPreview,
                ));
            }
            Err(QuerySingleError::MultipleEntities(_)) => panic!("Multiple fruit previews found"),
        }
    }
}

fn drop_fruit(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    fruit_materials: Res<FruitMaterialHandles>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut timer: ResMut<DropTimer>,
    mut current_fruit_type: ResMut<CurrentFruitType>,
    fruit_preview_query: Query<Entity, With<FruitPreview>>,
    sound: Res<SpawnSound>,
) {
    if !timer.0.finished() {
        return;
    }

    let (camera, camera_transform) = camera_query.single();

    let window = window_query.single();

    if let Some(position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let fruit_type = current_fruit_type.0.clone();

        commands.entity(fruit_preview_query.single()).despawn();

        current_fruit_type.0 = FruitType::random();

        let x = clamp_to_bounds(position.x, &fruit_type);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::default().into()).into(),
                material: fruit_type.material(&fruit_materials),
                transform: Transform::from_translation(Vec3::new(x, FRUIT_SPAWN_HEIGHT, 0.0))
                    .with_scale(Vec3::splat(fruit_type.size())),
                ..default()
            },
            Fruit(fruit_type),
            Collider::ball(0.5),
            RigidBody::Dynamic,
            GravityScale(100.0),
            Velocity::linear(Vec2::new(0.0, GRAVITY)),
            Damping {
                linear_damping: 5.0,
                angular_damping: 1000.0,
            },
            ActiveEvents::COLLISION_EVENTS,
            ExternalImpulse {
                impulse: Vec2::new(0.0, -100.0),
                ..default()
            },
            ColliderMassProperties::Density(10.0)
        ));

        timer.0.reset();

        commands.spawn(AudioBundle {
            source: sound.0.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

fn tick_drop_timer(time: Res<Time>, mut timer: ResMut<DropTimer>) {
    timer.0.tick(time.delta());
}

// fn show_drop_preview()

#[derive(Component)]
struct HasCollided;

fn handle_collisions(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    fruit_materials: Res<FruitMaterialHandles>,
    fruit_entity_query: Query<Entity, With<Fruit>>,
    mut fruit_query: Query<(&mut Fruit, &mut Transform, &mut Handle<ColorMaterial>)>,
    has_collided_query: Query<&HasCollided>,
    sound: Res<MergeSound>,
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
                            *material = new_fruit_type.material(&fruit_materials);
                        }

                        commands.spawn(AudioBundle {
                            source: sound.0.choose(&mut rand::thread_rng()).unwrap().clone(),
                            settings: PlaybackSettings::DESPAWN,
                        });
                    }
                }
            }
        }
    }
}

fn check_loss(
    mut commands: Commands,
    fruit_query: Query<&Transform, With<HasCollided>>,
    mut next_state: ResMut<NextState<State>>,
) {
    for transform in fruit_query.iter() {
        if transform.translation.y > LOSE_HEIGHT {
            next_state.set(State::Lost);

            commands.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "You Lost :(".to_string(),
                            style: TextStyle {
                                font_size: 64.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        },
                        TextSection {
                            value: "Click to restart".to_string(),
                            style: TextStyle {
                                font_size: 32.0,
                                color: Color::rgb(0.2, 0.9, 0.2),
                                ..default()
                            },
                        },
                    ],
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..default()
                },
                ..default()
            });
        }
    }
}

fn restart_game(
    mut commands: Commands,
    fruit_query: Query<Entity, With<Fruit>>,
    text_query: Query<Entity, With<Text>>,
    mut next_state: ResMut<NextState<State>>,
) {
    for entity in fruit_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in text_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.insert_resource(CurrentFruitType(FruitType::random()));

    next_state.set(State::Playing);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FruitMaterialsPlugin)
        .add_state::<State>()
        .insert_resource(DropTimer(Timer::from_seconds(
            DROP_COOLDOWN,
            TimerMode::Once,
        )))
        .insert_resource(CurrentFruitType(FruitType::random()))
        .add_systems(Startup, setup)
        // .add_systems(FixedUpdate, handle_collisions)
        .add_systems(
            Update,
            (
                check_loss,
                tick_drop_timer,
                drop_preview,
                drop_fruit.run_if(input_just_pressed(MouseButton::Left)),
            )
                .chain()
                .run_if(in_state(State::Playing)),
        )
        .add_systems(
            Update,
            restart_game
                .run_if(input_just_pressed(MouseButton::Left))
                .run_if(in_state(State::Lost)),
        )
        .run();
}
