//! bfs : graphe creux aléatoire (m = 16 n arêtes), CSR par tri par comptage, puis BFS depuis 0. Voir spec/workloads.d/bfs.md.

const SEED: u64 = 0xB5F0000000000001;
const INF: u32 = 0xFFFFFFFF;

#[inline(always)]
fn mix64(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E3779B97F4A7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}
#[inline(always)]
fn ckfold(ck: u64, v: u64) -> u64 {
    (ck ^ v).wrapping_mul(0x100000001B3)
}

pub fn run(n: usize, _threads: usize) -> u64 {
    let m = 16 * n;
    let nn = n as u64;
    let mut off = vec![0u32; n + 1];
    let mut adj = vec![0u32; m];
    for j in 0..m as u64 {
        off[(mix64(SEED.wrapping_add(2 * j)) % nn) as usize + 1] += 1;
    }
    for i in 1..=n {
        off[i] += off[i - 1];
    }
    {
        let mut cur = off[..n].to_vec();
        for j in 0..m as u64 {
            let u = (mix64(SEED.wrapping_add(2 * j)) % nn) as usize;
            let v = (mix64(SEED.wrapping_add(2 * j + 1)) % nn) as u32;
            adj[cur[u] as usize] = v;
            cur[u] += 1;
        }
    }
    let mut dist = vec![INF; n];
    let mut q = vec![0u32; n];
    let (mut head, mut tail) = (0usize, 1usize);
    dist[0] = 0;
    while head < tail {
        let u = q[head] as usize;
        head += 1;
        let du = dist[u] + 1;
        for e in off[u]..off[u + 1] {
            let v = adj[e as usize] as usize;
            if dist[v] == INF {
                dist[v] = du;
                q[tail] = v as u32;
                tail += 1;
            }
        }
    }
    let mut ck = 0xCBF29CE484222325u64;
    let mut maxd = 0u64;
    for &d in &dist {
        ck = ckfold(ck, d as u64);
        if d != INF && d as u64 > maxd {
            maxd = d as u64;
        }
    }
    ck = ckfold(ck, tail as u64);
    ckfold(ck, maxd)
}
