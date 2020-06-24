use std::collections::HashMap;

#[derive(Debug)]
struct Neighbor<'a> {
    vertex: &'a str,
    distance: i32,
}

#[derive(Debug)]
struct Result<'a> {
    shortestDistance: i32,
    previousVertex: Option<&'a str>,
}

fn createGraph<'a>() -> HashMap<&'static str, Vec<Neighbor<'a>>> {
    let mut paths = HashMap::new();
    let mut neighbors = vec![];
    neighbors.push(Neighbor {
        vertex: "B",
        distance: 6,
    });
    neighbors.push(Neighbor {
        vertex: "D",
        distance: 1,
    });
    paths.insert("A", neighbors);

    neighbors = vec![];

    neighbors.push(Neighbor {
        vertex: "A",
        distance: 1,
    });
    neighbors.push(Neighbor {
        vertex: "C",
        distance: 5,
    });
    neighbors.push(Neighbor {
        vertex: "D",
        distance: 2,
    });
    neighbors.push(Neighbor {
        vertex: "E",
        distance: 2,
    });

    paths.insert("B", neighbors);

    neighbors = vec![];

    neighbors.push(Neighbor {
        vertex: "B",
        distance: 5,
    });
    neighbors.push(Neighbor {
        vertex: "E",
        distance: 5,
    });

    paths.insert("C", neighbors);

    neighbors = vec![];

    neighbors.push(Neighbor {
        vertex: "A",
        distance: 1,
    });
    neighbors.push(Neighbor {
        vertex: "B",
        distance: 2,
    });
    neighbors.push(Neighbor {
        vertex: "E",
        distance: 1,
    });

    paths.insert("D", neighbors);

    neighbors = vec![];

    neighbors.push(Neighbor {
        vertex: "B",
        distance: 2,
    });
    neighbors.push(Neighbor {
        vertex: "C",
        distance: 5,
    });
    neighbors.push(Neighbor {
        vertex: "D",
        distance: 1,
    });

    paths.insert("E", neighbors);

    return paths;
}

fn createResults<'a>() -> HashMap<&'static str, Box<Result<'a>>> {
    let mut results = HashMap::new();

    results.insert(
        "A",
        Box::new(Result {
            shortestDistance: i32::MAX,
            previousVertex: None,
        }),
    );
    results.insert(
        "B",
        Box::new(Result {
            shortestDistance: i32::MAX,
            previousVertex: None,
        }),
    );
    results.insert(
        "C",
        Box::new(Result {
            shortestDistance: i32::MAX,
            previousVertex: None,
        }),
    );
    results.insert(
        "D",
        Box::new(Result {
            shortestDistance: i32::MAX,
            previousVertex: None,
        }),
    );
    results.insert(
        "E",
        Box::new(Result {
            shortestDistance: i32::MAX,
            previousVertex: None,
        }),
    );

    return results;
}

fn visitVertex<'a>(
    vertex: &'a str,
    paths: &HashMap<&'a str, Vec<Neighbor<'a>>>,
    unvisited: &mut Vec<&str>,
    visited: &mut Vec<&'a str>,
    results: &mut HashMap<&'a str, Box<Result<'a>>>,
) {
    if !unvisited.contains(&vertex) {
        return;
    }

    let curDistance = results
        .get(vertex)
        .expect(&*format!("No value found for vertex {}", vertex))
        .shortestDistance;
    let neighbors = paths
        .get(vertex)
        .expect(&*format!("No neighbors found for vertex {}", vertex));
    for Neighbor {
        vertex: neighbor,
        distance,
    } in neighbors.iter()
    {
        if unvisited.contains(&neighbor) {
            let newDistance = curDistance + distance;
            let mut result = results
                .get_mut(neighbor)
                .expect(&*format!("No result found for vertex {}", neighbor));
            if newDistance < result.shortestDistance {
                result.shortestDistance = newDistance;
                result.previousVertex = Some(vertex);
            }
        }
    }

    visited.push(vertex);
    unvisited.retain(|v| *v != vertex);
}

fn findPaths<'a>(
    start: &'a str,
    paths: HashMap<&'a str, Vec<Neighbor<'a>>>,
) -> HashMap<&'a str, Box<Result<'a>>> {
    let mut unvisited = vec!["A", "B", "C", "D", "E"];
    let mut visited: Vec<&str> = vec![];
    let mut results = createResults();
    let result = results
        .get_mut(start)
        .expect(&*format!("No result found for start {}", start));
    result.shortestDistance = 0;

    visitVertex(start, &paths, &mut unvisited, &mut visited, &mut results);

    while unvisited.len() > 0 {
        let mut minDistance = i32::MAX;
        let mut nextVertex: Option<&str> = None;
        for v in unvisited.iter() {
            let result = results
                .get(v)
                .expect(&*format!("No result found for v {}", v));
            if result.shortestDistance < minDistance {
                minDistance = result.shortestDistance;
                nextVertex = Some(v);
            }
        }

        match nextVertex {
            Some(v) => visitVertex(v, &paths, &mut unvisited, &mut visited, &mut results),
            None => return results,
        }
    }

    return results;
}

fn main() {
    let paths = createGraph();

    let results = findPaths("A", paths);

    for v in ["A", "B", "C", "D", "E"].iter() {
        let result = results.get(v).expect("Not found");
        println!(
            "{}\t{}\t{:?}",
            v, result.shortestDistance, result.previousVertex
        );
    }
}
