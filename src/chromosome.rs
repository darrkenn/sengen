use crate::words::Word;

pub struct GeneType<'a> {
    pub word: Box<dyn Word + 'a>,
}


