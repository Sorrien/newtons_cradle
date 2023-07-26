use crate::{loading::TextureAssets, GameState};
use bevy::{
    core_pipeline::Skybox,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};
pub struct MyScenePlugin;

impl Plugin for MyScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_camera)
            .add_systems(OnEnter(GameState::Playing), setup);
    }
}

fn setup_camera(
    mut commands: Commands,
    mut state: ResMut<NextState<GameState>>,
    textures: Res<TextureAssets>,
    mut images: ResMut<Assets<Image>>,
) {
    let image = images.get_mut(&textures.skybox_cubemap).unwrap();
    // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
    // so they appear as one texture. The following code reconfigures the texture as necessary.
    if image.texture_descriptor.array_layer_count() == 1 {
        image.reinterpret_stacked_2d_as_array(
            image.texture_descriptor.size.height / image.texture_descriptor.size.width,
        );
        image.texture_view_descriptor = Some(TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        });
    };

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
        EnvironmentMapLight {
            diffuse_map: textures.skybox_cubemap.clone(),
            specular_map: textures.skybox_cubemap.clone(),
        },
        Skybox(textures.skybox_cubemap.clone()),
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
