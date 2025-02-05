CREATE TABLE tile_instances (
    id      INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    tile_id INTEGER NOT NULL,

    FOREIGN KEY(tile_id) REFERENCES tiles(id)
)
