use vstd::prelude::*;
verus! {
fn reverse(v: &mut Vec<u32>)
    ensures
        final(v)@.len() == old(v)@.len(),
        forall|i: int| 0 <= i < final(v)@.len() ==> final(v)@[i] == old(v)@[final(v)@.len() - 1 - i],
{
    let n = v.len();
    let mut i: usize = 0;
    while i < n / 2
        invariant
            n == v@.len(), v@.len() == old(v)@.len(), i <= n / 2,
            forall|k: int| 0 <= k < i ==> v@[k] == old(v)@[n - 1 - k],
            forall|k: int| n - i <= k < n ==> v@[k] == old(v)@[n - 1 - k],
            forall|k: int| i <= k < n - i ==> v@[k] == old(v)@[k],
        decreases n / 2 - i,
    {
        let t = v[i];
        v[i] = v[n - 1 - i];
        v[n - 1 - i] = t;
        i += 1;
    }
}
fn main() {}
}
