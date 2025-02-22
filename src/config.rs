use bevy::{
    prelude::*,
    color::palettes::css,
};
use std::collections::HashMap;
use bevy::render::render_resource::{Extent3d, };

use crate::{
    pressure_mask::PressureMask,
    mix_methods::MixMethod,
    patterns::Pattern,
};



#[derive(Debug)]
pub(crate) struct ToolInfo {
    pub name: String, // name to translate.
    pub icon: String, // path to icon in assets.
    pub icon_handle: Option<Handle<Image>>,
}

#[derive(Debug)]
pub(crate) struct MenuInfo {
    pub name: String,
    pub icon: String,
    pub icon_handle: Option<Handle<Image>>,
}

                
#[derive(Debug)]
pub(crate) struct ToolsConfig {
    pub tools_info: Vec<ToolInfo>,
    pub default_font: String,
    pub default_text_size: f32,

    pub selecting_color: Srgba,
    pub deselecting_color: Srgba,

    pub exclusive_tools: HashMap<String, Entity>,
    pub current_tool: Option<String>,
    
    pub mix_method: MixMethod,
    pub pressure_mask: PressureMask,
    pub pattern: Pattern,
}


#[derive(Debug)]
pub(crate) struct MenuConfig {
    pub menu_info: Vec<MenuInfo>,
    pub default_font: String,
    pub default_text_size: f32,

    pub selecting_color: Srgba,
    pub deselecting_color: Srgba,
}

#[derive(Debug, Resource)]
pub(crate) struct AppConfig {
    pub default_canvas_size: Extent3d,
    pub default_clear_color: Srgba,

    pub default_top_menu_percentage: f32,
    pub default_bottom_menu_percentage: f32,

    pub tools_config: ToolsConfig,
    pub menu_config: MenuConfig,
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


