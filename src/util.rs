#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Id(pub(crate) u32);

impl Id {
    pub(crate) const START: Self = Self(2166136261);

    pub(crate) fn hash(&mut self, val: impl SimpleHash) { val.hash(self) }
}

pub(crate) trait SimpleHash {
    fn hash(self, id: &mut Id);
}

impl SimpleHash for u32 {
    fn hash(self, id: &mut Id) {
        id.0 ^= self;
        id.0 = id.0.wrapping_mul(16777619);
    }
}

impl<'a> SimpleHash for &'a [u8] {
    fn hash(self, id: &mut Id) {
        for byte in self {
            (*byte as u32).hash(id)
        }
    }
}

impl<'a> SimpleHash for &'a str {
    fn hash(self, id: &mut Id) { self.as_bytes().hash(id) }
}
