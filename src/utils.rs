use std::{ffi::OsStr, iter::once, os::windows::ffi::OsStrExt};

pub trait AsLPSZ {
    fn as_lpsz(&self) -> Vec<u16>;
}

impl<T: AsRef<OsStr> + ?Sized> AsLPSZ for T {
    fn as_lpsz(&self) -> Vec<u16> {
        OsStr::new(self)
            .encode_wide()
            .chain(once(0))
            .chain(once(0))
            .collect()
    }
}

#[macro_export] macro_rules! reg_rule {
    ($type: expr, None, $regex2: expr, $msg: expr) => {
        ($type, None, Some(Regex::new($regex2).unwrap()), $msg)
    };
    ($type: expr, $regex1: expr, None, $msg: expr) => {
        ($type, Some(Regex::new($regex1).unwrap()), None, $msg)
    };
    ($type: expr, $regex1: expr, $regex2: expr, $msg: expr) => {
        (
            $type,
            Some(Regex::new($regex1).unwrap()),
            Some(Regex::new($regex2).unwrap()),
            $msg,
        )
    };
}