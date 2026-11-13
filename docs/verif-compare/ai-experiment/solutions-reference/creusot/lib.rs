use creusot_std::prelude::*;

#[logic(open)]
#[variant(n)]
#[requires(0 <= n && n <= s.len())]
pub fn sum_to(s: Seq<u32>, n: Int) -> Int {
    pearlite! { if n == 0 { 0 } else { sum_to(s, n - 1) + s[n - 1]@ } }
}

#[requires(v@.len() <= 1_000_000)]
#[requires(forall<i: Int> 0 <= i && i < v@.len() ==> v@[i]@ <= 1000)]
#[ensures(result@ == sum_to(v@, v@.len()))]
pub fn sum(v: &[u32]) -> u64 {
    let mut s: u64 = 0;
    let mut i: usize = 0;
    #[invariant(i@ <= v@.len())]
    #[invariant(s@ == sum_to(v@, i@))]
    #[invariant(s@ <= 1000 * i@)]
    while i < v.len() {
        s += v[i] as u64;
        i += 1;
    }
    s
}

#[requires(forall<i: Int, j: Int> 0 <= i && i <= j && j < v@.len() ==> v@[i] <= v@[j])]
#[ensures(match result {
    Some(i) => i@ < v@.len() && v@[i@] == key,
    None => forall<i: Int> 0 <= i && i < v@.len() ==> v@[i] != key,
})]
pub fn search(v: &[u64], key: u64) -> Option<usize> {
    let mut lo: usize = 0;
    let mut hi: usize = v.len();
    #[invariant(lo@ <= hi@ && hi@ <= v@.len())]
    #[invariant(forall<i: Int> 0 <= i && i < lo@ ==> v@[i] < key)]
    #[invariant(forall<i: Int> hi@ <= i && i < v@.len() ==> v@[i] > key)]
    while lo < hi {
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

#[ensures((^v)@.len() == v@.len())]
#[ensures(forall<i: Int> 0 <= i && i < v@.len() ==> (^v)@[i] == v@[v@.len() - 1 - i])]
pub fn reverse(v: &mut [u32]) {
    let old = snapshot!(v@);
    let n = v.len();
    let mut i: usize = 0;
    #[invariant(v@.len() == n@ && i@ <= n@ / 2)]
    #[invariant(forall<k: Int> 0 <= k && k < i@ ==> v@[k] == old[n@ - 1 - k])]
    #[invariant(forall<k: Int> n@ - i@ <= k && k < n@ ==> v@[k] == old[n@ - 1 - k])]
    #[invariant(forall<k: Int> i@ <= k && k < n@ - i@ ==> v@[k] == old[k])]
    while i < n / 2 {
        v.swap(i, n - 1 - i);
        i += 1;
    }
}

#[ensures((^v)@.len() == v@.len())]
#[ensures(forall<i: Int, j: Int> 0 <= i && i <= j && j < (^v)@.len() ==> (^v)@[i] <= (^v)@[j])]
#[ensures((^v)@.permutation_of(v@))]
pub fn insertion_sort(v: &mut [u32]) {
    let old = snapshot!(v@);
    let n = v.len();
    let mut i: usize = 1;
    #[invariant(v@.len() == n@)]
    #[invariant(n@ == 0 || i@ <= n@)]
    #[invariant(forall<a: Int, b: Int> 0 <= a && a <= b && b < i@ && b < n@ ==> v@[a] <= v@[b])]
    #[invariant(v@.permutation_of(*old))]
    while i < n {
        let mut j: usize = i;
        #[invariant(v@.len() == n@ && j@ <= i@ && i@ < n@)]
        #[invariant(forall<a: Int, b: Int> 0 <= a && a <= b && b <= i@ && a != j@ && b != j@ ==> v@[a] <= v@[b])]
        #[invariant(forall<b: Int> j@ < b && b <= i@ ==> v@[j@] <= v@[b])]
        #[invariant(v@.permutation_of(*old))]
        while j > 0 && v[j - 1] > v[j] {
            v.swap(j - 1, j);
            j -= 1;
        }
        i += 1;
    }
}
