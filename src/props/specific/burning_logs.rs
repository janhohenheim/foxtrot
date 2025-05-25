use avian3d::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use bevy_trenchbroom::prelude::*;

use crate::{
    PostPhysicsAppSystems,
    props::{effects::disable_shadow_casting_on_instance_ready, setup::static_bundle},
    screens::Screen,
};
#[cfg(feature = "native")]
use crate::{RenderLayer, asset_tracking::LoadResource as _};
use bevy::{
    audio::{SpatialScale, Volume},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    #[cfg(feature = "native")]
    // This causes https://github.com/bevyengine/bevy/issues/18980
    app.load_resource::<BurningLogsAssets>();
    app.register_type::<Flicker>();
    app.add_systems(
        Update,
        flicker_light
            .run_if(in_state(Screen::Gameplay))
            .in_set(PostPhysicsAppSystems::Update),
    );
    app.add_observer(setup_burning_logs);
    #[cfg(feature = "native")]
    app.add_observer(particles::add_particle_effects);
    app.register_type::<BurningLogs>();
}

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/fireplace/burntwood.gltf")]
#[spawn_hooks(SpawnHooks::new().preload_model::<Self>())]
pub(crate) struct BurningLogs;

#[derive(Resource, Asset, Clone, TypePath)]
struct BurningLogsAssets {
    #[dependency]
    texture: Handle<Image>,
    #[dependency]
    sound: Handle<AudioSource>,
}

impl FromWorld for BurningLogsAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            texture: assets.load(TEXTURE_PATH),
            sound: assets.load(SOUND_PATH),
        }
    }
}

pub(crate) const TEXTURE_PATH: &str = {
    #[cfg(feature = "dev")]
    {
        "images/Flame.png"
    }
    #[cfg(not(feature = "dev"))]
    {
        "images/Flame.ktx2"
    }
};
const SOUND_PATH: &str = "audio/music/loop_flames_03.ogg";

const BASE_INTENSITY: f32 = 150_000.0;

