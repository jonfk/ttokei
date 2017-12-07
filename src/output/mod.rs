
pub mod text;

use tokei::{Languages, LanguageType};

pub trait Outputter {
    fn output(&self, input: Languages);
}
