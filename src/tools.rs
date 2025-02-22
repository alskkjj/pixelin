use bevy::prelude::*;
use image::RgbaImage;
use std::sync::Arc;

use crate::patterns::PatternGeneratingFunc;
use crate::pressure_mask::MaskGeneratingFunc;
use crate::mix_methods::MixMethod;

fn brush_mix4(
    mask_generating_func: &MaskGeneratingFunc,
    pattern_generating_func: &PatternGeneratingFunc,
    origin: &Srgba,
    mix_method: &MixMethod,
    loc: &UVec2,
    size: &UVec2,
    ) -> Srgba {
    let mask = (mask_generating_func.fun)(loc.x, loc.y, size);
    let pattern = (pattern_generating_func.fun)(loc.x, loc.y, size);
    let masked_pattern = mask * pattern;

    Srgba::from_u8_array(mix_method.perform_operation_4(
            &masked_pattern.to_u8_array(), &origin.to_u8_array()))
}

fn brush_mix3(
    mask_generating_func: &MaskGeneratingFunc,
    pattern_generating_func: &PatternGeneratingFunc,
    origin: &Srgba,
    mix_method: &MixMethod,
    loc: &UVec2,
    size: &UVec2,
    ) -> Srgba {
    let mask = (mask_generating_func.fun)(loc.x, loc.y, size);
    let pattern = (pattern_generating_func.fun)(loc.x, loc.y, size);
    let masked_pattern = mask * pattern;

    Srgba::from_u8_array_no_alpha(mix_method.perform_operation_3(
            &masked_pattern.to_u8_array_no_alpha(), &origin.to_u8_array_no_alpha()))
}

#[derive(Component, Debug)]
pub(crate) struct BottomTools {
    pub tool_name: String,
}

#[derive(Component, Debug)]
struct Bucket;

pub trait PointTool {
    fn apply(&self, image: &mut RgbaImage, relative_loc: Vec2);
}

pub(crate) struct Brush<'a> {
    mask_generating_func: MaskGeneratingFunc<'a>,
    pattern_generating_func: PatternGeneratingFunc<'a>,
    size: Arc<RwLock<UVec2>>,
    mix_method: MixMethod,
    mix_width: u8,
}

impl <'a> PointTool for Brush<'a> {
    fn apply(&self, image: &mut RgbaImage, relative_loc: Vec2) {
        let image_size = image.dimensions();
        let image_rect = Rect::new(0f32, 0., image_size.0 as f32, image_size.1 as f32);

        let pattern_size = *self.size.read().expect("get pattern size failed.");
        let half_pattern_size = pattern_size / 2;
        let f = half_pattern_size.as_vec2();
        let pattern_rect = Rect::from_corners(relative_loc - f, relative_loc + f);

        let overlap = image_rect.intersect(pattern_rect);
        if overlap.is_empty() { return; }
        let top_left = (overlap.min.x.ceil() as u32, overlap.min.y.ceil() as u32);
        let bottom_right = (overlap.max.x.floor() as u32, overlap.max.y.floor() as u32);

        for i in top_left.0..bottom_right.0 {
            for j in top_left.1..bottom_right.1 {
                if self.mix_width == 4 {
                    let pixel0 = image.get_pixel_mut_checked(i, j)
                        .expect("<Brush as PointTool>::apply: coordinate calculated error");
                    let pixel = Srgba::from_u8_array(pixel0.0);
                    let srgba = 
                        brush_mix4(&self.mask_generating_func, 
                            &self.pattern_generating_func, &pixel, &self.mix_method, 
                            &UVec2::new(i - top_left.0, j - top_left.1), &pattern_size); 

                    pixel0.0 = srgba.to_u8_array();
                } else if self.mix_width == 3 {
                    let pixel0 = image.get_pixel_mut_checked(i, j)
                        .expect("<Brush as PointTool>::apply: coordinate calculated error");
                    let pixel = Srgba::from_u8_array(pixel0.0);

                    let srgba = 
                        brush_mix3(&self.mask_generating_func, 
                            &self.pattern_generating_func, &pixel, &self.mix_method, 
                            &UVec2::new(i - top_left.0, j - top_left.1), &pattern_size); 

                    pixel0.0 = srgba.to_u8_array();
                }
            }
        }
    }
}
