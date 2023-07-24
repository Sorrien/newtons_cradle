use crate::GameState;
use bevy::{pbr::NotShadowCaster, prelude::*};
pub struct MyScenePlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MyScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_camera)
            .add_systems(OnEnter(GameState::Playing), setup);
    }
}

fn setup_camera(mut commands: Commands, mut state: ResMut<NextState<GameState>>) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(15.0, 5.0, 42.0)
                .looking_at(Vec3::new(13.0, 1.0, 1.0), Vec3::Y),
            ..Default::default()
        },
        FogSettings {
            color: Color::rgba(0.1, 0.2, 0.4, 1.0),
            directional_light_color: Color::rgba(1.0, 0.95, 0.75, 0.5),
            directional_light_exponent: 30.0,
            falloff: FogFalloff::Linear {
                start: 50.0,
                end: 400.0,
            },
            /*              falloff: FogFalloff::from_visibility_colors(
                300.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
                Color::rgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
                Color::rgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
            ),  */
        },
    ));
    state.set(GameState::Playing);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(200.0).into()),
        material: materials.add(StandardMaterial {
            base_color: Color::SILVER,
            perceptual_roughness: 1.0,
            ..default()
        }),
        transform: Transform::from_xyz(10.0, -15.0, 0.0),
        ..default()
    });

    // ambient light
    /*commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.02,
    });*/

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
        /*         cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 2,
            first_cascade_far_bound: 200.0,
            maximum_distance: 280.0,
            ..default()
        }
        .into(), */
        ..default()
    });

    // Sky
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(100.0, 100.0, 100.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("888888").unwrap(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(20.0)),
            ..default()
        },
        NotShadowCaster,
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 200.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
}
