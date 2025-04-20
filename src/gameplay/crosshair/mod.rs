use std::any::TypeId;

use assets::CursorAssets;
use bevy::{prelude::*, utils::HashSet};

use crate::{AppSet, screens::Screen};

mod assets;
pub(crate) mod cursor;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CrosshairState>();

    app.add_systems(
        Update,
        update_crosshair
            .run_if(in_state(Screen::Gameplay))
            .in_set(AppSet::ChangeUi),
    );
    app.add_systems(OnEnter(Screen::Gameplay), spawn_crosshair);

    app.add_plugins((assets::plugin, cursor::plugin));
}

/// Show a crosshair for better aiming
fn spawn_crosshair(mut commands: Commands, assets: Res<CursorAssets>) {
    commands
        .spawn((
            Name::new("Crosshair"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            StateScoped(Screen::Gameplay),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Crosshair Image"),
                CrosshairState::default(),
                ImageNode::new(assets.crosshair_dot.clone()),
            ));
        });
}

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component, Default)]
pub(crate) struct CrosshairState {
    pub(crate) wants_square: HashSet<TypeId>,
    pub(crate) wants_invisible: HashSet<TypeId>,
}

fn update_crosshair(
    crosshair: Option<
        Single<(&CrosshairState, &mut ImageNode, &mut Visibility), Changed<CrosshairState>>,
    >,
    assets: Res<CursorAssets>,
) {
    let Some((crosshair_state, mut image_node, mut visibility)) = crosshair.map(|c| c.into_inner())
    else {
        return;
    };
    if crosshair_state.wants_square.is_empty() {
        image_node.image = assets.crosshair_dot.clone();
    } else {
        image_node.image = assets.crosshair_square.clone();
    }

    if crosshair_state.wants_invisible.is_empty() {
        *visibility = Visibility::Inherited;
    } else {
        *visibility = Visibility::Hidden;
    }
}
