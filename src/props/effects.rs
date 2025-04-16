use avian_pickup::output::PropThrown;
use avian3d::prelude::*;
use bevy::{color::palettes::tailwind::RED_400, prelude::*, utils::HashSet};
use bevy_hanabi::prelude::*;

use super::generic::DynamicProp;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedPostUpdate,
        print_collisions.after(PhysicsStepSet::Last),
    );
}

fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let spawner = SpawnerSettings::once(100.0.into())
        // Disable starting emitting particles when the EffectSpawner is instantiated. We want
        // complete control, and only emit when reset() is called.
        .with_emit_on_start(false);

    let writer = ExprWriter::new();

    // Init the age of particles to 0, and their lifetime to 1.5 second.
    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(1.5).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Add a bit of linear drag to slow down particles after the inital spawning.
    // This keeps the particle around the spawn point, making it easier to visualize
    // the different groups of particles.
    let drag = writer.lit(2.).expr();
    let update_drag = LinearDragModifier::new(drag);

    // Bind the initial particle color to the value of the 'spawn_color' property
    // when the particle spawns. The particle will keep that color afterward,
    // even if the property changes, because the color will be saved
    // per-particle (due to the Attribute::COLOR).
    let spawn_color = writer.add_property("spawn_color", 0xFFFFFFFFu32.into());
    let color = writer.prop(spawn_color).expr();
    let init_color = SetAttributeModifier::new(Attribute::COLOR, color);

    let normal = writer.add_property("normal", Vec3::ZERO.into());
    let normal = writer.prop(normal);

    // Set the position to be the collision point, which in this example is always
    // the emitter position (0,0,0) at the ball center, minus the ball radius
    // alongside the collision normal. Also raise particle to Z=0.2 so they appear
    // above the black background box.
    //   pos = -normal * BALL_RADIUS + Z * 0.2;
    const BALL_RADIUS: f32 = 0.05;
    let pos = normal.clone() * writer.lit(-BALL_RADIUS) + writer.lit(Vec3::Z * 0.2);
    let init_pos = SetAttributeModifier::new(Attribute::POSITION, pos.expr());

    // Set the velocity to be a random direction mostly along the collision normal,
    // but with some spread. This cheaply ensures that we spawn only particles
    // inside the black background box (or almost; we ignore the edge case around
    // the corners). An alternative would be to use something
    // like a KillAabbModifier, but that would spawn particles and kill them
    // immediately, wasting compute resources and GPU memory.
    //   tangent = cross(Z, normal);
    //   spread = frand() * 2. - 1.;  // in [-1:1]
    //   speed = frand() * 0.2;
    //   velocity = normalize(normal + tangent * spread * 5.) * speed;
    let tangent = writer.lit(Vec3::Z).cross(normal.clone());
    let spread = writer.rand(ScalarType::Float) * writer.lit(2.) - writer.lit(1.);
    let speed = writer.rand(ScalarType::Float) * writer.lit(0.2);
    let velocity = (normal + tangent * spread * writer.lit(5.0)).normalized() * speed;
    let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, velocity.expr());

    let effect = effects.add(
        EffectAsset::new(32768, spawner, writer.finish())
            .with_name("spawn_on_command")
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .init(init_color)
            .update(update_drag)
            // Set a size of 3 (logical) pixels, constant in screen space, independent of projection
            .render(SetSizeModifier {
                size: Vec3::splat(3.).into(),
            })
            .render(ScreenSpaceSizeModifier),
    );

    commands.spawn((
        ParticleEffect::new(effect),
        EffectProperties::default(),
        Name::new("effect"),
    ));
}

fn print_collisions(
    mut collision_started: EventReader<CollisionStarted>,
    mut collision_event_reader: EventReader<Collision>,
    q_dynamic_prop: Query<&DynamicProp>,
    time: Res<Time<Substeps>>,
    q_name: Query<&Name>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q_transform: Query<&Transform>,
    q_collider_parent: Query<&ColliderParent>,
    mut mesh: Local<Option<Handle<Mesh>>>,
    mut material: Local<Option<Handle<StandardMaterial>>>,
) {
    let collisions_started = collision_started
        .read()
        .flat_map(|event| [event.0, event.1])
        .collect::<HashSet<_>>();
    for Collision(contacts) in collision_event_reader.read() {
        if !collisions_started.contains(&contacts.entity1)
            && !collisions_started.contains(&contacts.entity2)
        {
            continue;
        }
        let body1 = q_collider_parent.get(contacts.entity1).unwrap().get();
        let body2 = q_collider_parent.get(contacts.entity2).unwrap().get();
        let is_dynamic_prop = |entity: Entity| q_dynamic_prop.get(entity).is_ok();
        if !(is_dynamic_prop(body1) || is_dynamic_prop(body2)) {
            continue;
        }

        let normal_force = contacts.total_normal_force(time.delta_secs());
        let normal_impulse = contacts.total_normal_impulse;
        let tangent_force = contacts.total_tangent_force(time.delta_secs());
        let tangent_impulse = contacts.total_tangent_impulse;
        if normal_force < 50.0 {
            continue;
        }
        info!("--------------------------------");
        info!("Normal force: {:?}", normal_force);
        info!("Normal impulse: {:?}", normal_impulse);
        info!("Tangent force: {:?}", tangent_force);
        info!("Tangent impulse: {:?}", tangent_impulse);

        let dynamic_prop = if is_dynamic_prop(body1) { body1 } else { body2 };
        let xform = q_transform.get(dynamic_prop).unwrap().clone();
        if mesh.is_none() {
            *mesh = Some(meshes.add(Cuboid::from_length(0.3)));
        }
        if material.is_none() {
            *material = Some(materials.add(Color::from(RED_400)));
        }
        commands.spawn((
            xform,
            Mesh3d(mesh.clone().unwrap()),
            MeshMaterial3d(material.clone().unwrap()),
        ));

        let name1 = q_name.get(body1).unwrap().to_string();
        let name2 = q_name.get(body2).unwrap().to_string();
        info!(
            "Spawned prop: {} and {} at {}",
            name1, name2, xform.translation
        );
    }
}

/*
fn update(
    mut balls: Query<(&mut Ball, &mut Transform)>,
    mut effect: Query<(&mut EffectProperties, &mut EffectSpawner, &mut Transform), Without<Ball>>,
    time: Res<Time>,
) {
    // Note: On first frame where the effect spawns, EffectSpawner is spawned during
    // PostUpdate, so will not be available yet. Ignore for a frame if so.
    let Ok((mut properties, mut effect_spawner, mut effect_transform)) = effect.get_single_mut()
    else {
        return;
    };

    for (mut ball, mut transform) in balls.iter_mut() {
        let mut pos = transform.translation.xy() + ball.velocity * time.delta_secs();
        let mut collision = false;

        let mut normal = Vec2::ZERO;

        transform.translation = pos.extend(transform.translation.z);

        if collision {
            // This isn't the most accurate place to spawn the particle effect,
            // but this is just for demonstration, so whatever.
            effect_transform.translation = transform.translation;

            // Pick a random particle color
            let r = rand::random::<u8>();
            let g = rand::random::<u8>();
            let b = rand::random::<u8>();
            let color = 0xFF000000u32 | (b as u32) << 16 | (g as u32) << 8 | (r as u32);
            properties.set("spawn_color", color.into());

            // Set the collision normal
            let normal = normal.normalize();
            info!("Collision: n={:?}", normal);
            properties.set("normal", normal.extend(0.).into());

            // Spawn the particles
            effect_spawner.reset();
        }
    }
}
*/
