use graphs::Graph;

// For `a` > `b` returns whether number `a` is closer to
// `to` than `b`.
fn closer_to(a: usize, b: usize, to: usize) -> bool {
  match (a > to, b > to) {
    (_, true) => false,
    (true, false) => a - to <= to - b,
    (false, false) => true,
  }
}

pub fn approximate_length_search(
  graph: &Graph,
  start: usize,
  end: usize,
  length: usize,
) -> Option<Vec<usize>> {
  // A queue to maintain the vertices whose adjacency list
  // is to be scanned as per normal DFS algorithm.
  let mut queue = std::collections::VecDeque::new();

  // Keeps the distance to the start to each vertex.
  // usize::MAX means that the vertex wasn't reached.
  let mut distance = vec![usize::MAX; graph.size];

  // Keeps the predecessor of each vertex.
  // usize::MAX means that the vertex wasn't reached.
  let mut predecessors = vec![usize::MAX; graph.size];

  // The distance from the start to itself is 0.
  distance[start] = 0;

  queue.push_back(start);

  // See https://en.wikipedia.org/wiki/Breadth-first_search
  while let Some(current) = queue.pop_front() {
    for &neighbor in graph.get_neighbors(current) {
      if distance[neighbor] == usize::MAX {
        distance[neighbor] = distance[current] + 1;
        predecessors[neighbor] = current;

        if neighbor != end {
          queue.push_back(neighbor);
        }
      } else if neighbor == end {
        if closer_to(
          distance[current] + 1,
          distance[neighbor],
          length,
        ) {
          distance[neighbor] = distance[current] + 1;
          predecessors[neighbor] = current;
        } else {
          break;
        }
      }
    }
  }

  if predecessors[end] != usize::MAX {
    let mut path = vec![];
    let mut current = end;
    while current != usize::MAX {
      path.push(current);
      current = predecessors[current];
    }

    path.reverse();

    Some(path)
  } else {
    None
  }
}
