use bevy::{
    prelude::*,
    utils::HashSet,
};

#[derive(Resource, )]
struct GroupComponent{
    members: HashSet<Entity>,
    current_checked: Entity,
}

#[derive(Component)]
pub struct Checkable<CheckType>(CheckType);

#[derive(Component)]
pub struct CheckMarker;

fn on_checkable_clicked<T>(
    query: Query<(&Interaction, Checkable<T>, )
) {
}
