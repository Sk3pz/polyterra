use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::prelude::*;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use winit::window::Icon;
use crate::camera::{pan_orbit_camera, spawn_camera};
use crate::mouse::cursor_grab_system;
use crate::utils::setup_logger;

pub mod utils;
mod mouse;
pub mod camera;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // !! Camera setup was moved to the spawn_camera system
    // // set up the camera
    // let mut camera = OrthographicCameraBundle::new_3d();
    // camera.orthographic_projection.scale = 5.0;
    // camera.transform = Transform::from_xyz(15.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y);
    // // spawn the camera
    // commands.spawn_bundle(camera);

    // Spawn a light
    let mut sun = Light {
        color: Color::rgba(0.2, 0.2, 0.2, 1.0),
        depth: 0.1..50.0,
        fov: f32::to_radians(60.0),
        intensity: 1000.0,
        range: 20.0,
    };
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        light: sun,
        ..Default::default()
    });

    let mut altar_light = Light {
        color: Color::rgba(0.2, 0.8, 0.8, 1.0),
        depth: 0.1..50.0,
        fov: f32::to_radians(60.0),
        intensity: 50.0,
        range: 20.0,
    };
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        light: altar_light,
        ..Default::default()
    });

    // materials
    let mut ground_mat = StandardMaterial::from(Color::rgba(0.3, 0.5, 0.3, 1.0));
    ground_mat.reflectance = 0.1;
    ground_mat.roughness = 1.0;

    fn default_cube_mat() -> StandardMaterial {
        StandardMaterial {
            base_color: Color::rgba(0.8, 0.7, 0.6, 1.0),
            base_color_texture: None,
            roughness: 0.6,
            metallic: 0.01,
            metallic_roughness_texture: None,
            reflectance: 0.5,
            normal_map: None,
            double_sided: false,
            occlusion_texture: None,
            emissive: Color::BLACK,
            emissive_texture: None,
            unlit: false,
        }
    }

    let mut altar_mat = StandardMaterial {
        base_color: Color::rgba(0.4, 0.4, 0.3, 1.0),
        base_color_texture: None,
        roughness: 0.1,
        metallic: 0.01,
        metallic_roughness_texture: None,
        reflectance: 1.0,
        normal_map: None,
        double_sided: false,
        occlusion_texture: None,
        emissive: Color::rgba(0.2, 0.8, 0.8, 1.0),
        emissive_texture: None,
        unlit: false,
    };

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(ground_mat.into()),
        ..Default::default()
    });
    // cubes
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(default_cube_mat().into()),
        transform: Transform::from_xyz(1.5, 0.5, 1.5),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(default_cube_mat().into()),
        transform: Transform::from_xyz(1.5, 0.5, -1.5),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(default_cube_mat().into()),
        transform: Transform::from_xyz(-1.5, 0.5, 1.5),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(default_cube_mat().into()),
        transform: Transform::from_xyz(-1.5, 0.5, -1.5),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
        material: materials.add(altar_mat.into()),
        ..Default::default()
    });
}

fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}

fn set_window_icon(windows: Res<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("my_icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary.set_window_icon(Some(icon));
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(spawn_camera.system())
        //.add_system(cursor_grab_system.system())
        .add_system(pan_orbit_camera.system())
        .run();
}
