//! Day 20: Jurassic Jigsaw
//! https://adventofcode.com/2020/day/20

use crate::SimpleError;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Edge(Vec<bool>);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct CanonicalizedEdge(Vec<bool>);

impl Edge {
    fn canonicalize(&self) -> CanonicalizedEdge {
        let edge = &self.0;

        let mut i = 0;
        let mut j = edge.len() - 1;
        while i < j {
            if edge[i] != edge[j] {
                return if edge[i] {
                    CanonicalizedEdge(edge.clone())
                } else {
                    CanonicalizedEdge(edge.iter().rev().copied().collect())
                };
            }

            i += 1;
            j -= 1;
        }

        CanonicalizedEdge(edge.clone())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum EdgeDirection {
    Top,
    Bottom,
    Left,
    Right,
}

impl EdgeDirection {
    fn rotated_left(&self) -> Self {
        match self {
            Self::Top => Self::Left,
            Self::Left => Self::Bottom,
            Self::Bottom => Self::Right,
            Self::Right => Self::Top,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Image(Vec<Vec<bool>>);

impl Image {
    fn get_edge(&self, direction: EdgeDirection) -> Edge {
        match direction {
            EdgeDirection::Top => self.get_top_edge(),
            EdgeDirection::Bottom => self.get_bottom_edge(),
            EdgeDirection::Left => self.get_left_edge(),
            EdgeDirection::Right => self.get_right_edge(),
        }
    }

    fn get_top_edge(&self) -> Edge {
        Edge(self.0[0].clone())
    }

    fn get_bottom_edge(&self) -> Edge {
        Edge(self.0.last().unwrap().clone())
    }

    fn get_left_edge(&self) -> Edge {
        Edge((0..self.0.len()).map(|i| self.0[i][0]).collect())
    }

    fn get_right_edge(&self) -> Edge {
        Edge(
            (0..self.0.len())
                .map(|i| self.0[i][self.0.len() - 1])
                .collect(),
        )
    }

    fn get_edges_with_direction(&self) -> Vec<(EdgeDirection, Edge)> {
        vec![
            (EdgeDirection::Top, self.get_top_edge()),
            (EdgeDirection::Bottom, self.get_bottom_edge()),
            (EdgeDirection::Left, self.get_left_edge()),
            (EdgeDirection::Right, self.get_right_edge()),
        ]
    }

    fn rotated_left(&self) -> Self {
        let mut rotated_image = vec![vec![false; self.0.len()]; self.0[0].len()];

        for (i, row) in self.0.iter().enumerate() {
            for (j, &b) in row.iter().enumerate() {
                rotated_image[self.0[0].len() - 1 - j][i] = b;
            }
        }

        Self(rotated_image)
    }

    fn flipped_horizontally(&self) -> Self {
        let flipped_image = self
            .0
            .iter()
            .map(|row| row.iter().rev().copied().collect())
            .collect();

        Self(flipped_image)
    }

    fn flipped_vertically(&self) -> Self {
        let flipped_image = self.0.iter().rev().cloned().collect();

        Self(flipped_image)
    }

    fn reoriented_to(&self, target_edge: &Edge, target_direction: EdgeDirection) -> Self {
        let canonicalized_target_edge = target_edge.canonicalize();

        let (matching_direction, _) = self
            .get_edges_with_direction()
            .into_iter()
            .find(|(_, edge)| edge.canonicalize() == canonicalized_target_edge)
            .unwrap();

        let mut direction = matching_direction;
        let mut image = self.clone();
        while direction != target_direction {
            direction = direction.rotated_left();
            image = image.rotated_left();
        }

        if image.get_edge(target_direction) != *target_edge {
            match target_direction {
                EdgeDirection::Top | EdgeDirection::Bottom => {
                    image = image.flipped_horizontally();
                }
                EdgeDirection::Left | EdgeDirection::Right => {
                    image = image.flipped_vertically();
                }
            }
        }

        image
    }

    fn without_border(&self) -> Self {
        let image = self.0[1..self.0.len() - 1]
            .iter()
            .map(|row| row[1..row.len() - 1].to_vec())
            .collect();

        Self(image)
    }

    fn cardinality(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|&&b| b).count())
            .sum()
    }

    fn count_occurrences(&self, other: &Self) -> usize {
        let mut count = 0;

        for corner_i in 0..(self.0.len() - other.0.len()) {
            for corner_j in 0..(self.0[0].len() - other.0[0].len()) {
                let mut matches = true;

                'outer: for i in 0..other.0.len() {
                    for j in 0..other.0[0].len() {
                        if other.0[i][j] && !self.0[corner_i + i][corner_j + j] {
                            matches = false;
                            break 'outer;
                        }
                    }
                }

                if matches {
                    count += 1;
                }
            }
        }

        count
    }

    fn combine(images: &Vec<Vec<Self>>) -> Self {
        let mut combined = Vec::new();

        for image_row in images {
            for i in 0..image_row[0].0.len() {
                let combined_row: Vec<_> = image_row
                    .iter()
                    .flat_map(|image| image.0[i].clone())
                    .collect();
                combined.push(combined_row);
            }
        }

        Self(combined)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Tile {
    id: u64,
    image: Image,
}

fn solve_part_1(input: &str) -> Result<u64, SimpleError> {
    let tiles = parse_input(input)?;

    let mut edge_to_count = HashMap::new();
    for (_, edge) in tiles
        .iter()
        .flat_map(|tile| tile.image.get_edges_with_direction())
    {
        let edge = edge.canonicalize();
        if let Some(count) = edge_to_count.get_mut(&edge) {
            *count += 1;
        } else {
            edge_to_count.insert(edge, 1);
        }
    }

    let corner_tiles: Vec<_> = tiles
        .iter()
        .filter(|tile| {
            let unmatched_edge_count = tile
                .image
                .get_edges_with_direction()
                .into_iter()
                .filter(|(_, edge)| *edge_to_count.get(&edge.canonicalize()).unwrap() == 1)
                .count();
            unmatched_edge_count == 2
        })
        .collect();

    let corner_tile_id_product = corner_tiles.iter().map(|tile| tile.id).product();

    Ok(corner_tile_id_product)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let tiles = parse_input(input)?;

    let connected_tiles = connect_tiles(&tiles);
    let connected_images_without_border: Vec<Vec<_>> = connected_tiles
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|tile| tile.image.without_border())
                .collect()
        })
        .collect();

    let combined_image = Image::combine(&connected_images_without_border);

    let sea_monster_count = count_sea_monsters(&combined_image);

    let sea_monster_image = new_sea_monster_image();
    Ok(combined_image.cardinality() - sea_monster_count * sea_monster_image.cardinality())
}

const SEA_MONSTER_PATTERN: &str = concat!(
    "                  # \n",
    "#    ##    ##    ###\n",
    " #  #  #  #  #  #   \n",
);

fn new_sea_monster_image() -> Image {
    let sea_monster_image = SEA_MONSTER_PATTERN
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    Image(sea_monster_image)
}

fn count_sea_monsters(image: &Image) -> usize {
    let mut sea_monster_image = new_sea_monster_image();
    for _ in 0..2 {
        for _ in 0..4 {
            let sea_monster_occurrences = image.count_occurrences(&sea_monster_image);
            if sea_monster_occurrences > 0 {
                return sea_monster_occurrences;
            }

            sea_monster_image = sea_monster_image.rotated_left();
        }

        sea_monster_image = sea_monster_image.flipped_horizontally();
    }

    0
}

fn connect_tiles(tiles: &[Tile]) -> Vec<Vec<Tile>> {
    let mut edge_to_ids: HashMap<CanonicalizedEdge, Vec<u64>> = HashMap::new();
    for tile in tiles {
        for (_, edge) in tile.image.get_edges_with_direction() {
            let edge = edge.canonicalize();
            if let Some(tile_ids) = edge_to_ids.get_mut(&edge) {
                tile_ids.push(tile.id);
            } else {
                edge_to_ids.insert(edge, vec![tile.id]);
            }
        }
    }

    let mut top_left_tile = tiles
        .iter()
        .find(|tile| {
            let unmatched_edge_count = tile
                .image
                .get_edges_with_direction()
                .into_iter()
                .filter(|(_, edge)| edge_to_ids.get(&edge.canonicalize()).unwrap().len() == 1)
                .count();
            unmatched_edge_count == 2
        })
        .cloned()
        .unwrap();

    loop {
        let top_edge = top_left_tile.image.get_top_edge().canonicalize();
        let left_edge = top_left_tile.image.get_left_edge().canonicalize();

        if edge_to_ids.get(&top_edge).unwrap().len() == 1
            && edge_to_ids.get(&left_edge).unwrap().len() == 1
        {
            break;
        }

        top_left_tile = Tile {
            id: top_left_tile.id,
            image: top_left_tile.image.rotated_left(),
        };
    }

    let id_to_tile: HashMap<_, _> = tiles.iter().map(|tile| (tile.id, tile)).collect();

    let connected_side_len = (tiles.len() as f64).sqrt().round() as usize;
    let mut connected_tiles: Vec<Vec<Tile>> = Vec::new();
    let mut current_row = vec![top_left_tile];
    for i in 0..connected_side_len {
        for j in 0..connected_side_len {
            if j == 0 {
                if i == 0 {
                    continue;
                }

                let target_tile = &connected_tiles[connected_tiles.len() - 1][0];
                let target_edge = target_tile.image.get_bottom_edge();

                let matching_tile_id = edge_to_ids
                    .get(&target_edge.canonicalize())
                    .unwrap()
                    .iter()
                    .copied()
                    .find(|&id| id != target_tile.id)
                    .unwrap();
                let matching_tile = *id_to_tile.get(&matching_tile_id).unwrap();

                let reoriented_image = matching_tile
                    .image
                    .reoriented_to(&target_edge, EdgeDirection::Top);
                current_row.push(Tile {
                    id: matching_tile_id,
                    image: reoriented_image,
                });
            } else {
                let target_tile = current_row.last().unwrap();
                let target_edge = target_tile.image.get_right_edge();

                let matching_tile_id = edge_to_ids
                    .get(&target_edge.canonicalize())
                    .unwrap()
                    .iter()
                    .copied()
                    .find(|&id| id != target_tile.id)
                    .unwrap();
                let matching_tile = *id_to_tile.get(&matching_tile_id).unwrap();

                let reoriented_image = matching_tile
                    .image
                    .reoriented_to(&target_edge, EdgeDirection::Left);
                current_row.push(Tile {
                    id: matching_tile_id,
                    image: reoriented_image,
                });
            }
        }
        connected_tiles.push(current_row);
        current_row = Vec::new();
    }

    connected_tiles
}

fn parse_input(input: &str) -> Result<Vec<Tile>, SimpleError> {
    let lines: Vec<_> = input.lines().collect();

    lines
        .split(|s| s.is_empty())
        .filter(|tile_lines| !tile_lines.is_empty())
        .map(|tile_lines| {
            if tile_lines.len() < 2 {
                return Err(SimpleError::new(format!(
                    "found a tile with only {} lines",
                    tile_lines.len()
                )));
            }

            let (_, tile_id) = tile_lines[0].split_once(' ').ok_or_else(|| {
                SimpleError::new(format!(
                    "tile id line not in expected format: {}",
                    tile_lines[0]
                ))
            })?;

            let tile_id = tile_id[..tile_id.len() - 1].parse()?;

            let image = tile_lines[1..]
                .iter()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => Ok(true),
                            '.' => Ok(false),
                            _ => Err(SimpleError::new(format!("invalid image char: {c}"))),
                        })
                        .collect()
                })
                .collect::<Result<_, _>>()?;

            Ok(Tile {
                id: tile_id,
                image: Image(image),
            })
        })
        .collect()
}

pub fn solve(input: &str) -> Result<(u64, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample20.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(20899048083289), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(273), solve_part_2(SAMPLE_INPUT));
    }
}
