use std::fmt;
use std::sync::RwLock;
use bevy::prelude::*;

pub(crate) struct MaskGeneratingFunc<'a> {
    pub name: String,
    pub fun: Box<dyn Fn(u32, u32, &UVec2) -> f32 + 'a + Send + Sync>,
}

static NAME_NUMBER: RwLock<u32> = RwLock::new(0);

impl <'a> MaskGeneratingFunc<'a> {
    pub fn new(name: Option<String>, f: impl Fn(u32, u32, &UVec2) -> f32 + 'a + Send + Sync) -> Self {
        let num = format!("{}", 
            match NAME_NUMBER.write() {
                Ok(mut o) => {
                    let old = o.clone();
                    *o += 1;
                    old
                },
                Err(e) => {
                    panic!("number lock failed: {}.", e) 
                }
            });
            
        let name = name.unwrap_or("UnnamedMaskGeneratingFunc".to_owned() + &num);
        Self {
            name,
            fun: Box::new(f)
        }
    }
}

impl fmt::Debug for MaskGeneratingFunc<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
        f.debug_struct("MaskGeneratingFunc")
            .field("name", &self.name)
            .finish()
    }
}
