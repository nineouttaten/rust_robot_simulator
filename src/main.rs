use bevy::{prelude::*, window::PresentMode::AutoNoVsync};
use bevy_flycam::PlayerPlugin;
use bevy_obj::*;

static mut first_point: [f32; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            present_mode: AutoNoVsync,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_system(rotate_elbow)
        .add_system(rotate_lower_arm)
        .add_system(rotate_shoulder)
        .add_system(rotate_upper_arm)
        .add_system(rotate_wrist)
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugin(ObjPlugin)
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::new(
            Quat::from_rotation_y(-0.2),
        ))
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
        .insert(bevy_transform_gizmo::GizmoPickSource::default());

    let material = materials.add(Color::rgb(0.8, 0.7, 0.6).into());
    commands.spawn_bundle(PbrBundle {
        mesh: asset_server.load("models/Gleb_Robot/base.obj"),
        material: material.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    }).insert_bundle(bevy_mod_picking::PickableBundle::default())
            .insert(bevy_transform_gizmo::GizmoTransformable)
    .with_children(|parent| {
        parent
            .spawn_bundle(PbrBundle {
                mesh: asset_server.load("models/Gleb_Robot/shoulder.obj"),
                material: material.clone(),
                transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                    Vec3::new(1., 1., 1.),
                    Quat::from_rotation_x(0.),
                    Vec3::new(0.0, 0., 0.),
                )),
                ..Default::default()
            }).insert(Rotatable_shoulder{speed:1.})

            .with_children(|parent| {
                parent
                    .spawn_bundle(PbrBundle {
                        mesh: asset_server.load("models/Gleb_Robot/lower_arm.obj"),
                        material: material.clone(),
                        transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                            Vec3::new(1., 1., 1.),
                            Quat::from_rotation_y(0.),
                            Vec3::new(0., 0.8, 0.25),
                        )),
                        ..Default::default()
                    }).insert(Rotatable_lower_arm{speed:1.})

                    .with_children(|parent| {
                        parent
                            .spawn_bundle(PbrBundle {
                                mesh: asset_server.load("models/Gleb_Robot/elbow.obj"),
                                material: material.clone(),
                                transform: Transform::from_matrix(
                                    Mat4::from_scale_rotation_translation(
                                        Vec3::new(1., 1., 1.),
                                        Quat::from_rotation_y(0.),
                                        Vec3::new(0.0, 0.7379941, -1.5010117),
                                    ),
                                ),
                                ..Default::default()
                            }).insert(Rotatable_elbow{speed:1.})

                            .with_children(|parent| {
                                parent
                                    .spawn_bundle(PbrBundle {
                                        mesh: asset_server.load("models/Gleb_Robot/upper_arm.obj"),
                                        material: material.clone(),
                                        transform: Transform::from_matrix(
                                            Mat4::from_scale_rotation_translation(
                                                Vec3::new(1., 1., 1.),
                                                Quat::from_rotation_x(0.),
                                                Vec3::new(0.0, 0.9099869, 1.7750295),
                                            ),
                                        ),
                                        ..Default::default()
                                    }).insert(Rotatable_upper_arm{speed:1.})

                                    .with_children(|parent| {
                                        parent
                                            .spawn_bundle(PbrBundle {
                                                mesh: asset_server.load("models/Gleb_Robot/wrist.obj"),
                                                material: material.clone(),
                                                transform: Transform::from_matrix(
                                                    Mat4::from_scale_rotation_translation(
                                                        Vec3::new(1., 1., 1.),
                                                        Quat::from_rotation_x(0.25844246),
                                                        Vec3::new(0.0, -0.09199781, 0.33600545),
                                                    ),
                                                ),
                                                ..Default::default()
                                            }).insert(Rotatable_wrist{speed:1.});
                                    });
                            });
                    });
            });
    });

}

#[derive(Component)]
struct Rotatable_elbow {
    speed: f32,
}

fn rotate_elbow(mut objects: Query<(&mut Transform, &Rotatable_elbow)>, timer: Res<Time>,keyboard_input: Res<Input<KeyCode>>) {
    let mut can_rotate = false;
    let mut modifier;
    for (mut transform, object) in &mut objects {
        if keyboard_input.pressed(KeyCode::Q) {
            can_rotate = !can_rotate;
        }

        if keyboard_input.pressed(KeyCode::Z) {
            unsafe {
                first_point[0] = transform.rotation.x;
            }
        }
        if keyboard_input.pressed(KeyCode::C) {
            unsafe {
                if transform.rotation.x < first_point[0] {
                    modifier = (first_point[0] - transform.rotation.x) * 5.0;
                    transform.rotate_x(object.speed * modifier * timer.delta_seconds());
                }
                else {
                    modifier = (transform.rotation.x - first_point[0]) * 5.0;
                    transform.rotate_x(-object.speed * modifier * timer.delta_seconds());
                }
            }
        }

        if can_rotate {
            if transform.rotation.x < 0.1 {
                if keyboard_input.pressed(KeyCode::Left) {
                    transform.rotate_x(object.speed * timer.delta_seconds());
                }
            }
            if transform.rotation.x > -0.8 {
                if keyboard_input.pressed(KeyCode::Right) {
                    transform.rotate_x(-object.speed * timer.delta_seconds());
                }
            }
        }
    }
}

#[derive(Component)]
struct Rotatable_lower_arm {
    speed: f32,
}

