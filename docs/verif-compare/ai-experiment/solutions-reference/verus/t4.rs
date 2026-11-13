use vstd::prelude::*;
verus! {
broadcast use vstd::seq_lib::group_to_multiset_ensures;

pub open spec fn sorted(s: Seq<u32>) -> bool {
    forall|i: int, j: int| 0 <= i <= j < s.len() ==> s[i] <= s[j]
}

fn insertion_sort(v: &mut Vec<u32>)
    ensures
        final(v)@.len() == old(v)@.len(),
        sorted(final(v)@),
        final(v)@.to_multiset() == old(v)@.to_multiset(),
{
    let n = v.len();
    let mut i: usize = 1;
    while i < n
        invariant
            n == v@.len(), 1 <= i || n == 0,
            i <= n || n == 0,
            forall|a: int, b: int| 0 <= a <= b < i && b < n ==> v@[a] <= v@[b],
            v@.to_multiset() == old(v)@.to_multiset(),
        decreases n - i,
    {
        let mut j: usize = i;
        while j > 0 && v[j - 1] > v[j]
            invariant
                n == v@.len(), 0 <= j <= i < n,
                // [0, i] trié sauf autour de j
                forall|a: int, b: int| 0 <= a <= b <= i && b != j && a != j ==> v@[a] <= v@[b],
                forall|a: int, b: int| j < a <= b <= i ==> v@[a] <= v@[b],
                forall|b: int| j < b <= i ==> v@[j as int] <= v@[b],
                v@.to_multiset() == old(v)@.to_multiset(),
            decreases j,
        {
            let t = v[j - 1];
            let u = v[j];
            v[j - 1] = u;
            v[j] = t;
            j -= 1;
        }
        i += 1;
    }
}
fn main() {}
}
