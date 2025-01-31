use my_fluent_rs_helper::*;

mod mix_methods;
mod tools;
mod patterns;


use bevy_pancam::*;
use bevy::{
    asset::RenderAssetUsages,
    color::palettes::{css, tailwind},
    image::ImageSampler, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat, },
    ui::widget::NodeImageMode,
    log::once,
};

use std::borrow::BorrowMut;
use std::collections::HashMap;

#[derive(Component, Debug)]
struct BottomTools {
    pub tool_name: String,
}

#[derive(Component, Debug)]
struct TopMenu;

#[derive(Component, Debug)]
struct Brush;

#[derive(Component, Debug)]
struct Bucket;

#[derive(Debug)]
pub(crate) struct ToolInfo {
    name: String, // name to translate.
    icon: String, // path to icon in assets.
    icon_handle: Option<Handle<Image>>,
}

#[derive(Debug)]
pub(crate) struct MenuInfo {
    name: String,
    icon: String,
    icon_handle: Option<Handle<Image>>,
}

                
#[derive(Debug)]
pub(crate) struct ToolsConfig {
    tools_info: Vec<ToolInfo>,
    default_font: String,
    default_text_size: f32,

    selecting_color: Srgba,
    deselecting_color: Srgba,

    exclusive_tools: HashMap<String, Entity>,
    current_tool: Option<String>,
}


#[derive(Debug)]
struct MenuConfig {
    menu_info: Vec<MenuInfo>,
    default_font: String,
    default_text_size: f32,

    selecting_color: Srgba,
    deselecting_color: Srgba,
}

#[derive(Debug, Resource)]
struct AppConfig {
    default_canvas_size: Extent3d,
    default_clear_color: Srgba,

    default_top_menu_percentage: f32,
    default_bottom_menu_percentage: f32,

    tools_config: ToolsConfig,
    menu_config: MenuConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            default_canvas_size: Extent3d {
                width: 320u32, 
                height: 320u32,
                depth_or_array_layers: 1u32,
            },
            default_clear_color: css::WHITE,

            default_top_menu_percentage: 5.,
            default_bottom_menu_percentage: 5.,
            tools_config: ToolsConfig { tools_info: vec![
                ToolInfo {
                    name: "pencil".to_owned(),
                    icon: "icons/pencil.png".to_owned(),
                    icon_handle: None,
                },
                ToolInfo {
                    name: "bucket".to_owned(),
                    icon: "icons/bucket.png".to_owned(),
                    icon_handle: None,
                },],

                default_font: "fonts/AaBianYaKai-2.ttf".to_owned(),
                default_text_size: 7f32,
                selecting_color: css::RED.into(),
                deselecting_color: css::WHITE.into(),
                exclusive_tools: HashMap::new(),
                current_tool: None,
            },
            menu_config: MenuConfig {
                menu_info: Vec::new(),
                default_font: "fonts/AaBianYaKai-2.ttf".to_owned(),
                default_text_size: 7f32,
                selecting_color: css::RED.into(),
                deselecting_color: css::WHITE.into(),
            },
        }
    }
}

#[derive(Component, Debug,)]
struct Canvas {
}

fn test_button(mut query: Query<&Button,>) {
 //   println!("test button");
}

fn bottom_tools_clicked(mut query: Query<
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

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins.set( {
        let mut ass = AssetPlugin::default(); ass.file_path = "./assets".to_owned(); ass 
    }), PanCamPlugin))
        .insert_resource(AppConfig::default())
 //       .insert_resource(ClearColor(css::YELLOW.into()))
        .add_systems(Startup, (init, setup, generates_ui).chain())
        .add_systems(Update, test_button)
        .add_systems(Update, bottom_tools_clicked)
    ;
    
    app.run();
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


fn generates_ui(mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut app_config: ResMut<AppConfig>,
) {
    commands.spawn(Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::FlexStart,

        padding: UiRect::all(Val::Percent(1.)),
        ..default()
    })
    .with_children(|b| 
        build_top_menu(&app_config.menu_config, asset_server.borrow_mut(), b))
    .with_child(
Node {
            width: Val::Percent(100.),
            height: Val::Percent(
                100. - app_config.default_top_menu_percentage 
                - app_config.default_bottom_menu_percentage
            ), // naturally 90..
            ..default()
        }
    )
    .with_children(|b| 
        build_bottom_menu(&mut app_config.tools_config, asset_server.borrow_mut(), b));

}


fn make_canvas_image(app_config: &AppConfig) -> Image {

    let mut image = Image::new_fill(
        app_config.default_canvas_size.clone(),
        TextureDimension::D2,
        &app_config.default_clear_color.to_u8_array(),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
    );

    image.sampler = ImageSampler::nearest();
    image

}

fn init(mut app_config: ResMut<AppConfig>,
    images: ResMut<Assets<Image>>,
    assets_server: ResMut<AssetServer>) {
    init_lang(None, Some("assets/languages/".to_owned()));

    for x in &mut app_config.tools_config.tools_info {
        x.icon_handle = Some(assets_server.load(&x.icon));
    }

    for x in &mut app_config.menu_config.menu_info {
        x.icon_handle = Some(assets_server.load(&x.icon));
    }
    
}
fn setup(mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    app_config: Res<AppConfig>, ) {

    commands.spawn(PanCam::default());
 
    let canvas_image = images.add(make_canvas_image(app_config.as_ref()));

    commands.spawn((
            Canvas {},
            Sprite::from_image(canvas_image),
    ));
}

