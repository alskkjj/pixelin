use bevy::prelude::*;
use bevy::ecs::system::SystemId;

use std::marker::PhantomData;


pub trait CheckAction {
    fn do_check<'w>(&mut self, commands: &mut Commands, parent: Entity, children: &'w [Entity]);
    fn do_uncheck<'w>(&mut self, commands: &mut Commands, entity: Entity, children: &'w [Entity]);
}

#[derive(Component)]
pub struct Checkable<CheckType: Sync + Send + CheckAction>(pub CheckType);

#[derive(Component)]
pub struct CheckMarker<CheckType: Sync + Send + CheckAction + 'static>(PhantomData<CheckType>);

#[derive(Event, Debug, )]
pub struct Checked;

#[derive(Resource)]
pub struct RemoveMarkedId<T: Sync + Send + CheckAction + 'static>(SystemId, PhantomData<T>);

pub fn init_me<CheckType>(app: &mut App)
    where CheckType: Clone + Send + Sync + CheckAction + 'static {
        app.add_systems(
            Update,
            on_checkable_clicked::<CheckType>,
        );
        let sid = app.register_system(remove_marked::<CheckType>);
        app.insert_resource(RemoveMarkedId(sid, PhantomData::<CheckType>));
}

fn remove_marked<T>(
    mut commands: Commands,
    marked: Option<Single<(Entity, &mut Checkable<T>, &Children), With<CheckMarker<T>>>>,
    ) 
    where T: Clone + Send + Sync + CheckAction + 'static
{
    if let Some(marked_single) = marked {
        let (marked_ent, mut checkable, children) = marked_single.into_inner();
        commands.entity(marked_ent).remove::<CheckMarker<T>>();
        checkable.0.do_uncheck(&mut commands, marked_ent, &children);
    }
}

fn on_checkable_clicked1<T: Clone + Send + Sync + CheckAction + 'static>(
    trigger: Trigger<Checked>,
    mut commands: Commands,
    mut checkable: Query<(Entity, &mut Checkable<T>), With<Checkable<T>>>,
    children_query: Query<&Children, With<Checkable<T>>>,
    sid: Res<RemoveMarkedId<T>>,
)
{
    let (parent, mut ck) = checkable.get_mut(trigger.entity())
        .expect("The Entity observed isn't checkable.");
    
    let children = children_query.children(parent);
    ck.0.do_check(&mut commands, parent, children);
    commands.run_system(sid.0);
}

fn on_checkable_clicked<T: Clone + Send + Sync + CheckAction + 'static>(
    mut checked: Query<(
        &Interaction, &mut Checkable<T>, Entity, ), 
    (Changed<Interaction>,With<Button>, With<Checkable<T>, >, Without<CheckMarker<T>>)>,
    last_checked: Option<Single<(Entity, &mut Checkable<T>, ), With<CheckMarker<T>>>>,
    children_query: Query<&Children>,
    mut commands: Commands,
) {

    '_debug: {
         for (inter,
             _, _,
             ) in &checked {
             debug!("checked query: {:?}", inter,);
         }
    }

     {
         if let Ok((interaction, mut checkable, entity, )) = checked.get_single_mut()
         {
             match interaction {
                 Interaction::Pressed => {
                     if let Some(last_checked) = last_checked {
                         let (last_checked, mut ch) = last_checked.into_inner();
                         let cs = children_query.children(last_checked);
                         commands.entity(last_checked).remove::<CheckMarker<T>>();
                         ch.0.do_uncheck(&mut commands, entity, cs);

                         if last_checked == entity {
                             return;
                         }
                     }

                     commands.entity(entity).insert(CheckMarker::<T>(PhantomData::<T>));
                     let cs = children_query.children(entity);
                     checkable.0.do_check(&mut commands, entity, cs);
                 },
                 _ => {
                 }
             }
         }
     }
}
