use bevy::{
    prelude::*,
    ui::widget::NodeImageMode,
};
use crate::config::MenuConfig;
use my_fluent_rs_helper::build_language_0;

#[derive(Component, Debug)]
pub(crate) struct TopMenu;

pub(crate) fn build_menu_bar<'a, 'b>(menu_config: &MenuConfig,
    asset_server: &mut AssetServer,
    parent: &'a mut ChildBuilder<'b>) {
    build_top_menu(menu_config, asset_server, parent);
}

fn build_top_menu<'a, 'b>(menu_config: &MenuConfig,
    asset_server: &mut AssetServer,
    parent: &'a mut ChildBuilder<'b>) {
    parent.spawn(
        Node {
            height: Val::Percent(5.),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        }
    )
        .with_children(|builder| {
            for x in &menu_config.menu_info {
                builder.spawn(Node {
                    height: Val::Percent(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    ..default()
                })
                .with_child(ImageNode::new(
                        match &x.icon_handle {
                            None => { panic!("Didn't load assets.") },
                            Some(s) => { s.clone() }
                        }).with_mode(NodeImageMode::Stretch))
                .with_child((
                        Text::new(build_language_0(&x.name)),
                        TextFont {
                            font: asset_server.load(&menu_config.default_font),
                            font_size: menu_config.default_text_size,
                            ..default()
                        }
                ));
            }
        });
}


