use bevy::winit::WinitSettings;
use my_fluent_rs_helper::*;

mod mix_methods;
mod canvas;
mod tools;
mod patterns;
mod pressure_mask;
mod tools_bar;
mod config;
mod group_checker;
mod menu_bar;

use bevy_pancam::*;
use bevy::{
    prelude::*,
 //  color::palettes::{css, tailwind},
 //   render::render_resource::{Extent3d, TextureDimension, TextureFormat, },
};

use std::borrow::BorrowMut;

#[bevy_main]
fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins.set( {
        let mut ass = AssetPlugin::default(); ass.file_path = "./assets".to_owned(); ass
    })
            .set(bevy::log::LogPlugin {
                level: bevy::log::Level::DEBUG,
                ..default()
            }), PanCamPlugin, ))
    .insert_resource(config::AppConfig::default())
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, (init, setup, generates_ui).chain())
        //       .add_systems(Update, crate::tools_bar::bottom_tools_clicked);
        ;

    tools_bar::init_me(&mut app);
    app.run();
}

fn generates_ui(mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut app_config: ResMut<config::AppConfig<'static, 'static>>,
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
        menu_bar::build_menu_bar(&app_config.menu_config, asset_server.borrow_mut(), b))
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
        tools_bar::build_tools_bar(&mut app_config.tools_config, asset_server.borrow_mut(), b));

}



fn init(mut app_config: ResMut<config::AppConfig<'static, 'static>>,
 //   images: ResMut<Assets<Image>>,
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
    app_config: Res<config::AppConfig<'static, 'static>>, ) {

    commands.spawn(PanCam::default());

    let canvas_image = images.add(canvas::make_canvas_image_by_config(&app_config));

    commands.spawn((
            canvas::Canvas {},
            Sprite::from_image(canvas_image),
    ));
}

