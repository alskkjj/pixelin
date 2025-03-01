#[derive(Default, Debug, )]
pub(crate) enum MixMethod {
    #[default]
    Normal,
    Average,
    Multiply,
    Lighten,
    Darken,
    Screen,
    Addition,
    Substraction,
    RatioAdd(f32),
}

impl MixMethod {


    pub fn perform_operation_4(&self, a: &[u8; 4], b: &[u8; 4]) -> [u8; 4] {
        match self {
            MixMethod::Normal => { return b.clone(); }
            MixMethod::Average => {
                let mut ret = [0u8; 4];
                for i in 0..a.len() {
                    ret[i] = a[i] / 2 + b[i] / 2;
                }
                return ret;
            },
            MixMethod::Multiply => {
                let mut ret = [0u8; 4];
                for i in 0..a.len() {
                    ret[i] = ((a[i] as f32 / 255. * b[i] as f32 / 255.) * 255.) as u8;
                }
                return ret;
            }
            MixMethod::Lighten => {
                let mut ret = [0u8; 4];
                for i in 0..a.len() {
                    ret[i] = a[i].max(b[i]);
                }
                return ret;
            }
            MixMethod::Darken => {
                let mut ret = [0u8; 4];
                for i in 0..a.len() {
                    ret[i] = a[i].min(b[i]);
                }
                return ret;
            }
            MixMethod::Screen => {
                let mut ret = [0u8; 4];
                for i in 0..a.len() {
                    ret[i] = ((1. - ((1. - (a[0] as f32 / 255.)) * (1. - (b[0] as f32 / 255.)))) * u8::MAX as f32) as u8;
                }
                return ret;
            }
            MixMethod::Addition => {
                let mut ret = [0u8; 4];
                for i in 0..a.len() {
                    ret[i] = (a[0] as u16 + b[0] as u16).clamp(0, u8::MAX as u16) as u8;
                }
                return ret;
            }
            MixMethod::Substraction => {
                let mut ret = [0u8; 4];
                for i in 0..a.len() {
                    ret[i] = (a[0] as i16 - b[0] as i16).clamp(0, u8::MAX as i16) as u8;
                }
                return ret;
            }
            MixMethod::RatioAdd(ratio) => { 
                let mut ret = [0u8; 4];
                for i in 0..a.len() {
                    ret[i] = (a[i] as f32 * (1.-ratio) + b[i] as f32 * ratio).round() as u8;
                }
                return ret;
            },
        }
    }

    pub fn perform_operation_3(&self, a: &[u8; 3], b: &[u8; 3]) -> [u8; 3] {
        match self {
            MixMethod::Normal => { return b.clone(); }
            MixMethod::Average => {
                let mut ret = [0u8; 3];
                for i in 0..a.len() {
                    ret[i] = a[i] / 2 + b[i] / 2;
                }
                return ret;
            },
            MixMethod::Multiply => {
                let mut ret = [0u8; 3];
                for i in 0..a.len() {
                    ret[i] = ((a[i] as f32 / 255. * b[i] as f32 / 255.) * 255.) as u8;
                }
                return ret;
            }
            MixMethod::Lighten => {
                let mut ret = [0u8; 3];
                for i in 0..a.len() {
                    ret[i] = a[i].max(b[i]);
                }
                return ret;
            }
            MixMethod::Darken => {
                let mut ret = [0u8; 3];
                for i in 0..a.len() {
                    ret[i] = a[i].min(b[i]);
                }
                return ret;
            }
            MixMethod::Screen => {
                let mut ret = [0u8; 3];
                for i in 0..a.len() {
                    ret[i] = ((1. - ((1. - (a[0] as f32 / 255.)) * (1. - (b[0] as f32 / 255.)))) * u8::MAX as f32) as u8;
                }
                return ret;
            }
            MixMethod::Addition => {
                let mut ret = [0u8; 3];
                for i in 0..a.len() {
                    ret[i] = (a[0] as u16 + b[0] as u16).clamp(0, u8::MAX as u16) as u8;
                }
                return ret;
            }
            MixMethod::Substraction => {
                let mut ret = [0u8; 3];
                for i in 0..a.len() {
                    ret[i] = (a[0] as i16 - b[0] as i16).clamp(0, u8::MAX as i16) as u8;
                }
                return ret;
            }
            MixMethod::RatioAdd(ratio) => { 
                let mut ret = [0u8; 3];
                for i in 0..a.len() {
                    ret[i] = (a[i] as f32 * (1.-ratio) + b[i] as f32 * ratio).round() as u8;
                }
                return ret;
            },
        }
    }
}

