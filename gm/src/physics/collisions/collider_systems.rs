use bevy::{ecs::query, prelude::*};

use crate::{physics::bodies::*, player::player_data::Player};

use super::{Collider, ColliderVertexInfo, ContactInfo};

pub(crate) fn update_vertices(
    mut query: Query<(&mut RigidbodyComponent, &mut Transform), With<RigidbodyComponent>>,
) {
    for (mut body, _) in query.iter_mut() {
        //let rot: [f32; 3] = transform.rotation.to_euler(EulerRot::XYZ).into();
        body.collider.axes = [
            body.collider.rotation * Vec3::X,
            body.collider.rotation * Vec3::Y,
            body.collider.rotation * Vec3::Z,
        ];

        body.collider.vertex_info = ColliderVertexInfo::from_cuboid(
            &body.collider.center,
            &body.collider.half_extents,
            &body.collider.rotation,
        );
    }
}

fn rotation_to_align(from: Vec3, to: Vec3) -> Quat {
    let from = from.normalize();
    let to = to.normalize();
    let axis = from.cross(to);
    let angle = from.angle_between(to);
    if axis.length_squared() < 1e-6 {
        if from.dot(to) < 0.0 {
            // Rotate 180° around any perpendicular axis
            Quat::from_axis_angle(Vec3::X, std::f32::consts::PI)
        } else {
            Quat::IDENTITY
        }
    } else {
        Quat::from_axis_angle(axis.normalize(), angle)
    }
}

type RigidbodyQuery<'a, 'w> = Query<'w, 'a, (&'a mut RigidbodyComponent, &'a mut Transform)>;
type PlayerQuery<'a> = Query<'a, 'a, (&'a mut Player, &'a mut Transform)>;

pub fn detect_player_collisions(mut paramset: ParamSet<(RigidbodyQuery, PlayerQuery)>) {
    let bodies: Vec<(RigidbodyComponent, Transform)> = {
        let mut binding = paramset.p0();
        binding
            .iter_mut()
            .map(|(rb, tf)| (rb.clone(), *tf))
            .collect()
    };
    if let Ok((mut player, mut transform)) = paramset.p1().single_mut() {
        for (body, _) in bodies {
            if body.collider.axes == [Vec3::X, Vec3::Y, Vec3::Z] {
                if let Some(contact) = aabb_player_vs_collider(
                    transform.translation,
                    Vec3::new(1., 5., 1.),
                    player.pos.vel,
                    &body.collider,
                    &body.velocity,
                ) {
                    // println!(
                    //     "collision -- normal: {:?}, penetration: {}",
                    //     contact.normal, contact.penetration_depth
                    // );
                    resolve_player(&mut player, &mut transform, body.clone(), contact);
                    //println!("{:?}", player.pos.grounded);
                }
            } else {
                // use sat
            }
        }
    }
}

fn resolve_player(
    player: &mut Player,
    trans: &mut Transform,
    mut body: RigidbodyComponent,
    info: ContactInfo,
) {
    if info.normal.dot(Vec3::Y) > 0.7 {
        player.pos.grounded = true;
    }

    if body.rbt == RigidbodyType::Static {
        trans.translation += info.normal * info.penetration_depth;

        let into_surface = player.pos.vel.dot(info.normal);
        if into_surface < 0.0 {
            player.pos.vel -= info.normal * into_surface;
        }
    }
    if body.rbt == RigidbodyType::Dynamic {
        trans.translation += info.normal * info.penetration_depth;
        let into_surface = player.pos.vel.dot(info.normal);
        if into_surface < 0.0 {
            player.pos.vel -= info.normal * into_surface;
        }

        body.velocity.linear += player.pos.vel;
    }
}

pub fn aabb_player_vs_collider(
    player_position: Vec3,
    player_half_size: Vec3,
    player_velocity: Vec3,
    other: &Collider,
    other_velocity: &Velocity,
) -> Option<ContactInfo> {
    let min_a = player_position - player_half_size;
    let max_a = player_position + player_half_size;

    let min_b = other.center - other.half_extents;
    let max_b = other.center + other.half_extents;

    if (min_a.x > max_b.x || max_a.x < min_b.x)
        || (min_a.y > max_b.y || max_a.y < min_b.y)
        || (min_a.z > max_b.z || max_a.z < min_b.z)
    {
        return None;
    }

    let overlap_x = (max_a.x - min_b.x).min(max_b.x - min_a.x);
    let overlap_y = (max_a.y - min_b.y).min(max_b.y - min_a.y);
    let overlap_z = (max_a.z - min_b.z).min(max_b.z - min_a.z);

    let (penetration_depth, normal) = if overlap_x < overlap_y && overlap_x < overlap_z {
        (
            overlap_x,
            Vec3::X
                * (if player_position.x < other.center.x {
                    -1.0
                } else {
                    1.0
                }),
        )
    } else if overlap_y < overlap_z {
        (
            overlap_y,
            Vec3::Y
                * (if player_position.y < other.center.y {
                    -1.0
                } else {
                    1.0
                }),
        )
    } else {
        (
            overlap_z,
            Vec3::Z
                * (if player_position.z < other.center.z {
                    -1.0
                } else {
                    1.0
                }),
        )
    };

    Some(ContactInfo {
        normal,
        penetration_depth,
        contact_point_a: player_position,
        a_vel: Velocity {
            linear: player_velocity,
            angular: Vec3::ZERO,
        },
        contact_point_b: other.center,
        b_vel: *other_velocity,
    })
}

