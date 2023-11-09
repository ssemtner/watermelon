use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use std::ops::DerefMut;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

const GRAVITY: f32 = -10.0;
const LEFT_WALL: f32 = -400.0;
const RIGHT_WALL: f32 = 400.0;
const BOTTOM_WALL: f32 = -300.0;
const TOP_WALL: f32 = 300.0;
const WALL_THICKNESS: f32 = 10.0;
const WALL_COLOR: Color = Color::rgb(0.0, 0.7, 0.7);

const FRUIT_SPAWN_HEIGHT: f32 = 200.0;

const DROP_COOLDOWN: f32 = 0.5;

const SCALE: f32 = 50.0;

#[derive(Component)]
struct Fruit(FruitType);

#[derive(Debug, PartialEq, Clone)]
enum FruitType {
    Blueberry,
    Strawberry,
}

#[derive(Resource)]
pub struct FruitMaterialHandles {
    pub blueberry: Handle<ColorMaterial>,
    pub strawberry: Handle<ColorMaterial>,
}

struct FruitMaterialsPlugin;

impl Plugin for FruitMaterialsPlugin {
    fn build(&self, app: &mut App) {
        let mut materials = app
            .world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .unwrap();

        // FRUIT COLORS DEFINED HERE
        let blueberry = materials.add(Color::rgb(0.0, 0.0, 0.7).into());
        let strawberry = materials.add(Color::rgb(0.7, 0.0, 0.0).into());

        app.insert_resource(FruitMaterialHandles {
            blueberry,
            strawberry,
        });
    }
}

impl FruitType {
    fn random() -> FruitType {
        match rand::random::<u8>() % 2 {
            0 => FruitType::Blueberry,
            1 => FruitType::Strawberry,
            _ => unreachable!(),
        }
    }

    fn size(&self) -> f32 {
        // FRUIT SIZES DEFINED HERE (in pixels)
        match self {
            FruitType::Blueberry => 30.0,
            FruitType::Strawberry => 50.0,
        }
    }

    fn material(&self, fruit_materials: &Res<FruitMaterialHandles>) -> Handle<ColorMaterial> {
        match self {
            FruitType::Blueberry => fruit_materials.blueberry.clone(),
            FruitType::Strawberry => fruit_materials.strawberry.clone(),
        }
    }

    fn merge(&self) -> FruitType {
        match self {
            FruitType::Blueberry => FruitType::Strawberry,
            FruitType::Strawberry => FruitType::Strawberry,
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

#[derive(Resource)]
struct DropTimer(Timer);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}

fn spawn_fruit(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fruit_materials: Res<FruitMaterialHandles>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut timer: ResMut<DropTimer>,
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
        let fruit_type = FruitType::random();

        let left_bound = LEFT_WALL + WALL_THICKNESS + fruit_type.size();
        let right_bound = RIGHT_WALL - WALL_THICKNESS - fruit_type.size();
        let x = position.x.clamp(left_bound, right_bound);

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
        ));

        timer.0.reset();
    }
}

fn tick_drop_timer(time: Res<Time>, mut timer: ResMut<DropTimer>) {
    timer.0.tick(time.delta());
}

fn handle_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    fruit_materials: Res<FruitMaterialHandles>,
    mut fruit_query: Query<(&mut Fruit, &mut Transform, &mut Handle<ColorMaterial>)>,
) {
    // might need to make this func faster or blocking or something

    for event in collision_events.iter() {
        if let CollisionEvent::Started(entity1, entity2, ..) = event {
            if let Some(fruit1) = fruit_query.get_component::<Fruit>(*entity1).ok() {
                if let Some(fruit2) = fruit_query.get_component::<Fruit>(*entity2).ok() {
                    if fruit1.0 == fruit2.0 {
                        commands.entity(*entity2).despawn();

                        let new_fruit_type = fruit1.0.merge();

                        // fruit1.0 = new_fruit_type.clone();

                        if let Some((mut fruit, mut transform, mut material)) = fruit_query.get_mut(*entity1).ok() {
                            fruit.0 = new_fruit_type.clone();
                            transform.scale = Vec3::splat(new_fruit_type.size());
                            *material = new_fruit_type.material(&fruit_materials);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FruitMaterialsPlugin)
        .insert_resource(DropTimer(Timer::from_seconds(
            DROP_COOLDOWN,
            TimerMode::Once,
        )))
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_collisions,).chain())
        .add_systems(
            Update,
            (
                tick_drop_timer,
                spawn_fruit.run_if(input_just_pressed(MouseButton::Left)),
            ),
        )
        .run();
}
