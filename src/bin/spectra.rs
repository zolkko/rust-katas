use std::ops::Sub;
use std::collections::HashMap;
use std::hash::Hash;


trait Abs {
    fn abs(&self) -> Self;
}

impl Abs for f64 {
    fn abs(&self) -> Self {
        self.abs()
    }
}


type Count = usize;


fn find_spectra<T>(first_set: Vec<T>, second_set: Vec<T>) -> Option<(usize, T)>
where
    T: PartialOrd + Copy + Abs + Sub + Eq + Hash,
{
    let mut maybe_result: Option<(usize, T)> = None;

    let mut acc = HashMap::new();
    for s1 in first_set {
        for s2 in second_set {
            let val = *s1 - *s2;
            let cnt = acc.entry(val).or_insert(0);
            *cnt += 1;

            if let Some((_, cur_val)) = maybe_result {
                if cur_val < val {
                    maybe_result = Some((*cnt, val.abs()))
                }
            } else {
                maybe_result = Some((*cnt, val.abs()));
            }
        }
    }

    maybe_result
}


fn main() {

}