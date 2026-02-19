//! A splash screen that plays briefly at startup.

use bevy::{asset::embedded_asset, input::common_conditions::input_just_pressed, prelude::*};

use crate::{PostPhysicsAppSystems, screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    // Spawn splash screen.
    app.insert_resource(ClearColor(SPLASH_BACKGROUND_COLOR));
    app.add_systems(OnEnter(Screen::Splash), spawn_splash_screen);
    embedded_asset!(app, "files/splash.png");

    // Animate splash screen.
    app.add_systems(
        Update,
        (
            tick_fade_in_out.in_set(PostPhysicsAppSystems::TickTimers),
            apply_fade_in_out.in_set(PostPhysicsAppSystems::Update),
        )
            .run_if(in_state(Screen::Splash)),
    );

    // Add splash timer.
    app.add_systems(OnEnter(Screen::Splash), insert_splash_timer);
    app.add_systems(
        Update,
        (
            tick_splash_timer.in_set(PostPhysicsAppSystems::TickTimers),
            check_splash_timer.in_set(PostPhysicsAppSystems::Update),
        )
            .run_if(in_state(Screen::Splash)),
    );

    // Exit the splash screen early if the player hits escape.
    app.add_systems(
        Update,
        enter_title_screen
            .run_if(input_just_pressed(KeyCode::Escape).and(in_state(Screen::Splash))),
    );
}

const SPLASH_BACKGROUND_COLOR: Color = Color::srgb(0.157, 0.157, 0.157);
const SPLASH_DURATION_SECS: f32 = 1.8;
const SPLASH_FADE_DURATION_SECS: f32 = 0.6;

fn spawn_splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            widget::ui_root("Splash Screen"),
            DespawnOnExit(Screen::Splash),
            children![(
                Name::new("Splash image"),
                Node {
                    margin: UiRect::all(Val::Auto),
                    width: Val::Percent(70.0),
                    ..default()
                },
                ImageNode::new(asset_server.load("embedded://foxtrot/screens/files/splash.png")),
                ImageNodeFadeInOut {
                    total_duration: SPLASH_DURATION_SECS,
                    fade_duration: SPLASH_FADE_DURATION_SECS,
                    t: 0.0,
                },
            )],
        ))
        // Override the default background color provided by `ui_root`
        .insert(BackgroundColor(SPLASH_BACKGROUND_COLOR));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ImageNodeFadeInOut {
    /// Total duration in seconds.
    total_duration: f32,
    /// Fade duration in seconds.
    fade_duration: f32,
    /// Current progress in seconds, between 0 and [`Self::total_duration`].
    t: f32,
}

impl ImageNodeFadeInOut {
    fn alpha(&self) -> f32 {
        // Normalize by duration.
        let t = (self.t / self.total_duration).clamp(0.0, 1.0);
        let fade = self.fade_duration / self.total_duration;

        // Regular trapezoid-shaped graph, flat at the top with alpha = 1.0.
        ((1.0 - (2.0 * t - 1.0).abs()) / fade).min(1.0)
    }
}

fn tick_fade_in_out(time: Res<Time>, mut animation_query: Query<&mut ImageNodeFadeInOut>) {
    for mut anim in &mut animation_query {
        anim.t += time.delta_secs();
    }
}

fn apply_fade_in_out(mut animation_query: Query<(&ImageNodeFadeInOut, &mut ImageNode)>) {
    for (anim, mut image) in &mut animation_query {
        image.color.set_alpha(anim.alpha())
    }
}

#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
struct SplashTimer(Timer);

impl Default for SplashTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(SPLASH_DURATION_SECS, TimerMode::Once))
    }
}

fn insert_splash_timer(mut commands: Commands) {
    commands.spawn((SplashTimer::default(), DespawnOnExit(Screen::Splash)));
}

fn tick_splash_timer(time: Res<Time>, mut timer: Single<&mut SplashTimer>) {
    timer.0.tick(time.delta());
}

fn check_splash_timer(timer: Single<&SplashTimer>, mut next_screen: ResMut<NextState<Screen>>) {
    if timer.0.just_finished() {
        next_screen.set(Screen::Title);
    }
}

fn enter_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