#[cfg_attr(feature = "hot_patch", hot)]
fn setup_burning_logs(
    trigger: Trigger<OnAdd, BurningLogs>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let static_bundle =
        static_bundle::<BurningLogs>(&asset_server, ColliderConstructor::ConvexHullFromMesh);
    let sound_effect: Handle<AudioSource> = asset_server.load(SOUND_PATH);

    commands
        .entity(trigger.target())
        .insert((
            static_bundle,
            AudioPlayer(sound_effect),
            PlaybackSettings::LOOP
                .with_spatial(true)
                .with_volume(Volume::Linear(0.25))
                .with_spatial_scale(SpatialScale::new(0.3)),
        ))
        .observe(disable_shadow_casting_on_instance_ready)
        .with_child((
            PointLight {
                color: Color::srgb(1.0, 0.7, 0.4),
                intensity: BASE_INTENSITY,
                radius: 0.1,
                shadows_enabled: true,
                #[cfg(feature = "native")]
                soft_shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(0.0, 0.2, 0.0),
            Flicker,
        ));
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
struct Flicker;

#[cfg_attr(feature = "hot_patch", hot)]
fn flicker_light(time: Res<Time>, mut query: Query<&mut PointLight, With<Flicker>>) {
    for mut light in &mut query {
        let flickers_per_second = 20.0;
        let flicker_percentage = 0.1;
        let flicker = (time.elapsed_secs() * flickers_per_second).sin();
        light.intensity = BASE_INTENSITY + flicker * BASE_INTENSITY * flicker_percentage;
    }
}

#[cfg(feature = "native")]
mod particles {
    use super::*;

    use bevy::render::view::RenderLayers;
    use bevy_hanabi::prelude::*;
    use std::f32::consts::TAU;

    #[cfg_attr(feature = "hot_patch", hot)]
    pub(super) fn add_particle_effects(
        trigger: Trigger<OnAdd, BurningLogs>,
        asset_server: Res<AssetServer>,
        mut effects: ResMut<Assets<EffectAsset>>,
        mut commands: Commands,
    ) {
        let particle_bundle = particle_bundle(&asset_server, &mut effects);
        commands.entity(trigger.target()).insert(particle_bundle);
    }

    fn particle_bundle(
        asset_server: &AssetServer,
        effects: &mut Assets<EffectAsset>,
    ) -> impl Bundle {
        let effect_handle = setup_particles(effects);
        let texture: Handle<Image> = asset_server.load(TEXTURE_PATH);
        (
            ParticleEffect::new(effect_handle),
            RenderLayers::from(RenderLayer::PARTICLES),
            EffectMaterial {
                images: vec![texture.clone()],
            },
        )
    }

    fn setup_particles(effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
        let writer = ExprWriter::new();

        // Random upward velocity with some lateral randomness for flicker
        let mean_velocity = writer.lit(Vec3::new(0.0, 1.5, 0.0));
        let sd_velocity = writer.lit(Vec3::new(0.2, 0.5, 0.2));
        let velocity = SetAttributeModifier::new(
            Attribute::VELOCITY,
            mean_velocity.normal(sd_velocity).expr(),
        );

        // Load the texture
        let particle_texture_modifier = ParticleTextureModifier {
            texture_slot: writer.lit(0u32).expr(),
            sample_mapping: ImageSampleMapping::Modulate,
        };

        // Random rotation
        let orientation = OrientModifier {
            rotation: Some(writer.lit(0.0).uniform(writer.lit(TAU)).expr()),
            mode: OrientMode::FaceCameraPosition,
        };

        let mut module = writer.finish();
        module.add_texture_slot("shape");

        // Spawn from small spherical area at the base
        let init_pos = SetPositionSphereModifier {
            center: module.lit(Vec3::Y * 0.2),
            radius: module.lit(0.35),
            dimension: ShapeDimension::Volume,
        };

        // Short lifetime for fire particles
        let lifetime = SetAttributeModifier::new(Attribute::LIFETIME, module.lit(0.4));

        // Constant upward acceleration (mimics heat rise)
        let accel = module.lit(Vec3::Y * 0.4);
        let update_accel = AccelModifier::new(accel);

        // Additive blending to simulate light emission
        let alpha_mode = bevy_hanabi::AlphaMode::Add;

        // Color gradient for fire: transparent → bright yellow → orange → dark red → transparent
        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(0.0, 0.0, 0.0, 0.0)); // transparent
        gradient.add_key(0.1, Vec4::new(1.0, 0.8, 0.0, 1.0)); // bright yellow
        gradient.add_key(0.3, Vec4::new(1.0, 0.4, 0.0, 1.0)); // orange
        gradient.add_key(0.6, Vec4::new(0.6, 0.0, 0.0, 0.8)); // dark red
        gradient.add_key(1.0, Vec4::new(0.0, 0.0, 0.0, 0.0)); // transparent
        let color_over_lifetime = ColorOverLifetimeModifier {
            gradient,
            ..default()
        };

        // Size over lifetime modifier: small -> larger -> fade out
        let mut size_curve = Gradient::new();
        size_curve.add_key(0.0, Vec3::splat(0.2)); // start small
        size_curve.add_key(0.3, Vec3::splat(0.5)); // grow
        size_curve.add_key(1.0, Vec3::splat(0.0)); // shrink to nothing

        let size_over_lifetime = SizeOverLifetimeModifier {
            gradient: size_curve,
            screen_space_size: false,
        };

        const MAX_PARTICLES: u32 = 32768;
        let effect = EffectAsset::new(MAX_PARTICLES, SpawnerSettings::rate(150.0.into()), module)
            .with_name("FireEffect")
            .init(init_pos)
            .init(velocity)
            .init(lifetime)
            .with_alpha_mode(alpha_mode)
            .update(update_accel)
            .render(orientation)
            .render(color_over_lifetime)
            .render(particle_texture_modifier)
            .render(size_over_lifetime);

        effects.add(effect)
    }
}
