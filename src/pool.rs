use super::Id;

#[derive(Default, Copy, Clone)]
struct PoolItem {
    pub id: Id,
    pub last_update: usize,
}

#[derive(Copy, Clone)]
pub struct Pool<const N: usize> {
    vec: [PoolItem; N],
}

impl<const N: usize> Pool<N> {
    pub fn alloc(&mut self, id: Id, frame: usize) -> usize {
        let mut res = None;
        let mut latest_update = frame;
        for i in 0..N {
            if self.vec[i].last_update < latest_update {
                latest_update = self.vec[i].last_update;
                res = Some(i);
            }
        }

        assert!(res.is_some());
        self.vec[res.unwrap()].id = id;
        self.update(res.unwrap(), frame);
        res.unwrap()
    }

    pub fn get(&self, id: Id) -> Option<usize> { (0..N).find(|&i| self.vec[i].id == id) }

    pub fn update(&mut self, idx: usize, frame: usize) { self.vec[idx].last_update = frame; }

    pub fn reset(&mut self, idx: usize) { self.vec[idx] = PoolItem::default(); }
}

impl<const N: usize> Default for Pool<N> {
    fn default() -> Self { Self { vec: [PoolItem::default(); N] } }
}
