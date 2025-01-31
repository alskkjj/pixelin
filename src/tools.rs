use bevy::prelude::*;
use image::RgbaImage;

use crate::mix_methods::MixMethod;
use crate::patterns::Pattern;

use std::rc::Rc;

pub trait PointTool {
    fn apply(&self, image: &mut RgbaImage, relative_loc: Vec2);
}

pub(crate) struct Brush {
    pattern: Rc<Pattern>,
    mix_method: MixMethod,
    mix_width: u8,
}

impl PointTool for Brush {
    fn apply(&self, image: &mut RgbaImage, relative_loc: Vec2) {
        let image_size = image.dimensions();
        let image_rect = Rect::new(0f32, 0., image_size.0 as f32, image_size.1 as f32);

        let pattern_size = self.pattern.size();
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
                    let pixel = image.get_pixel_mut_checked(i, j)
                        .expect("<Brush as PointTool>::apply: coordinate calculated error");
                    let srgba = self.pattern.get_color_at(UVec2::new(i - top_left.0, j - top_left.1))
                        .expect("<Brush as PointTool>::apply: coordinate calculated error");
                    let u8s = &srgba.to_u8_array();
                    let np = self.mix_method.perform_operation_4(&pixel.0, u8s);
                    pixel.0 = np;
                } else if self.mix_width == 3 {
                    let pixel = image.get_pixel_mut_checked(i, j)
                        .expect("<Brush as PointTool>::apply: coordinate calculated error");
                    let srgba = self.pattern.get_color_at(UVec2::new(i - top_left.0, j - top_left.1))
                        .expect("<Brush as PointTool>::apply: coordinate calculated error");
                    let p0: &[u8; 4] = &pixel.0;
                    let iprgba: &[u8; 3] = &p0[0..3].try_into().expect("Cast to shorter array failed.");
                    let srgba_u8 = srgba.to_u8_array_no_alpha();

                    let np = self.mix_method.perform_operation_3(iprgba, &srgba_u8);
                    {
                        let a = &mut (pixel.0[0..3])
                            .try_into()
                            .expect("Cast to shorter array failed.");
                        *a = np;
                    }
                }
            }
        }
    }
}
