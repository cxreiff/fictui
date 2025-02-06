CREATE TABLE gates (
    id                  INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name                VARCHAR NOT NULL,
    summary             VARCHAR NOT NULL,
    body                TEXT NOT NULL,
    source_tile_id      INTEGER NOT NULL,
    destination_tile_id INTEGER NOT NULL,
    direction           TEXT CHECK(direction IN ('north', 'east', 'south', 'west', 'up', 'down')) NOT NULL,

    FOREIGN KEY(source_tile_id) REFERENCES tiles(id),
    FOREIGN KEY(destination_tile_id) REFERENCES tiles(id)
)
