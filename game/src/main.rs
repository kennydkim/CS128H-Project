use bevy::prelude::*;
use bevy_rapier2d::prelude::*; 
mod camera; 
pub use camera::*; 

const OCEAN_BLUE: Color = Color::rgb(0.0, 0.4118, 0.5804);
// const SKY_BLUE: Color = Color::rgb(0.6431, 0.8588, 0.9098);
const SAND: Color = Color::rgb(0.761, 0.698, 0.502);

// for live updating run use: cargo watch -cx "run --release"
// for the web server use: cargo run --release --target wasm32-unknown-unknown


pub fn main() {
    App::new()
        .add_startup_stage("player_setup", SystemStage::single(spawn_player))
        .add_startup_system(setup)
        .add_startup_stage("floor_setup", SystemStage::single(spawn_floor))
        .add_system(player_jumps)
        .add_system(jump_reset)
        .insert_resource(WindowDescriptor {
            title: "Rustacean!".to_string(), 
            ..Default::default()
        })
        .insert_resource(ClearColor(OCEAN_BLUE))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(DefaultPlugins)
        .run();
}

#[derive(Component)]
struct Player; 

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(new_camera_2d()); 
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let rigid_body = RigidBodyBundle {
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        activation: RigidBodyActivation::cannot_sleep().into(),
        ccd: RigidBodyCcd { ccd_enabled: true, ..Default::default() }.into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(0.65, 0.65).into(),
        flags: ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        }.into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7), 
                custom_size: Some(Vec2::new(1.5, 1.5)),
                ..Default::default()
            }, 
            texture: asset_server.load("krabby.png"),
            ..Default::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Player)
        .insert(Jumper { jump_impulse: 5. , is_jumping: false });
}

pub fn spawn_floor(mut commands: Commands) {
    let width = 10.; 
    let height = 1.5;
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(0.0, -3.).into(), 
        body_type: RigidBodyType::Static.into(), 
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(width / 2., height / 2.).into(), 
        ..Default::default()
    };
    commands    
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SAND, 
                custom_size: Some(Vec2::new(width, height)), 
                ..Default::default()
            }, 
            ..Default::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete);
}

#[derive(Component)] 
struct Jumper {
    jump_impulse: f32, 
    is_jumping: bool 
}

fn player_jumps (
    keyboard_input: Res<Input<KeyCode>>, 
    mut players: Query<(&mut Jumper, &mut RigidBodyVelocityComponent), With<Player>>
) {
    for (mut jumper, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) && !jumper.is_jumping {
            velocity.linvel = Vec2::new(0., jumper.jump_impulse).into();
            jumper.is_jumping = true; 
        }
    }
}

fn jump_reset( 
    mut query: Query<(Entity, &mut Jumper)>, 
    mut contact_events: EventReader<ContactEvent>, 
) {
    for contact_event in contact_events.iter() {
        for (entity, mut jumper) in query.iter_mut() {
            set_jumping_false_if_touching_floor(entity, &mut jumper, contact_event); 
        }
    }
}

fn set_jumping_false_if_touching_floor(entity: Entity, jumper: &mut Jumper, event: &ContactEvent) {
    if let ContactEvent::Started(h1, h2) = event {
        if h1.entity() == entity || h2.entity() == entity {
            jumper.is_jumping = false;
        }
    }
}