fn rotate_lower_arm(mut objects: Query<(&mut Transform, &Rotatable_lower_arm)>, timer: Res<Time>,keyboard_input: Res<Input<KeyCode>>) {
    let mut can_rotate = false;
    let mut modifier;
    for (mut transform, object) in &mut objects {
        if keyboard_input.pressed(KeyCode::W) {
            can_rotate = !can_rotate;
        }

        if keyboard_input.pressed(KeyCode::Z) {
            unsafe {
                first_point[1] = transform.rotation.x;
            }
        }
        if keyboard_input.pressed(KeyCode::C) {
            unsafe {
                if transform.rotation.x < first_point[1] {
                    modifier = (first_point[1] - transform.rotation.x) * 5.0;
                    transform.rotate_x(object.speed * modifier * timer.delta_seconds());
                }
                else {
                    modifier = (transform.rotation.x - first_point[1]) * 5.0;
                    transform.rotate_x(-object.speed * modifier * timer.delta_seconds());
                }
            }
        }

        if can_rotate {
            if transform.rotation.x < 0.7 {
                if keyboard_input.pressed(KeyCode::Left) {
                    transform.rotate_x(object.speed * timer.delta_seconds());
                }
            }
            if transform.rotation.x > -0.3 {
                if keyboard_input.pressed(KeyCode::Right) {
                    transform.rotate_x(-object.speed * timer.delta_seconds());
                }
            }
        }
    }
}

#[derive(Component)]
struct Rotatable_shoulder {
    speed: f32,
}

fn rotate_shoulder(mut objects: Query<(&mut Transform, &Rotatable_shoulder)>, timer: Res<Time>,keyboard_input: Res<Input<KeyCode>>) {
    let mut can_rotate = false;
    let mut modifier;
    for (mut transform, object) in &mut objects {
        if keyboard_input.pressed(KeyCode::E) {
            can_rotate = !can_rotate;
        }

        if keyboard_input.pressed(KeyCode::Z) {
            unsafe {
                first_point[2] = transform.rotation.y;
            }
        }
        if keyboard_input.pressed(KeyCode::C) {
            unsafe {
                if transform.rotation.y < first_point[2] {
                    modifier = (first_point[2] - transform.rotation.y) * 5.0;
                    transform.rotate_y(object.speed * modifier * timer.delta_seconds());
                }
                else {
                    modifier = (transform.rotation.y - first_point[2]) * 5.0;
                    transform.rotate_y(-object.speed * modifier * timer.delta_seconds());
                }
            }
        }

        if can_rotate {
            if keyboard_input.pressed(KeyCode::Left) {
                transform.rotate_y(object.speed * timer.delta_seconds());
            }
            if keyboard_input.pressed(KeyCode::Right) {
                transform.rotate_y(-object.speed * timer.delta_seconds());
            }
        }
    }
}

#[derive(Component)]
struct Rotatable_upper_arm {
    speed: f32,
}

fn rotate_upper_arm(mut objects: Query<(&mut Transform, &Rotatable_upper_arm)>, timer: Res<Time>,keyboard_input: Res<Input<KeyCode>>) {
    let mut can_rotate = false;
    let mut modifier;
    for (mut transform, object) in &mut objects {
        if keyboard_input.pressed(KeyCode::R) {
            can_rotate = !can_rotate;
        }

        if keyboard_input.pressed(KeyCode::Z) {
            unsafe {
                first_point[3] = transform.rotation.x;
            }
        }
        if keyboard_input.pressed(KeyCode::C) {
            unsafe {
                if transform.rotation.x < first_point[3] {
                    modifier = (first_point[3] - transform.rotation.x) * 5.0;
                    transform.rotate_x(object.speed * modifier * timer.delta_seconds());
                }
                else {
                    modifier = (transform.rotation.x - first_point[3]) * 5.0;
                    transform.rotate_x(-object.speed * modifier * timer.delta_seconds());
                }
            }
        }

        if can_rotate {
            if transform.rotation.x < 1. {
                if keyboard_input.pressed(KeyCode::Left) {
                    transform.rotate_x(object.speed * timer.delta_seconds());
                }
            }
            if transform.rotation.x > -1. {
                if keyboard_input.pressed(KeyCode::Right) {
                    transform.rotate_x(-object.speed * timer.delta_seconds());
                }
            }
        }
    }
}

#[derive(Component)]
struct Rotatable_wrist {
    speed: f32,
}

fn rotate_wrist(mut objects: Query<(&mut Transform, &Rotatable_wrist)>, timer: Res<Time>,keyboard_input: Res<Input<KeyCode>>) {
    let mut can_rotate = false;
    let mut modifier;
    for (mut transform, object) in &mut objects {
        if keyboard_input.pressed(KeyCode::T) {
            can_rotate = !can_rotate;
        }

        if keyboard_input.pressed(KeyCode::Z) {
            unsafe {
                first_point[4] = transform.rotation.z;
            }
        }
        if keyboard_input.pressed(KeyCode::C) {
            unsafe {
                if transform.rotation.z < first_point[4] {
                    modifier = (first_point[4] - transform.rotation.z) * 5.0;
                    transform.rotate_z(object.speed * modifier * timer.delta_seconds());
                }
                else {
                    modifier = (transform.rotation.z - first_point[4]) * 5.0;
                    transform.rotate_z(-object.speed * modifier * timer.delta_seconds());
                }
            }
        }

        if can_rotate {
            if transform.rotation.z < 1. {
                if keyboard_input.pressed(KeyCode::Left) {
                    transform.rotate_z(object.speed * timer.delta_seconds());
                }
            }
            if transform.rotation.z > -1. {
                if keyboard_input.pressed(KeyCode::Right) {
                    transform.rotate_z(-object.speed * timer.delta_seconds());
                }
            }
        }
    }
}

