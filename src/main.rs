use std::collections::HashMap;

#[derive(Debug)]
struct Neighbor<'a> {
    vertex: &'a str,
    distance: i32,
}

#[derive(Debug)]
struct Result<'a> {
    shortest_distance: i32,
    previous_vertex: Option<&'a str>,
}

fn create_graph<'a>() -> HashMap<&'static str, Vec<Neighbor<'a>>> {
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

fn create_results<'a>() -> HashMap<&'static str, Box<Result<'a>>> {
    let mut results = HashMap::new();

    results.insert(
        "A",
        Box::new(Result {
            shortest_distance: i32::MAX,
            previous_vertex: None,
        }),
    );
    results.insert(
        "B",
        Box::new(Result {
            shortest_distance: i32::MAX,
            previous_vertex: None,
        }),
    );
    results.insert(
        "C",
        Box::new(Result {
            shortest_distance: i32::MAX,
            previous_vertex: None,
        }),
    );
    results.insert(
        "D",
        Box::new(Result {
            shortest_distance: i32::MAX,
            previous_vertex: None,
        }),
    );
    results.insert(
        "E",
        Box::new(Result {
            shortest_distance: i32::MAX,
            previous_vertex: None,
        }),
    );

    return results;
}

fn visit_vertex<'a>(
    vertex: &'a str,
    paths: &HashMap<&'a str, Vec<Neighbor<'a>>>,
    unvisited: &mut Vec<&str>,
    visited: &mut Vec<&'a str>,
    results: &mut HashMap<&'a str, Box<Result<'a>>>,
) {
    if !unvisited.contains(&vertex) {
        return;
    }

    let cur_distance = results
        .get(vertex)
        .expect(&*format!("No value found for vertex {}", vertex))
        .shortest_distance;
    let neighbors = paths
        .get(vertex)
        .expect(&*format!("No neighbors found for vertex {}", vertex));
    for Neighbor {
        vertex: neighbor,
        distance,
    } in neighbors.iter()
    {
        if unvisited.contains(&neighbor) {
            let new_distance = cur_distance + distance;
            let mut result = results
                .get_mut(neighbor)
                .expect(&*format!("No result found for vertex {}", neighbor));
            if new_distance < result.shortest_distance {
                result.shortest_distance = new_distance;
                result.previous_vertex = Some(vertex);
            }
        }
    }

    visited.push(vertex);
    unvisited.retain(|v| *v != vertex);
}

fn find_paths<'a>(
    start: &'a str,
    paths: HashMap<&'a str, Vec<Neighbor<'a>>>,
) -> HashMap<&'a str, Box<Result<'a>>> {
    let mut unvisited = vec!["A", "B", "C", "D", "E"];
    let mut visited: Vec<&str> = vec![];
    let mut results = create_results();
    let result = results
        .get_mut(start)
        .expect(&*format!("No result found for start {}", start));
    result.shortest_distance = 0;

    visit_vertex(start, &paths, &mut unvisited, &mut visited, &mut results);

    while unvisited.len() > 0 {
        let mut min_distance = i32::MAX;
        let mut next_vertex: Option<&str> = None;
        for v in unvisited.iter() {
            let result = results
                .get(v)
                .expect(&*format!("No result found for v {}", v));
            if result.shortest_distance < min_distance {
                min_distance = result.shortest_distance;
                next_vertex = Some(v);
            }
        }

        match next_vertex {
            Some(v) => visit_vertex(v, &paths, &mut unvisited, &mut visited, &mut results),
            None => return results,
        }
    }

    return results;
}

fn main() {
    let paths = create_graph();

    let results = find_paths("A", paths);

    for v in ["A", "B", "C", "D", "E"].iter() {
        let result = results.get(v).expect("Not found");
        println!(
            "{}\t{}\t{:?}",
            v, result.shortest_distance, result.previous_vertex
        );
    }
}
