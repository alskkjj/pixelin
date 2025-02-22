use bevy::{
    prelude::*,
    ui::widget::NodeImageMode,
    color::palettes::css,
};

use my_fluent_rs_helper::build_language_0;

use crate::config::AppConfig;
use crate::tools::*;
use crate::config::ToolsConfig;

pub(crate) fn build_tools_bar<'a, 'b>(tools_config: &mut ToolsConfig, asset_server: &mut AssetServer,
    parent: &'a mut ChildBuilder<'b>) {
    build_bottom_menu(tools_config, asset_server, parent)
}

fn build_bottom_menu<'a, 'b>(tools_config: &mut ToolsConfig, asset_server: &mut AssetServer,
    parent: &'a mut ChildBuilder<'b>) {

    parent.spawn(( // Container of bottom tools.
            Node { 
                height: Val::Percent(5.),
 //               border: UiRect::all(Val::Px(1.0)),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
    )).with_children(|builder| {
        for x in &tools_config.tools_info { 
            let id = builder.spawn(( // container of clickable
                    Button,
                    // clickable area.
                    PickingBehavior {
                        should_block_lower: true,
                        is_hoverable: true,
                    },

                    BottomTools {tool_name: x.name.to_owned()} ,

                    Node {
                        height: Val::Percent(100.),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexStart,
                        row_gap: Val::Px(0.5),
                        ..default()
                    }, 
            )).with_child( ( // clickable area
                        Node {
                            width: Val::Px(30.),
                            height: Val::Percent(95.),
                            margin: UiRect::all(Val::Px(4.)),
                            ..default()
                        },
                        ImageNode::new(
                            match &x.icon_handle {
                                None => { panic!("Didn't load assets.") }
                                Some(s) => s.clone()
                            }
                        ).with_mode(NodeImageMode::Auto),
                        Outline {
                            width: Val::Px(1.),
                            offset: Val::Px(1.0),
                            color: tools_config.deselecting_color.into(),
                        })
            )
                .with_child((
                        Text::new(build_language_0(&x.name)),
                        TextFont {
                            font: asset_server.load(&tools_config.default_font),
                            font_size: tools_config.default_text_size,
                            ..default()
                        },
                        TextColor(css::LIME.into()),
                )).id();
            tools_config.exclusive_tools.insert(x.name.clone(), id);
        } // for tool config in ...
    });
}


pub(crate) fn bottom_tools_clicked(mut query: Query<
    (&Interaction, &BottomTools, &Children),
    (Changed<Interaction>, With<Button>, With<BottomTools>)
    >,
    mut app_config: ResMut<AppConfig>,
    mut outlines: Query<&mut Outline>) {
    let selecting_color = app_config.tools_config.selecting_color;
    let deselecting_color = app_config.tools_config.deselecting_color;

    let mut need_to_close_ents = Vec::new();
        //children.get(0);
    for (interaction, bottom_tool, children) in &mut query {
        match interaction {
            Interaction::Pressed => {
                if let Ok(mut outline) = outlines.get_mut(children[0]) {
                    outline.color = selecting_color.into();
                    app_config.tools_config.current_tool = Some(bottom_tool.tool_name.clone());
                } else {
                    warn!("There is a bottom tool without outline in first child.");
                }
            }
            _ => {
                if let Some(e) = children.get(0) {
                    need_to_close_ents.push(e);
                }
            }
        }
    }
    for ent in need_to_close_ents {
        if let Ok(mut outline) = outlines.get_mut(*ent) {
            if outline.color == selecting_color.into() {
                outline.color = deselecting_color.into();
                app_config.tools_config.current_tool = None;
            }
        }
    }
}


