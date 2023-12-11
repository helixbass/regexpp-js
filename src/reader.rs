use crate::Wtf16;

pub type CodePoint = u32;

fn is_surrogate_code_point(value: u16) -> bool {
    (0xd800..=0xdfff).contains(&value)
}

fn get_single_surrogate_pair_code_point(values: &[u16]) -> CodePoint {
    let mut iterator = char::decode_utf16(values.into_iter().copied());
    let first_char = iterator
        .next()
        .expect("Should've gotten at least one char")
        .expect("Expected valid surrogate pair");
    assert!(iterator.next().is_none(), "Expected only one char");
    let first_code_point: CodePoint = first_char.into();
    assert!(first_code_point > 0xffff);
    first_code_point
}

pub struct Reader {
    _use_unicode_impl: bool,
    _s: Wtf16,
    _i: usize,
    _start: usize,
    _end: usize,
    _cp1: Option<CodePoint>,
    _w1: usize,
    _cp2: Option<CodePoint>,
    _w2: usize,
    _cp3: Option<CodePoint>,
    _w3: usize,
    _cp4: Option<CodePoint>,
}

impl Default for Reader {
    fn default() -> Self {
        Self {
            _use_unicode_impl: Default::default(),
            _s: Default::default(),
            _i: Default::default(),
            _start: Default::default(),
            _end: Default::default(),
            _cp1: Default::default(),
            _w1: 1,
            _cp2: Default::default(),
            _w2: 1,
            _cp3: Default::default(),
            _w3: 1,
            _cp4: Default::default(),
        }
    }
}

impl Reader {
    fn at(&self, i: usize) -> Option<CodePoint> {
        (i < self._end).then(|| {
            let possibly_first_half_of_surrogate_pair = self._s[i];
            if !self._use_unicode_impl || !is_surrogate_code_point(possibly_first_half_of_surrogate_pair) {
                return possibly_first_half_of_surrogate_pair.into();
            }
            get_single_surrogate_pair_code_point(&self._s[i..=i + 1])
        })
    }

    fn width(&self, c: Option<CodePoint>) -> usize {
        match c {
            Some(c) if c > 0xffff => 2,
            _ => 1,
        }
    }

    pub fn index(&self) -> usize {
        self._i
    }

    pub fn current_code_point(&self) -> Option<CodePoint> {
        self._cp1
    }

    pub fn next_code_point(&self) -> Option<CodePoint> {
        self._cp2
    }

    pub fn next_code_point2(&self) -> Option<CodePoint> {
        self._cp3
    }

    pub fn next_code_point3(&self) -> Option<CodePoint> {
        self._cp4
    }

    pub fn reset(&mut self, source: &[u16], start: usize, end: usize, u_flag: bool) {
        self._use_unicode_impl = u_flag;
        self._start = start;
        self._s = source.into();
        self._end = end;
        self.rewind(start);
    }

    pub fn rewind(&mut self, index: usize) {
        assert!(
            index >= self._start,
            "Not expecting to rewind past initial start"
        );
        self._i = index;
        self._cp1 = self.at(index);
        self._w1 = self.width(self._cp1);
        self._cp2 = self.at(index + self._w1);
        self._w2 = self.width(self._cp2);
        self._cp3 = self.at(index + self._w1 + self._w2);
        self._w3 = self.width(self._cp3);
        self._cp4 = self.at(index + self._w1 + self._w2 + self._w3);
    }

    pub fn advance(&mut self) {
        if self._cp1.is_some() {
            self._i += self._w1;
            self._cp1 = self._cp2;
            self._w1 = self._w2;
            self._cp2 = self._cp3;
            self._w2 = self.width(self._cp2);
            self._cp3 = self._cp4;
            self._w3 = self.width(self._cp3);
            self._cp4 = self.at(self._i + self._w1 + self._w2 + self._w3);
        }
    }

    pub fn eat(&mut self, cp: CodePoint) -> bool {
        if self._cp1 == Some(cp) {
            self.advance();
            return true;
        }
        false
    }

    pub fn eat2(&mut self, cp1: CodePoint, cp2: CodePoint) -> bool {
        if self._cp1 == Some(cp1) && self._cp2 == Some(cp2) {
            self.advance();
            self.advance();
            return true;
        }
        false
    }

    pub fn eat3(&mut self, cp1: CodePoint, cp2: CodePoint, cp3: CodePoint) -> bool {
        if self._cp1 == Some(cp1) && self._cp2 == Some(cp2)  && self._cp3 == Some(cp3) {
            self.advance();
            self.advance();
            self.advance();
            return true;
        }
        false
    }
}
