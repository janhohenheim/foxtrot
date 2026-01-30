use std::mem::discriminant;

use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct AnimationState<T> {
    state: Option<T>,
}

impl<T> Default for AnimationState<T> {
    fn default() -> Self {
        Self { state: None }
    }
}

pub(crate) enum AnimationStateTransition<'a, T> {
    Maintain {
        state: &'a T,
    },

    Alter {
        #[allow(dead_code)]
        old_state: Option<T>,
        state: &'a T,
    },
}

impl<T> AnimationState<T> {
    pub(crate) fn update_by(
        &'_ mut self,
        new_state: T,
        comparison: impl FnOnce(&T, &T) -> bool,
    ) -> AnimationStateTransition<'_, T> {
        let is_same = self
            .state
            .as_ref()
            .is_some_and(|old_state| comparison(old_state, &new_state));

        let old_state = self.state.replace(new_state);

        if is_same {
            AnimationStateTransition::Maintain {
                state: self.state.as_ref().expect("state was just placed there"),
            }
        } else {
            AnimationStateTransition::Alter {
                old_state,

                state: self.state.as_ref().expect("state was just placed there"),
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn update_by_value(&'_ mut self, new_state: T) -> AnimationStateTransition<'_, T>
    where
        T: PartialEq,
    {
        self.update_by(new_state, |a, b| a == b)
    }

    pub(crate) fn update_by_discriminant(
        &'_ mut self,
        new_state: T,
    ) -> AnimationStateTransition<'_, T> {
        self.update_by(new_state, |a, b| discriminant(a) == discriminant(b))
    }

    #[allow(dead_code)]
    pub(crate) fn get(&self) -> Option<&T> {
        self.state.as_ref()
    }
}
