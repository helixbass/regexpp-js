use crate::CodePoint;

pub fn is_id_start(cp: CodePoint) -> bool {
    if cp < 0x41 {
        return false;
    }
    if cp < 0x5b {
        return true;
    }
    if cp < 0x61 {
        return false;
    }
    if cp < 0x7b {
        return true;
    }
    is_large_id_start(cp)
}

fn is_large_id_start(cp: CodePoint) -> bool {
    unimplemented!()
}
