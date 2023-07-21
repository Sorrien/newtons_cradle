use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

use crate::GameState;

pub struct InteractionPlugin;

/// This plugin handles player related stuff like movement
impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorState::default())
            .add_systems(
                Update,
                (
                    my_cursor_system.run_if(in_state(GameState::Playing)),
                    handle_drag_selection.run_if(in_state(GameState::Playing)),
                    handle_drag_release.run_if(in_state(GameState::Playing)),
                    handle_drag.run_if(in_state(GameState::Playing)),
                ),
            )
            .add_systems(OnEnter(GameState::Playing), setup_cursor_entity);
    }
}

fn setup_cursor_entity(mut commands: Commands) {
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
        RigidBody::KinematicPositionBased,
        CursorInteractor::default(),
    ));
}

fn handle_drag_selection(
    buttons: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut cursor_state: ResMut<CursorState>,
    transforms: Query<&Transform, Without<CursorInteractor>>,
    mut cursor_interactor_q: Query<(Entity, &mut Transform), With<CursorInteractor>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (cursor_entity, mut cursor_transform) = cursor_interactor_q.single_mut();

        if cursor_state.current_hit_entity.is_some() {
            cursor_state.drag_entity = cursor_state.current_hit_entity;
            if let Some(drag_entity) = cursor_state.drag_entity {
                if let Ok(drag_transform) = transforms.get(drag_entity) {
                    cursor_transform.translation = drag_transform.translation;
                    cursor_transform.rotation = drag_transform.rotation;
                }

                let unlocked_axis = Vec3::X;
                let fixed_joint = RevoluteJointBuilder::new(unlocked_axis).build();
                let joint = ImpulseJoint::new(cursor_entity, fixed_joint);

                commands.entity(drag_entity).with_children(|parent| {
                    parent.spawn((joint, CursorInteractorJoint::default()));
                });
            }
        }
    }
}

fn handle_drag_release(
    buttons: Res<Input<MouseButton>>,
    cursor_joint_q: Query<Entity, With<CursorInteractorJoint>>,
    mut commands: Commands,
) {
    if buttons.just_released(MouseButton::Left) {
        for cursor_joint in &cursor_joint_q {
            commands.entity(cursor_joint).despawn();
        }
    }
}

fn handle_drag(
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut motion_evr: EventReader<MouseMotion>,
    mut cursor_interactor_transform_q: Query<&mut Transform, With<CursorInteractor>>,
) {
    if buttons.pressed(MouseButton::Left) {
        let mut cursor_transform = cursor_interactor_transform_q.single_mut();
        for ev in motion_evr.iter() {
            cursor_transform.translation.x += ev.delta.x * time.delta_seconds();
            cursor_transform.translation.y += ev.delta.y * -1. * time.delta_seconds();
        }
    }
}

fn my_cursor_system(
    // need to get window dimensions
    windows: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
    mut cursor_state: ResMut<CursorState>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    let window = windows.single();
    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(cursor_position) = window.cursor_position() {
        if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
            cursor_state.current_cursor_position = ray.origin;

            if let Some((hit_entity, _)) =
                rapier_context.cast_ray(ray.origin, ray.direction, 1000.0, true, QueryFilter::new())
            {
                cursor_state.current_hit_entity = Some(hit_entity);
            } else {
                cursor_state.current_hit_entity = None;
            }
        }
    }
}

#[derive(Resource, Default)]
struct CursorState {
    current_hit_entity: Option<Entity>,
    drag_entity: Option<Entity>,
    current_cursor_position: Vec3,
}

#[derive(Component, Default)]
struct CursorInteractor {}

#[derive(Component, Default)]
struct CursorInteractorJoint {}
