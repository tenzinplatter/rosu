use bevy::ecs::system::SystemId;
use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Callback {
    pub(crate) system_id: SystemId<(), ()>,
    pub(crate) timer: Timer,
}

pub struct CallbackPlugin;

impl Plugin for CallbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_callbacks);
    }
}

fn handle_callbacks(
    mut query: Query<(&mut Callback, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut cb, entity) in &mut query {
        if cb.timer.tick(time.delta()).just_finished() {
            commands.run_system(cb.system_id);
            commands.unregister_system(cb.system_id);
            commands.entity(entity).despawn();
        }
    }
}
