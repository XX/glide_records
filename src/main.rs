use bevy::{
    app::App,
    asset::Assets,
    ecs::system::{Commands, IntoSystem, ResMut},
    input::system::exit_on_esc_system,
    pbr::{LightBundle, PbrBundle},
    prelude::StandardMaterial,
    render::{
        color::Color,
        mesh::{shape, Mesh},
        render_graph::base::Msaa,
    },
    transform::components::Transform,
    DefaultPlugins,
};

use crate::{camera::PanOrbitCameraPlugin, diagnostic::DiagnosticPlugin, hello::HelloPlugin};

mod camera;
mod diagnostic;
mod hello;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DiagnosticPlugin)
        .add_plugin(HelloPlugin)
        .add_plugin(PanOrbitCameraPlugin)
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
