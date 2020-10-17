use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;

type Graph<V, E> = BTreeMap<V, Vec<(V, E)>>;

pub fn prim<V: Ord + Clone>(graph: &Graph<V, i64>) -> Graph<V, i64> {
    match graph.keys().next() {
        Some(v) => prim_with_start(graph, v),
        None => BTreeMap::new(),
    }
}

pub fn prim_with_start<V: Ord + Clone>(graph: &Graph<V, i64>, start: &V) -> Graph<V, i64> {
    let mut mst = Graph::new();
    let mut prio = BinaryHeap::new();
    prio.push(Reverse((0, start, None)));

    while let Some(Reverse((d, t, p))) = prio.pop() {
        if mst.contains_key(t) {
            continue;
        }

        mst.insert(t.clone(), Vec::new());

        // special case:
        // the first vertex has no predecessor
        if let Some(prev) = p {
            mst.get_mut(prev).unwrap().push((t.clone(), d));
            mst.get_mut(t).unwrap().push((prev.clone(), d));
        }

        for (v, c) in &graph[&t] {
            if !mst.contains_key(&v) {
                prio.push(Reverse((*c, &v, Some(t))));
            }
        }
    }

    mst
}