pub(crate) fn detect_object_collisions(
    mut query: Query<(&mut RigidbodyComponent, &mut Transform)>,
) {
    let mut items: Vec<_> = query.iter_mut().collect();

    for i in 0..items.len() {
        for j in (i + 1)..items.len() {
            let (left, right) = items.split_at_mut(j);
            let (ref mut body_a, ref mut transform_a) = left[i];
            let (ref mut body_b, ref mut transform_b) = right[0];

            if let Some(collision_data) = get_collision_info(
                &body_a.collider,
                &body_a.velocity,
                &body_b.collider,
                &body_b.velocity,
            ) {
                // println!(
                //     "collision -- normal: {:?}, penetration: {}",
                //     collision_data.normal, collision_data.penetration_depth
                // );
                resolve_object_collisions(
                    body_a,
                    transform_a,
                    body_b,
                    transform_b,
                    &collision_data,
                );
            }
        }
    }
}

fn resolve_object_collisions(
    a: &mut RigidbodyComponent,
    tf_a: &mut Transform,
    b: &mut RigidbodyComponent,
    tf_b: &mut Transform,
    contact: &ContactInfo,
) {
    let total_inverse_mass = a.inverse_mass + b.inverse_mass;
    if total_inverse_mass == 0.0 {
        return;
    }

    let correction = contact.normal * (contact.penetration_depth / total_inverse_mass);

    let temp_vel = a.velocity.linear;

    if a.rbt != RigidbodyType::Static {
        a.collider.center += correction * a.inverse_mass;
        if a.rbt == RigidbodyType::Static {
            a.velocity.linear = Vec3::ZERO;
        }
        a.velocity.linear *= 1. - a.friction;
        a.velocity.angular = Vec3::ZERO;
        a.velocity.linear += a.restitution * (contact.normal + temp_vel) * (1. - a.damping.linear)
            + contact.b_vel.linear;
        tf_a.translation = a.collider.center;
    }

    if a.rbt == RigidbodyType::Dynamic {
        let world_down = Vec3::Y;
        let local_down = a.collider.rotation * world_down;
        let align_rotation: Vec3 = rotation_to_align(local_down, contact.normal)
            .to_euler(EulerRot::XYZ)
            .into();
        println!("!!!");
        a.velocity.angular += align_rotation * a.inverse_mass * a.velocity.linear.length().max(6.5);
    }

    if b.rbt != RigidbodyType::Static {
        b.collider.center += correction * b.inverse_mass;
        if b.rbt == RigidbodyType::Static {
            b.velocity.linear = Vec3::ZERO;
        }
        b.velocity.angular = Vec3::ZERO;
        b.velocity.linear *= 1. - b.friction;
        b.velocity.linear += b.restitution * (contact.normal + temp_vel) * (1. - b.damping.linear)
            + contact.a_vel.linear;
        tf_b.translation = b.collider.center;
    }
}

fn project_collider(collider: &Collider, axis: Vec3) -> (f32, f32) {
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    for vertex in &collider.vertex_info.vertices {
        let projection = axis.dot(*vertex);
        min = min.min(projection);
        max = max.max(projection);
    }

    (min, max)
}

fn get_overlap(min_a: f32, max_a: f32, min_b: f32, max_b: f32) -> f32 {
    (max_a.min(max_b) - min_a.max(min_b)).max(0.0)
}

fn get_collision_info(
    a: &Collider,
    a_vel: &Velocity,
    b: &Collider,
    b_vel: &Velocity,
) -> Option<ContactInfo> {
    let axes = a.get_axes(b);
    let mut min_overlap = f32::INFINITY;
    let mut collision_axis = Vec3::ZERO;

    for axis in axes {
        let axis = axis.normalize();
        let (min_a, max_a) = project_collider(a, axis);
        let (min_b, max_b) = project_collider(b, axis);

        let overlap = get_overlap(min_a, max_a, min_b, max_b);
        if overlap <= 0.0 {
            return None;
        }

        if overlap < min_overlap {
            min_overlap = overlap;
            collision_axis = axis;
        }
    }

    Some(ContactInfo {
        normal: collision_axis,
        penetration_depth: min_overlap,
        contact_point_a: a.center,
        contact_point_b: b.center,
        a_vel: *a_vel,
        b_vel: *b_vel,
    })
}
