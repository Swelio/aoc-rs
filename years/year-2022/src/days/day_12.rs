//! Instructions are there: https://adventofcode.com/2022/day/12

use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader, Read, Seek};

use petgraph::{
    algo::{astar, dijkstra},
    graph::{Graph, NodeIndex},
    Directed,
};

use utils::{CodeSolution, SantaError};

type Elevation = char;
type EndNode = NodeIndex;
type Grid = Vec<Vec<char>>;
type HeightGraph = Graph<Elevation, (), Directed>;
type ParsedGraph = (HeightGraph, Option<StartNode>, Option<EndNode>);
type StartNode = NodeIndex;
type Steps = u16;

const START_POINT: char = 'S';
const END_POINT: char = 'E';

pub struct DailySolution;

impl CodeSolution for DailySolution {
    fn run<I>(input: I) -> Result<(), Box<dyn Error>>
    where
        I: Read + Seek,
    {
        let (graph, start_node, end_node) = parse_graph(BufReader::new(input))?;
        let start_node = start_node
            .ok_or_else(|| SantaError::InvalidInput("No start point found.".to_owned()))?;
        let end_node =
            end_node.ok_or_else(|| SantaError::InvalidInput("No end point found.".to_owned()))?;

        let steps = part_1(&graph, start_node, end_node)?;
        println!(
            "The shortest path from node {} to node {} has {} steps.",
            start_node.index(),
            end_node.index(),
            steps
        );

        let steps = part_2(graph, end_node)?;
        println!(
            "The shortest path from lowest to highest elevation is {} steps.",
            steps
        );

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Position {
    x: usize,
    y: usize,
}

fn part_1(
    graph: &HeightGraph,
    start_node: NodeIndex,
    end_node: NodeIndex,
) -> Result<Steps, SantaError> {
    dijkstra(graph, start_node, Some(end_node), |_| 1)
        .get(&end_node)
        .map(|s| *s as Steps)
        .ok_or(SantaError::NoPathFound)
}

fn part_2(mut graph: HeightGraph, end_node: NodeIndex) -> Result<Steps, SantaError> {
    graph.reverse();

    astar(&graph, end_node, |n| graph[n] == 'a', |_| 1, |_| 0)
        .map(|(steps, _)| steps as Steps)
        .ok_or(SantaError::NoPathFound)
}

fn get_elevation(value: Elevation) -> Elevation {
    match value {
        START_POINT => 'a',
        END_POINT => 'z',
        c => c,
    }
}

fn parse_graph<I>(input: I) -> Result<ParsedGraph, Box<dyn Error>>
where
    I: BufRead,
{
    let mut grid = Grid::new();

    for line in input.lines() {
        grid.push(line?.chars().collect());
    }

    let mut start_node = None;
    let mut end_node = None;
    let (grid_x, grid_y) = (grid.len(), grid.first().unwrap().len());
    let mut graph = Graph::with_capacity(grid_x * grid_y, 0);
    let mut node_ids = HashMap::with_capacity(grid_x * grid_y);

    for (x, row) in grid.iter().enumerate() {
        for (y, &elevation) in row.iter().enumerate() {
            let current_position = Position { x, y };
            let node_id = graph.add_node(elevation);

            node_ids.insert(current_position, node_id);

            if elevation == START_POINT {
                let _ = start_node.insert(node_id);
            } else if elevation == END_POINT {
                let _ = end_node.insert(node_id);
            }

            let canonical_elevation = get_elevation(elevation);

            for angle in [std::f64::consts::PI, 3.0 * std::f64::consts::FRAC_PI_2] {
                let (x_mod, y_mod) = angle.sin_cos();

                // Calculate coordinates of the next position
                let next_x = match x.checked_add_signed(x_mod.round() as isize) {
                    Some(x) => x,
                    None => continue,
                };
                let next_y = match y.checked_add_signed(y_mod.round() as isize) {
                    Some(y) => y,
                    None => continue,
                };

                // Retrieve the value of the next position (converting S to as and E to z)
                let next_elevation =
                    get_elevation(match grid.get(next_x).map(|row| row.get(next_y)) {
                        Some(Some(&val)) => val,
                        _ => continue,
                    });

                let previous_position = Position {
                    x: next_x,
                    y: next_y,
                };

                if canonical_elevation as u8 == next_elevation as u8 - 1
                    || canonical_elevation >= next_elevation
                {
                    graph.add_edge(node_id, *node_ids.get(&previous_position).unwrap(), ());
                }

                if next_elevation as u8 == canonical_elevation as u8 - 1
                    || next_elevation >= canonical_elevation
                {
                    graph.add_edge(*node_ids.get(&previous_position).unwrap(), node_id, ());
                }
            }
        }
    }

    Ok((graph, start_node, end_node))
}

#[cfg(test)]
mod test_day {
    use std::io::{BufReader, Cursor};

    use super::*;

    const SAMPLE_INPUT: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    #[test]
    fn test_part_1() {
        let input_cursor = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let (map, start, end) = parse_graph(input_cursor).unwrap();
        let steps = part_1(&map, start.unwrap(), end.unwrap()).unwrap();

        assert_eq!(steps, 31);
    }

    #[test]
    fn test_part_2() {
        let input_cursor = Cursor::new(SAMPLE_INPUT);
        let (map, _, end) = parse_graph(input_cursor).unwrap();
        let steps = part_2(map, end.unwrap()).unwrap();

        assert_eq!(steps, 29);
    }
}
