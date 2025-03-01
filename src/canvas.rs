use bevy::prelude::*;
use bevy::render::render_resource::{TextureDimension, TextureFormat, };
use bevy::asset::RenderAssetUsages;
use bevy::image::ImageSampler;

use crate::config::AppConfig;

pub(crate) fn make_canvas_image_by_config(app_config: &AppConfig) -> Image {

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

#[derive(Component, Debug,)]
pub(crate) struct Canvas {
}


