use crate::{loading::AudioAssets, GameState};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_rapier3d::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin).add_systems(
            Update,
            handle_ball_impact_sounds.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component, Default)]
pub struct BallSound {}

fn handle_ball_impact_sounds(
    mut collision_events: EventReader<CollisionEvent>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    velocities: Query<&Velocity>,
) {
    for event in collision_events.iter() {
        let (entity_a, entity_b, _ongoing) = unpack_collision_event(event);

        if let Ok(velocity_a) = velocities.get(entity_a) {
            if let Ok(velocity_b) = velocities.get(entity_b) {
                let rel_velocity = (velocity_a.linvel - velocity_b.linvel).abs();
                let volume = (rel_velocity.length() / 10.0).clamp(0.0, 1.0) as f64;

                if volume > 0.2 {
                    let _handle = audio
                        .play(audio_assets.newton_impact.clone())
                        .with_volume(volume)
                        .handle();
                }
            }
        }
    }
}

fn unpack_collision_event(event: &CollisionEvent) -> (Entity, Entity, bool) {
    match event {
        CollisionEvent::Started(entity_a, entity_b, _kind) => (*entity_a, *entity_b, true),
        CollisionEvent::Stopped(entity_a, entity_b, _kind) => (*entity_a, *entity_b, false),
    }
}
