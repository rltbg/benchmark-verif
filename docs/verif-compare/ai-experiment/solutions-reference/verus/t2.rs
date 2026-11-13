use vstd::prelude::*;
verus! {
fn search(v: &[u64], key: u64) -> (r: Option<usize>)
    requires forall|i: int, j: int| 0 <= i <= j < v@.len() ==> v@[i] <= v@[j],
    ensures
        match r {
            Some(i) => i < v@.len() && v@[i as int] == key,
            None => forall|i: int| 0 <= i < v@.len() ==> v@[i] != key,
        },
{
    let mut lo: usize = 0;
    let mut hi: usize = v.len();
    while lo < hi
        invariant
            lo <= hi <= v@.len(),
            forall|i: int, j: int| 0 <= i <= j < v@.len() ==> v@[i] <= v@[j],
            forall|i: int| 0 <= i < lo ==> v@[i] < key,
            forall|i: int| hi <= i < v@.len() ==> v@[i] > key,
        decreases hi - lo,
    {
        let mid = lo + (hi - lo) / 2;
        if v[mid] < key {
            lo = mid + 1;
        } else if v[mid] > key {
            hi = mid;
        } else {
            return Some(mid);
        }
    }
    None
}
fn main() {}
}
