use std::ops::Range;

pub struct ObjectIdTable {
    ids: Vec<Range<u64>>,
    first_free_id: usize,
}

impl ObjectIdTable {
    pub const fn new() -> ObjectIdTable {
        ObjectIdTable {
            ids: Vec::new(),
            first_free_id: 0,
        }
    }

    /*pub fn init(&mut self) {
        self.ids.push(0..0)
    }*/

    pub fn take_new_id(&mut self) -> u64 {
        if self.ids.len() == 0 {
            self.ids.push(0..1);
            return 0;
        }
        let range = &mut self.ids[0];
        if range.start > 1 {
            self.ids.insert(0, 0..1);
            return 0;
        } else if range.start == 1 {
            range.start = 0;
            return 0;
        }
        let new = range.end;
        range.end += 1;
        let end = new; // because borrow checker complains

        // check if merging needed:
        if self.ids.len() > 1 && end == self.ids[1].start {
            let next_end = self.ids.remove(1).end;
            //range.end = next_end
            self.ids[0].end = next_end // because borrow checker complains
        }
        new
    }

    pub fn release_id(&mut self, id: u64) {
        for mut i in 0..self.ids.len() {
            let r = &mut self.ids[i];
            if id >= r.start && id < r.end {
                if r.end - r.start > 1 {
                    if id == r.start {
                        r.start += 1;
                        return;
                    } else if id == r.end - 1 {
                        r.end -= 1;
                        return;
                    }
                    let mut r2 = r.clone();
                    r2.start = id + 1;
                    r.end = id;
                    self.ids.insert(i + 1, r2);
                } else {
                    self.ids.remove(i);
                    return;
                }
            }
        }
    }
}
