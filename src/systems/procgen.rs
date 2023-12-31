use super::camps::Camp;

pub const TILE_SIDE: i32 = 35;

#[derive(PartialEq, Clone, Debug)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Vec<Tile>>,
    pub caves: Vec<CaveNode>,
    pub camps: Vec<Camp>,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Tile {
    pub tile: Tiles,
    pub smooth_tile: Option<SmoothTiles>,
    pub durability: i32,
}

impl Tile {
    pub fn new(tile: Tiles) -> Self {
        Self {
            tile,
            smooth_tile: None,
            durability: tile.durability(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct CaveNode {
    x: i32,
    y: i32,
    next: Vec<CaveNode>,
    radius: i32,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Tiles {
    /// ------------------- World building -------------------
    Air,
    Grass,
    Dirt,
    Stone,
    HardStone,
    /// HardStone is a stone that is harder to break than normal stone and forms the deepest layer of the world.

    /// ------------------- Ores -------------------
    /// Ores are scattered throughout the stone and hardstone layers.
    /// They are forming small clusters of 2-5 ores.
    IronOre,
    GoldOre,
    RuneOre,

    /// ------------------- Gems -------------------
    /// Gems are rare and can be found in the hardstone layer inside caves.
    /// They are forming bigger clusters of 5-7 gems.
    Ruby,
    Emerald,
    Sapphire,
    Diamond,

    /// ------------------- Environmental -------------------
    /// Environmental tiles are usually used for generated structures.
    Log,
    Leaves,

    /// ------------------- Player built -------------------
    /// Player built tiles are created and placed by the player.
    Wood,
    HardenedStone,
    Iron,


    /// TileData represents a block that holds data, that data will be stored somewhere else
    TileData,
}

impl Tiles {
    pub fn durability(&self) -> i32 {
        match self {
            _ => 5,
        }
    }
}

/// Smooth tiles can be placed on top of other tiles.
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum SmoothTiles {
    Pebbles,
    Sand,
    Vine,
    Torch,
    /// ------------------- Cave -------------------
    SpiderWeb,
    SpiderEgg,
}

impl Grid {
    /// Procedurally generates a new grid.
    pub fn new() -> Self {
        const WIDTH: i32 = 512;
        const HEIGHT: i32 = 1024;
        let mut tiles = Vec::new();
        let mut caves = Vec::new();
        let mut camps = Vec::new();

        for _ in 0..WIDTH {
            let mut row = Vec::new();
            for _ in 0..HEIGHT {
                row.push(Tile {
                    tile: Tiles::Air,
                    smooth_tile: None,
                    durability: 0,
                });
            }
            tiles.push(row);
        }

        let mut this = Self {
            width: WIDTH,
            height: HEIGHT,
            tiles,
            caves,
            camps,
        };

        // lowest layer
        for x in 0..WIDTH {
            for y in 512..HEIGHT {
                if y < 513 {
                    this.set_tile(x, y, Tile::new(Tiles::Grass));
                } else if y < 518 {
                    this.set_tile(x, y, Tile::new(Tiles::Dirt));
                } else if y < 600 {
                    this.set_tile(x, y, Tile::new(Tiles::Stone));
                } else {
                    this.set_tile(x, y, Tile::new(Tiles::HardStone));
                }
            }
        }
        this
    }

    pub fn get_tile(&self, x: i32, y: i32) -> &Tile {
        &self.tiles[x as usize][self.height as usize - y as usize - 1]
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        self.tiles[x as usize][self.height as usize - y as usize - 1] = tile;
    }


    /// returns position of tile from real world coordinates
    pub fn get_coords(&mut self, x: f32, y: f32) -> (i32, i32) {
        ((x/TILE_SIDE as f32).floor() as i32, (y/TILE_SIDE as f32).floor() as i32)
    }
}
