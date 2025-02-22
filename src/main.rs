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
    asset::RenderAssetUsages,
    color::palettes::{css, tailwind},
    image::ImageSampler, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat, },
    ui::widget::NodeImageMode,
    log::once,
};

use std::borrow::BorrowMut;
use std::collections::HashMap;

use crate::config::AppConfig;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins.set( {
        let mut ass = AssetPlugin::default(); ass.file_path = "./assets".to_owned(); ass 
    }), PanCamPlugin))
        .insert_resource(AppConfig::default())
        .add_systems(Startup, (init, setup, generates_ui).chain())
        .add_systems(Update, bottom_tools_clicked);
    
    app.run();
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

