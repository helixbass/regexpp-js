pub type CodePoint = u32;

fn is_surrogate_code_point(value: u16) -> bool {
    (0xd800..=0xdfff).contains(&value)
}

fn get_single_surrogate_pair_code_point(values: &[u16]) -> CodePoint {
    let mut iterator = char::decode_utf16(values.into_iter().copied());
    let first_char = iterator.next().expect("Should've gotten at least one char").expect("Expected valid surrogate pair");
    assert!(iterator.next().is_none(), "Expected only one char");
    let first_code_point: CodePoint = first_char.into();
    assert!(first_code_point > 0xffff);
    first_code_point
}

pub struct Reader {
    _use_unicode_impl: bool,
    _s_between_start_and_end: Vec<u16>,
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
            _s_between_start_and_end: Default::default(),
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
            let index = self.passed_index_to_internal_index(i);
            let possibly_first_half_of_surrogate_pair = self._s_between_start_and_end[index];
            if !is_surrogate_code_point(possibly_first_half_of_surrogate_pair) {
                return possibly_first_half_of_surrogate_pair.into();
            }
            get_single_surrogate_pair_code_point(&self._s_between_start_and_end[index..=index + 1])
        })
    }

    fn passed_index_to_internal_index(&self, index: usize) -> usize {
        assert!(index >= self._start && index < self._end);
        index - self._start
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

    pub fn reset(&mut self, source: &str, start: usize, end: usize, u_flag: bool) {
        self._use_unicode_impl = u_flag;
        self._start = start;
        self._s_between_start_and_end = source[start..end].encode_utf16().collect();
        self._end = end;
        self.rewind(start);
    }

    pub fn rewind(&mut self, index: usize) {
        assert!(index >= self._start, "Not expecting to rewind past initial start");
        self._i = index;
        self._cp1 = self.at(index);
        self._w1 = self.width(self._cp1);
        self._cp2 = self.at(index + self._w1);
        self._w2 = self.width(self._cp2);
        self._cp3 = self.at(index + self._w1 + self._w2);
        self._w3 = self.width(self._cp3);
        self._cp4 = self.at(index + self._w1 + self._w2 + self._w3);
    }
}
