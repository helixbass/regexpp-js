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

pub fn is_id_continue(cp: CodePoint) -> bool {
    if cp < 0x30 {
        return false;
    }
    if cp < 0x3a {
        return true;
    }
    if cp < 0x41 {
        return false;
    }
    if cp < 0x5b {
        return true;
    }
    if cp == 0x5f {
        return true;
    }
    if cp < 0x61 {
        return false;
    }
    if cp < 0x7b {
        return true;
    }
    is_large_id_start(cp) || is_large_id_continue(cp)
}

fn is_large_id_start(cp: CodePoint) -> bool {
    unimplemented!()
}

fn is_large_id_continue(cp: CodePoint) -> bool {
    unimplemented!()
}
