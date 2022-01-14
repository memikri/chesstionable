macro_rules! impl_index {
    ($name:ident($count:expr)) => {
        impl $name {
            pub fn index(self) -> usize {
                self as usize
            }

            #[inline]
            pub fn from_index(index: usize) -> Option<$name> {
                if index < $count {
                    Some(unsafe { transmute(index as u8) })
                } else {
                    None
                }
            }
        }
    };
}

pub(crate) use impl_index;
