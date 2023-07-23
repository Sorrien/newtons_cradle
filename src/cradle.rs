use std::f32::consts::PI;

use crate::{loading::AudioAssets, GameState, audio::BallSound};
use bevy::{pbr::{CascadeShadowConfigBuilder, NotShadowCaster}, prelude::*};
use bevy_kira_audio::{Audio, AudioControl};
use bevy_rapier3d::prelude::*;

pub struct CradlePlugin;

/// This plugin handles the newtons cradle setup
impl Plugin for CradlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_newtons_cradle);     
    }
}

fn create_rope_joints(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    origin: Vect,
    use_dz: bool,
) {
    let rad = 1.0;
    let shift = 1.1;

    let cradle_offset = 15.0;

    let parent1 = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(
                origin.x,
                origin.y,
                origin.z - cradle_offset,
            )),
            RigidBody::Fixed,
            Collider::cuboid(rad, rad, rad),
        ))
        .id();

    let parent2 = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(
                origin.x,
                origin.y,
                origin.z + cradle_offset,
            )),
            RigidBody::Fixed,
            Collider::cuboid(rad, rad, rad),
        ))
        .id();

    let dz = if use_dz { -12.0 } else { 0.0 };

    let rope_max = 15.0;

    let ball_rope_attach_offset = 0.3;

    let rope1 = RopeJointBuilder::new()
        .local_anchor2(Vec3::new(0.0, -ball_rope_attach_offset, -shift))
        .limits([0.0, rope_max]);
    let joint1 = ImpulseJoint::new(parent1, rope1);

    let rope2 = RopeJointBuilder::new()
        .local_anchor2(Vec3::new(0.0, ball_rope_attach_offset, -shift))
        .limits([0.0, rope_max]);
    let joint2 = ImpulseJoint::new(parent2, rope2);

    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 1.0,
                    ..default()
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    perceptual_roughness: 0.1,
                    metallic: 1.0,
                    ..default()
                }),
                transform: Transform::from_xyz(
                    origin.x + dz,
                    origin.y - (rope_max + 5.0),
                    origin.z,
                ),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(rad),
            Friction::default(),
            Damping::default(), //emulate air resistance
            ColliderMassProperties::Density(2.0),
            Restitution {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Average,
            },
            Velocity::default(),
            BallSound::default(),
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .with_children(|parent| {
            // NOTE: we want to attach multiple impulse joints to this entity, so
            //       we need to add the components to children of the entity. Otherwise
            //       the second joint component would just overwrite the first one.
            parent.spawn(joint1);
            parent.spawn(joint2);
        });
}

pub fn setup_newtons_cradle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let starting_point = 5.0;
    let offset = 2.5;

    create_rope_joints(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(starting_point, 10.0, 0.0),
        true,
    );
    for i in 1..5 {
        create_rope_joints(
            &mut commands,
            &mut meshes,
            &mut materials,
            Vec3::new(i as f32 * offset + starting_point, 10.0, 0.0),
            false,
        );
    }
}