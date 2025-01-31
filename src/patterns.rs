use std::collections::HashMap;
use std::cell::LazyCell;
use std::rc::Rc;

use std::f64::consts::{SQRT_2, FRAC_1_SQRT_2};
use bevy::color::palettes::css;
use bevy::prelude::*;
use image::RgbaImage;

#[derive(Clone, )]
pub(crate) struct SimpleColorPattern {
    pub size: UVec2, // omit the z axis for now.
    pub pattern: Vec<u8>, // has color or not.
    pub default_color: Srgba, 
    pub color_map: ColorMap,
}

#[derive(Clone, )]
pub(crate) enum ColorMap {
    Image(RgbaImage),
    CoordColor(HashMap<UVec2, Srgba>),
    Color(Srgba),
}

fn coord_to_index(coord: UVec2, width: u32) -> usize {
    (coord.x * width + coord.y) as usize
}

fn coord_in_size(coord: UVec2, size: UVec2) -> bool {
    coord.x < size.x && coord.y < size.y
}

impl SimpleColorPattern {

    pub fn center(&self) -> UVec2 {
        self.size / 2
    }
    pub fn get_color_at(&self, coord: UVec2) -> Option<Srgba> {
        if !coord_in_size(coord, self.size) { return None; }
        let index = coord_to_index(coord, self.size.x);
        let re = self.pattern.get(index).expect("SimpleColorPattern::get_color_at: ");
        if *re == 0 {
            Some(self.default_color)
        } else {
            match &self.color_map {
                ColorMap::Image(image) => {
                    match image.get_pixel_checked(coord.x, coord.y) {
                        None => { None },
                        Some(ic) => {
                            Some(Srgba::from_u8_array(ic.0))
                        }
                    }
                        
                },
                ColorMap::CoordColor(cc) => { cc.get(&coord).cloned() },
                ColorMap::Color(cl) => { Some(cl.clone()) }
            }
        }
    }
    
    fn map_from_centralized_coord(&self, loc: UVec2) -> UVec2 {
        let center = self.center();
        loc + center
    }

}

impl Default for SimpleColorPattern {
    fn default() -> Self {
        Self {
            size: UVec2::splat(1),
            pattern: vec![1u8],
            default_color: css::BLACK,
            color_map: ColorMap::Color(css::BLACK.into()),
        }
    }
}


pub(crate) enum Pattern {
    SimpleColor(SimpleColorPattern),
}

impl Pattern {
    pub fn center(&self) -> UVec2 {
        match self {
            Pattern::SimpleColor(c) => {
                c.center()
            }
        }
    }
    
    pub fn size(&self) -> UVec2 {
        match self {
            Pattern::SimpleColor(sc) => { sc.size }
        }
    }

    pub fn get_color_at(&self, loc: UVec2) -> Option<Srgba> {
        match self {
            Self::SimpleColor(sc) => {
                if loc.x > sc.size.x || loc.y > sc.size.y { return None; }
                sc.get_color_at(loc)
            }
        }
    }
}

fn quartar_cake_generate_use_default<T, F>(radius: f64, f: F) -> Vec<T> 
    where T: Default + Copy,
          F: Fn(usize, usize) -> T
{
    quartar_cake_generate(radius, T::default(), f)
}

fn quartar_cake_generate<T, F>(radius: f64, defau: T, f: F) -> Vec<T> 
    where T: Copy,
        F: Fn(usize, usize) -> T
{
    if f64::abs(radius - 1.0f64) < 10E-6 { return vec![f(0, 0)]; }
    if f64::abs(radius) < 10E-6 { return vec![]; }
    let r_radius = SQRT_2 * (FRAC_1_SQRT_2 * radius).floor();
    let w = radius.ceil() as usize;
    let mut ret = Vec::from_iter((0..w*w).map(|_a| {defau} ));

    for i in 0..w {
        for j in 0..w {
            let (dx, dy) = (i as f64, j as f64);
            let d = f64::sqrt(dx * dx + dy * dy);

            if d < r_radius {
                let (idx, jdx) = (i, j);
                ret[idx*w + jdx] = f(i, j);
            } else { break; }
        }
    }
    ret
}


fn sub_helper(i: usize, j: usize, w: usize) -> (isize, isize) {
    let i = i as isize;
    let j = j as isize;
    let w = w as isize;
    return (i - w, j - w);
}

fn cake_generate<T, F>(radius: f64, defau: T, f: F) -> Vec<Vec<T>> 
    where T: Copy,
          F: Fn(f64, f64) -> T
{
    if f64::abs(radius - 1.0f64) < 10E-6 { return vec![vec![f(0., 0.)]]; }
    if f64::abs(radius) < 10E-6 { return vec![]; }
    let r_dius = SQRT_2 * (FRAC_1_SQRT_2 * radius).floor();
    let w = radius.ceil() as usize;
    let mut ret = Vec::from_iter(
        (0..(2*w)).map(|_a| Vec::from_iter((0..(2*w)).map(|_a| {defau}))));

    for i in 0..ret.len() {
        for j in 0..ret[i].len() {
            let (cx, cy) = sub_helper(i, j, w);
            let (dx, dy) = (cx as f64, cy as f64);
            let d = f64::sqrt(dx*dx + dy*dy);
            
            if d < r_dius {
                ret[i][j] = f(dx, dy);
            } 
        }
    }

    ret
}

// The examples of pattern
pub(crate) const PENCIL: LazyCell<Rc<Pattern>> = LazyCell::new(|| Rc::new(Pattern::SimpleColor(SimpleColorPattern::default())));
pub(crate) const DOT_BRUSH6: LazyCell<Rc<Pattern>> = LazyCell::new(|| { Rc::new({
    let v = cake_generate(6.0, 0u8, |_, _| 1u8);
    let size = UVec2::new(v.len() as u32, if v.len() != 0 {v.first().unwrap().len() as u32} else {0u32} );
    let pattern: Vec<u8> = v.iter().flatten().cloned().collect();
    let p = SimpleColorPattern {
        size,
        pattern,
        default_color: Srgba::new(0., 0., 0., 0.),
        color_map: ColorMap::Color(css::BLACK.into()),
    };
    Pattern::SimpleColor(p)
}) });

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_cake_generate() {
        {
            let expected = 123u8;
            let defa = 0u8;
            let v = quartar_cake_generate(1.0f64, defa, 
                |_a, _b| { expected });
            assert_eq!(v.len(), 1);
            assert_eq!(v[0], expected);
        }
        {
            let expected = 123u8;
            let defa = 0u8;
            let v = quartar_cake_generate(2.0f64, defa, 
                |_a, _b| { expected });
            assert_eq!(v.len(), 4);
            assert_eq!(v[0], expected);
            assert_eq!(v[1], expected);
            assert_eq!(v[2], expected);
            assert_eq!(v[3], defa);
        }
        {
            let expected = 123u8;
            let defa = 0u8;
            let v = quartar_cake_generate(3.0f64, defa, 
                |_a, _b| { expected });
            assert_eq!(v.len(), 9);
            assert_eq!(v[0], expected);
            assert_eq!(v[1], expected);
            assert_eq!(v[2], expected);
            assert_eq!(v[3], expected);
        }
    }
}
