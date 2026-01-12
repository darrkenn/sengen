use genetica::individual::Generate;

use crate::words::Word;

pub struct GeneType<'a> {
    pub word: Box<dyn Word + 'a>,
}

impl<'a> Generate for GeneType<'a> {
    fn generate() -> Self {}
}
