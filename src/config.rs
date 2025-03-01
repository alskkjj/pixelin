use bevy::{
    prelude::*,
    color::palettes::css,
};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use bevy::render::render_resource::{Extent3d, };

use crate::{
    pressure_mask::MaskGeneratingFunc,
    mix_methods::MixMethod,
    patterns::PatternGeneratingFunc,
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
pub(crate) struct ToolsConfig<'a, 'b> {
    pub tools_info: Vec<ToolInfo>,
    pub default_font: String,
    pub default_text_size: f32,

    pub selecting_color: Arc<RwLock<Srgba>>,
    pub deselecting_color: Arc<RwLock<Srgba>>,

    pub exclusive_tools: HashMap<String, Entity>,
    pub current_tool: Option<String>,
    
    pub mix_method: MixMethod,
    pub pressure_mask: MaskGeneratingFunc<'a>,
    pub pattern: PatternGeneratingFunc<'b>,
}


#[derive(Debug)]
pub(crate) struct MenuConfig {
    pub menu_info: Vec<MenuInfo>,
    pub default_font: String,
    pub default_text_size: f32,
}

#[derive(Debug, Resource)]
pub(crate) struct AppConfig<'a, 'b> {
    pub default_canvas_size: Extent3d,
    pub default_clear_color: Srgba,

    pub default_top_menu_percentage: f32,
    pub default_bottom_menu_percentage: f32,

    pub tools_config: ToolsConfig<'a, 'b>,
    pub menu_config: MenuConfig,
}

impl <'a, 'b> Default for AppConfig<'a, 'b> {
    fn default() -> Self {
        let selecting_color1 = Arc::new(RwLock::new(css::RED.into()));
        let deselecting_color1 = Arc::new(RwLock::new(css::WHITE.into()));
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
                selecting_color: selecting_color1.clone(),
                deselecting_color: deselecting_color1.clone(),
                exclusive_tools: HashMap::new(),
                current_tool: None,

                mix_method: MixMethod::Normal,
                pressure_mask: MaskGeneratingFunc {
                    name: "overwrite".to_owned(),
                    fun: Box::new(|_x, _y, _size| { 1.0 })
                },
                pattern: PatternGeneratingFunc {
                    name: "dot".to_owned(),
                    fun: Box::new(move |_x, _y, _sz| {
                        selecting_color1.read()
                            .expect("read lock failed.")
                            .clone()
                    })
                }
            },
            menu_config: MenuConfig {
                menu_info: Vec::new(),
                default_font: "fonts/AaBianYaKai-2.ttf".to_owned(),
                default_text_size: 7f32,
            },
        }
    }
}


