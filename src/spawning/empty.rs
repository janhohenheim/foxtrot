use crate::spawning::PrimedGameObjectSpawner;
use bevy::ecs::system::EntityCommands;

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn_empty(&'a mut self) -> EntityCommands<'w, 's, 'a> {
        self.commands.spawn_empty()
    }
}
