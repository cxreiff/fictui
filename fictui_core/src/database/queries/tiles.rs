use crate::database::{
    tables::{gates::Gate, tile_instances::TileInstance, tiles::Tile},
    types::direction::Direction,
    Database,
};

#[derive(Debug)]
pub struct TileExtended {
    pub tile: Tile,
    pub tile_instance: TileInstance,
    pub neighbors: Vec<TileExtendedNeighbor>,
}

#[derive(Debug)]
pub struct TileExtendedNeighbor {
    pub gate: Gate,
    pub tile_instance: TileInstance,
    pub tile: Tile,
}

impl TileExtended {
    pub fn neighbor_in_direction(&self, direction: &Direction) -> Option<&TileExtendedNeighbor> {
        self.neighbors
            .iter()
            .find(|neighbor| *direction == neighbor.gate.direction)
    }
}

impl Database {
    pub fn extended_tile_query(&self, tile_instance_id: i32) -> rusqlite::Result<TileExtended> {
        let tile_extended = self
            .prepare(
                "SELECT
                    ti.id,
                    ti.tile_id,
                    t.id,
                    t.name,
                    t.summary,
                    t.body,
                    g.id,
                    g.name,
                    g.summary,
                    g.body,
                    g.direction,
                    g.source_id,
                    g.destination_id,
                    ni.id,
                    ni.tile_id,
                    nt.id,
                    nt.name,
                    nt.summary,
                    nt.body
                FROM
                    tile_instances AS ti
                    LEFT JOIN tiles AS t ON ti.tile_id = t.id
                    LEFT JOIN gates AS g ON ti.id = g.source_id
                    LEFT JOIN tile_instances AS ni ON ni.id = g.destination_id
                    LEFT JOIN tiles AS nt ON nt.id = ni.tile_id
                WHERE
                    ti.id = ?1
                ",
            )?
            .query((tile_instance_id,))?
            .mapped(|row| {
                Ok((
                    TileInstance {
                        id: row.get(0)?,
                        tile_id: row.get(1)?,
                    },
                    Tile {
                        id: row.get(2)?,
                        name: row.get(3)?,
                        summary: row.get(4)?,
                        body: row.get(5)?,
                    },
                    Gate {
                        id: row.get(6)?,
                        name: row.get(7)?,
                        summary: row.get(8)?,
                        body: row.get(9)?,
                        direction: row.get(10)?,
                        source_id: row.get(11)?,
                        destination_id: row.get(12)?,
                    },
                    TileInstance {
                        id: row.get(13)?,
                        tile_id: row.get(14)?,
                    },
                    Tile {
                        id: row.get(15)?,
                        name: row.get(16)?,
                        summary: row.get(17)?,
                        body: row.get(18)?,
                    },
                ))
            })
            .filter_map(Result::ok)
            .fold(
                None,
                |acc, (tile_instance, tile, gate, neighbor_tile_instance, neighbor_tile)| {
                    acc.or(Some(TileExtended {
                        tile_instance,
                        tile,
                        neighbors: vec![],
                    }))
                    .map(|mut tile_extended| {
                        tile_extended.neighbors.push(TileExtendedNeighbor {
                            gate,
                            tile_instance: neighbor_tile_instance,
                            tile: neighbor_tile,
                        });
                        tile_extended
                    })
                },
            )
            .unwrap();

        Ok(tile_extended)
    }
}
