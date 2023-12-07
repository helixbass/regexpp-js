#[derive(Default)]
pub struct Reader;

impl Reader {
    pub fn index(&self) -> usize {
        unimplemented!()
    }

    pub fn reset(
        &mut self,
        source: &str,
        start: usize,
        end: usize,
        u_flag: bool,
    ) {
        unimplemented!()
    }

    pub fn rewind(
        &mut self,
        index: usize,
    ) {
        unimplemented!()
    }
}
