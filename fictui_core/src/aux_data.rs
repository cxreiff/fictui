use rusqlite::Connection;

use crate::{
    database::{
        conversions::Retrievable,
        fields::direction::Direction,
        tables::{gates::Gate, tiles::Tile},
    },
    save_data::SaveData,
};

#[derive(Debug)]
pub struct AuxData {
    pub tile: TileWithNeighbors,
}

#[derive(Debug)]
pub struct TileWithNeighbors {
    pub tile: Tile,
    pub neighbors: Vec<Neighbor>,
}

#[derive(Debug)]
pub struct Neighbor {
    pub gate: Gate,
    pub tile: Option<TileWithNeighbors>,
}

impl AuxData {
    pub fn construct(connection: &Connection, save_data: &SaveData) -> rusqlite::Result<Self> {
        let tile = TileWithNeighbors::retrieve_recursive(connection, save_data.current_tile, 3)?;

        Ok(Self { tile })
    }
}

impl TileWithNeighbors {
    pub fn neighbor(&self, direction: &Direction) -> Option<&Neighbor> {
        self.neighbors
            .iter()
            .find(|neighbor| neighbor.gate.direction == *direction)
    }

    pub fn retrieve_recursive(
        connection: &Connection,
        id: i32,
        depth: usize,
    ) -> rusqlite::Result<Self> {
        let mut tile_with_neighbors = TileWithNeighbors::retrieve(connection, id)?;

        if depth > 0 {
            for neighbor in &mut tile_with_neighbors.neighbors {
                neighbor.tile = Some(TileWithNeighbors::retrieve_recursive(
                    connection,
                    neighbor.gate.destination_id,
                    depth - 1,
                )?);
            }
        }

        Ok(tile_with_neighbors)
    }
}

impl Retrievable for TileWithNeighbors {
    fn retrieve(connection: &Connection, id: i32) -> rusqlite::Result<Self> {
        let tile_with_neighbors = connection
            .prepare(
                "SELECT
                    t.name,
                    t.summary,
                    t.body,
                    g.name,
                    g.summary,
                    g.body,
                    g.direction,
                    g.source_id,
                    g.destination_id
                FROM
                    tiles AS t
                    LEFT JOIN gates AS g ON t.id = g.source_id
                WHERE
                    t.id = ?1
                ",
            )?
            .query((id,))?
            .mapped(|row| {
                let tile = Tile {
                    name: row.get(0)?,
                    summary: row.get(1)?,
                    body: row.get(2)?,
                };

                let gate = if row.get::<usize, String>(3).is_ok() {
                    Some(Gate {
                        name: row.get(3)?,
                        summary: row.get(4)?,
                        body: row.get(5)?,
                        direction: row.get(6)?,
                        source_id: row.get(7)?,
                        destination_id: row.get(8)?,
                    })
                } else {
                    None
                };

                Ok((tile, gate))
            })
            .fold(None, |acc, row| {
                let Ok((tile, gate)) = row else { return acc };

                acc.or(Some(TileWithNeighbors {
                    tile,
                    neighbors: vec![],
                }))
                .map(|mut tile_extended| {
                    if let Some(gate) = gate {
                        tile_extended.neighbors.push(Neighbor { gate, tile: None });
                    }
                    tile_extended
                })
            })
            .expect("id should match one tile");

        Ok(tile_with_neighbors)
    }
}
