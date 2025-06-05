use crate::gamestate::AppState;
use crate::physics::collisions::collider_systems::detect_player_collisions;
use crate::physics::{bodies::RigidbodyComponent, prelude::Collider};
use bevy::{prelude::*, window::CursorGrabMode};

use super::{
    player_data::{Player, PlayerPositioning},
    player_info::{PlayerId, PlayerInfo, PlayerLevelInfo, PlayerUsername},
    player_stats::PlayerStats,
};

const JUMP_FORCE: f32 = 55.;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), setup_camera)
            .add_systems(
                Update,
                (
                    mouse_movement.run_if(in_state(crate::gamestate::AppState::Playing)),
                    lock_cursor.run_if(in_state(crate::gamestate::AppState::Playing)),
                    player_movement
                        .after(detect_player_collisions)
                        .run_if(in_state(crate::gamestate::AppState::Playing)),
                    apply_player_forces.run_if(in_state(crate::gamestate::AppState::Playing)),
                ),
            );
    }
}

pub fn update_fov(mut query: Query<&mut Projection>, mut query1: Query<&mut CameraSettings>) {
    if let Ok(mut projection) = query.single_mut() {
        if let Ok(settings) = query1.single_mut() {
            *projection = Projection::Perspective(PerspectiveProjection {
                fov: settings.fov.to_radians(),
                ..Default::default()
            });
        }
    }
}

fn apply_player_forces(mut query: Query<(&mut Player, &mut Transform)>, time: Res<Time>) {
    if let Ok((mut player, mut transform)) = query.single_mut() {
        let linear_damping = 0.085;
        player.pos.vel *= 1.0 - linear_damping;
        player.pos.vel.y += -9.18 * 25. * time.delta_secs();

        transform.translation.x += player.pos.vel.x * time.delta_secs();
        transform.translation.z += player.pos.vel.z * time.delta_secs();
        transform.translation.y += player.pos.vel.y * time.delta_secs();
    }
}

fn setup_camera(
    mut camq: Query<(&Camera2d, Entity)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok((_, entity)) = camq.single_mut() {
        commands.entity(entity).despawn();
    }

    let cam_set = CameraSettings {
        cursor_locked: CursorLocked(true),
        view_bobbing: true,
        fov: 100.,
    };

    commands.spawn(PointLight::default());
    let cam = commands
        .spawn((
            Camera3d::default(),
            WorldModelCamera,
            cam_set.clone(),
            Projection::from(PerspectiveProjection {
                fov: cam_set.fov.to_radians(),
                ..default()
            }),
            InheritedVisibility::default(),
        ))
        .id();

    // Find the player entity and attach the camera to it
    let player_id = commands.spawn(()).id();
    if let Ok(mut player_entity) = commands.get_entity(player_id) {
        player_entity.add_child(cam);
    }

    let cuboid = Cuboid::new(100., 1., 100.);
    commands.spawn((
        Mesh3d(meshes.add(cuboid)),
        MeshMaterial3d(materials.add(Color::WHITE)),
        RigidbodyComponent::new_static(Collider::from_cuboid(
            cuboid.half_size,
            Vec3::ZERO,
            Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
        )),
        Transform::from_xyz(0., -10., 0.),
    ));
}

pub fn player_movement(
    mut player_q: Query<(&mut Transform, &mut super::player_data::Player)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((transform, mut player)) = player_q.single_mut() {
        let mut direction = Vec3::ZERO;
        let forward: Vec3 = transform.forward().into();
        let right: Vec3 = transform.right().into();

        let forward_horizontal = Vec3::new(forward.x, 0.0, forward.z).normalize();
        let right_horizontal = Vec3::new(right.x, 0.0, right.z).normalize();

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += forward_horizontal;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= forward_horizontal;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= right_horizontal;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += right_horizontal;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        let speed = if keyboard_input.pressed(KeyCode::ShiftLeft) {
            player.stats.speed.speed * 1.5
        } else {
            player.stats.speed.speed
        };

        let horizontal_movement = direction * speed * time.delta_secs();

        player.pos.vel += speed * horizontal_movement; //.with_y(0.);
        //println!("{:?}", player.pos.grounded);
        if keyboard_input.just_pressed(KeyCode::Space) && player.pos.grounded {
            player.pos.vel.y = JUMP_FORCE;
            player.pos.grounded = false;
        }

        player.pos.set_loc(transform.translation);
    }
}

#[derive(Clone, Debug, Component)]
pub struct CameraSettings {
    pub cursor_locked: CursorLocked,
    pub view_bobbing: bool,
    pub fov: f32,
}

#[derive(Debug, Component)]
pub struct WorldModelCamera;

#[derive(Debug, Component, Deref, DerefMut, Clone)]
pub struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))
    }
}

#[derive(Clone, Debug)]
pub struct CursorLocked(pub bool);

pub fn mouse_movement(
    mut windows: Query<&mut Window>,
    mut query: Query<(
        (&mut Transform, &CameraSensitivity),
        &mut super::player_data::Player,
    )>,
    accumulated_mouse_motion: Res<bevy::input::mouse::AccumulatedMouseMotion>,
) {
    if let Ok(window) = windows.single_mut() {
        if window.cursor_options.grab_mode == CursorGrabMode::Locked {
            for ((mut transform, sens), mut player) in query.iter_mut() {
                let delta = accumulated_mouse_motion.delta;

                if delta != Vec2::ZERO {
                    let delta_yaw = -delta.x * sens.x;
                    let delta_pitch = -delta.y * sens.y;

                    let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
                    let yaw = yaw + delta_yaw;

                    const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.01;
                    let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

                    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
                    player.pos.set_dir(transform.rotation);
                    println!("{}", player.pos.dir);
                }
            }
        }
    }
}

pub fn lock_cursor(
    mut q: Query<&mut CameraSettings>,
    mut windows: Query<&mut Window>,
    mouse_input: Res<ButtonInput<KeyCode>>,
    click_input: Res<ButtonInput<MouseButton>>,
) {
    if let Ok(mut window) = windows.single_mut() {
        if let Ok(mut cam) = q.single_mut() {
            if mouse_input.just_pressed(KeyCode::Escape) || mouse_input.just_pressed(KeyCode::Tab) {
                cam.cursor_locked.0 = false;
            }
            if click_input.just_pressed(MouseButton::Left) {
                cam.cursor_locked.0 = true;
                window.cursor_options.visible = false;
                window.cursor_options.grab_mode = CursorGrabMode::Locked;
            }
            if !cam.cursor_locked.0 {
                window.cursor_options.visible = true;
                window.cursor_options.grab_mode = CursorGrabMode::None;
            } else {
                window.cursor_options.visible = false;
                window.cursor_options.grab_mode = CursorGrabMode::Locked;
            }
        }
    }
}
