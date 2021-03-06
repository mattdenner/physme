use bevy::prelude::*;
use physme::prelude2d::*;

#[derive(Default)]
pub struct CharacterController;

fn main() {
    let mut builder = App::build();
    builder
        .add_default_plugins()
        .add_plugin(Physics2dPlugin)
        .add_resource(GlobalFriction(0.90))
        .add_startup_system(setup.system());
    let character_system = CharacterControllerSystem::default().system(builder.resources_mut());
    builder.add_system(character_system);
    builder.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let icon = asset_server.load("icon.png");
    let square = asset_server.load("square.png");
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(icon.into()),
            ..Default::default()
        })
        .with(
            RigidBody::new(Mass::Real(1.0))
                .with_status(Status::Semikinematic)
                .with_position(Vec2::new(0.0, 0.0))
                .with_terminal(Vec2::new(500.0, 5000.0)),
        )
        .with(CharacterController::default())
        .with_children(|parent| {
            parent.spawn((Shape::from(Size2::new(28.0, 28.0)),));
        })
        .spawn(SpriteComponents {
            material: materials.add(square.into()),
            ..Default::default()
        })
        .with(
            RigidBody::new(Mass::Real(1.0))
                .with_status(Status::Semikinematic)
                .with_position(Vec2::new(0.0, 60.0)),
        )
        .with_children(|parent| {
            parent.spawn((Shape::from(Size2::new(20.0, 20.0)),));
        });
}

#[derive(Default)]
pub struct CharacterControllerSystem;

impl CharacterControllerSystem {
    pub fn system(self, res: &mut Resources) -> Box<dyn System> {
        let system = character_system.system();
        res.insert_local(system.id(), self);
        system
    }
}

fn character_system(
    _state: Local<CharacterControllerSystem>,
    input: Res<Input<KeyCode>>,
    mut query: Query<With<CharacterController, Mut<RigidBody>>>,
) {
    for mut body in &mut query.iter() {
        if input.pressed(KeyCode::Q) {
            body.rotation += 0.1;
        }
        if input.pressed(KeyCode::E) {
            body.rotation -= 0.1;
        }
        if input.pressed(KeyCode::W) {
            body.apply_linear_impulse(Vec2::new(0.0, 5.0));
        }
        if input.pressed(KeyCode::S) {
            body.apply_linear_impulse(Vec2::new(0.0, -5.0));
        }
        if input.pressed(KeyCode::A) {
            body.apply_linear_impulse(Vec2::new(-5.0, 0.0));
        }
        if input.pressed(KeyCode::D) {
            body.apply_linear_impulse(Vec2::new(5.0, 0.0));
        }
    }
}
