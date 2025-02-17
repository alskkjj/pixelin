use bevy::{
    prelude::*,
    utils::HashSet,
};

#[derive(Resource, )]
struct GroupComponent{
    members: HashSet<Entity>,
    current_checked: Entity,
}

pub trait Checkable {
    fn do_check();
    fn do_uncheck();
}


