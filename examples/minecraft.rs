
use vertix::prelude::*;

#[derive(Copy, Clone, Default, Debug)]
pub struct Block {
    block_type: BlockType,
    is_solid: bool,
}
enum Face {
    Top,
    Bottom,
    Left,
    Right,
    Back,
    Front,
}
impl Block {
    pub fn new(block_type: BlockType) -> Self {
        let mut is_solid;
        match block_type {
            BlockType::Grass => is_solid = true,
            _ => is_solid = false,
        }
        Block {
            block_type,
            is_solid,
        }
    }
}
#[derive(Copy, Clone, Default, Debug)]
pub enum BlockType {
    #[default]
    Air,
    Water,
    Grass,
    Stone,
}
pub struct Chunk {
    blocks: Vec<Vec<Vec<Block>>>,
    mesh: Mesh,
}
pub async fn build_chunk(
    state: &mut State,
    blocks: &Vec<Vec<Vec<Block>>>,
    x_offset: f32,
    z_offset: f32,
    left_chunk: Option<&Vec<Vec<Vec<Block>>>>,
    right_chunk: Option<&Vec<Vec<Vec<Block>>>>,
    front_chunk: Option<&Vec<Vec<Vec<Block>>>>,
    back_chunk: Option<&Vec<Vec<Vec<Block>>>>,
) {
    let mut vertices: Vec<ModelVertex> = vec![];
    let mut indices: Vec<u32> = vec![];

    //vars in for loop code, preinitialized
    let mut grass_above;
    let mut neighbor_chunk_block_option;
    let mut base_index;
    let mut face;
    let mut neighbor;
    for (x, column) in blocks.iter().enumerate() {
        for (y, row) in column.iter().enumerate() {
            for (z, block) in row.iter().enumerate() {
                //init code
                if let BlockType::Air = block.block_type {
                    continue;
                }
                let pos = [x as f32 + x_offset, y as f32, z as f32 + z_offset];
                grass_above = y + 1 < column.len()
                    && matches!(blocks[x][y + 1][z].block_type, BlockType::Grass);

                //block rendering
                base_index = vertices.len() as u32;
                face = Face::Top;
                neighbor = if y + 1 < column.len() {
                    Some(&blocks[x][y + 1][z])
                } else {
                    None
                };
                get_block_face(
                    base_index,
                    face,
                    neighbor,
                    block,
                    pos,
                    &mut vertices,
                    &mut indices,
                    false,
                    None,
                );

                base_index = vertices.len() as u32;
                face = Face::Bottom;
                neighbor = if y > 0 {
                    Some(&blocks[x][y - 1][z])
                } else {
                    None
                };
                get_block_face(
                    base_index,
                    face,
                    neighbor,
                    block,
                    pos,
                    &mut vertices,
                    &mut indices,
                    false,
                    None,
                );

                base_index = vertices.len() as u32;
                face = Face::Left; //this is actually front i think
                neighbor = if x > 0 {
                    Some(&blocks[x - 1][y][z])
                } else {
                    None
                };
                neighbor_chunk_block_option =
                    left_chunk.map_or(None, |chunk| Some(&chunk[15][y][z]));
                get_block_face(
                    base_index,
                    face,
                    neighbor,
                    block,
                    pos,
                    &mut vertices,
                    &mut indices,
                    grass_above,
                    neighbor_chunk_block_option,
                );

                base_index = vertices.len() as u32;
                face = Face::Right;
                neighbor = if x + 1 < blocks.len() {
                    Some(&blocks[x + 1][y][z])
                } else {
                    None
                };
                neighbor_chunk_block_option =
                    right_chunk.map_or(None, |chunk| Some(&chunk[0][y][z]));
                get_block_face(
                    base_index,
                    face,
                    neighbor,
                    block,
                    pos,
                    &mut vertices,
                    &mut indices,
                    grass_above,
                    neighbor_chunk_block_option,
                );

                base_index = vertices.len() as u32;
                face = Face::Front;
                neighbor = if z + 1 < row.len() {
                    Some(&blocks[x][y][z + 1])
                } else {
                    None
                };
                neighbor_chunk_block_option =
                    front_chunk.map_or(None, |chunk| Some(&chunk[x][y][0]));
                get_block_face(
                    base_index,
                    face,
                    neighbor,
                    block,
                    pos,
                    &mut vertices,
                    &mut indices,
                    grass_above,
                    neighbor_chunk_block_option,
                );

                base_index = vertices.len() as u32;
                face = Face::Back;
                neighbor = if z > 0 {
                    Some(&blocks[x][y][z - 1])
                } else {
                    None
                };
                neighbor_chunk_block_option =
                    back_chunk.map_or(None, |chunk| Some(&chunk[x][y][15]));
                get_block_face(
                    base_index,
                    face,
                    neighbor,
                    block,
                    pos,
                    &mut vertices,
                    &mut indices,
                    grass_above,
                    neighbor_chunk_block_option,
                );
            }
        }
    }
    let position= cgmath::Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    let rotation = cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0));
    state.build_mesh(vertices, indices, "texture_atlas.png", vec![Instance { position, rotation  }]).await
}

fn get_block_face(
    base_index: u32,
    face: Face,
    neighbor_block_option: Option<&Block>,
    block: &Block,
    pos: [f32; 3],
    vertices: &mut Vec<ModelVertex>,
    indices: &mut Vec<u32>,
    grass_above: bool,
    neighbor_chunk_block_option: Option<&Block>,
) {
    let mut render = false;
    match neighbor_block_option {
        Some(neighbor_block) => {
            if let BlockType::Air = neighbor_block.block_type {
                vertices.extend_from_slice(&get_mesh_texture_and_pos(
                    face,
                    &block.block_type,
                    pos,
                    grass_above,
                ));
                render = true;
            }
            //otherwise the neighboring block is a solid block so you don't need to render
        }
        None => {
            match neighbor_chunk_block_option {
                Some(neighbor_chunk_block) => {
                    if let BlockType::Air = neighbor_chunk_block.block_type {
                        vertices.extend_from_slice(&get_mesh_texture_and_pos(
                            face,
                            &block.block_type,
                            pos,
                            grass_above,
                        ));
                        render = true;
                    }
                    //otherwise the neighboring chunk's block is a solid block so you don't need to render
                }
                None => {}
            }
        }
    }
    if render {
        indices.push(base_index + 3);
        indices.push(base_index + 2);
        indices.push(base_index);
        indices.push(base_index + 1);
        indices.push(base_index + 2);
        indices.push(base_index + 3);
    }
}
fn get_mesh_texture_and_pos(
    face: Face,
    block_type: &BlockType,
    pos: [f32; 3],
    grass_above: bool,
) -> Vec<ModelVertex> {
    let vertices = match face {
        Face::Top => [
            [pos[0] - 0.5, pos[1] + 0.5, pos[2] - 0.5],
            [pos[0] + 0.5, pos[1] + 0.5, pos[2] + 0.5],
            [pos[0] + 0.5, pos[1] + 0.5, pos[2] - 0.5],
            [pos[0] - 0.5, pos[1] + 0.5, pos[2] + 0.5],
        ],
        Face::Bottom => [
            [pos[0] + 0.5, pos[1] - 0.5, pos[2] - 0.5],
            [pos[0] - 0.5, pos[1] - 0.5, pos[2] + 0.5],
            [pos[0] - 0.5, pos[1] - 0.5, pos[2] - 0.5],
            [pos[0] + 0.5, pos[1] - 0.5, pos[2] + 0.5],
        ],
        Face::Left => [
            [pos[0] - 0.5, pos[1] - 0.5, pos[2] + 0.5],
            [pos[0] - 0.5, pos[1] + 0.5, pos[2] - 0.5],
            [pos[0] - 0.5, pos[1] - 0.5, pos[2] - 0.5],
            [pos[0] - 0.5, pos[1] + 0.5, pos[2] + 0.5],
        ],
        Face::Right => [
            [pos[0] + 0.5, pos[1] - 0.5, pos[2] - 0.5],
            [pos[0] + 0.5, pos[1] + 0.5, pos[2] + 0.5],
            [pos[0] + 0.5, pos[1] - 0.5, pos[2] + 0.5],
            [pos[0] + 0.5, pos[1] + 0.5, pos[2] - 0.5],
        ],
        Face::Front => [
            [pos[0] + 0.5, pos[1] - 0.5, pos[2] + 0.5],
            [pos[0] - 0.5, pos[1] + 0.5, pos[2] + 0.5],
            [pos[0] - 0.5, pos[1] - 0.5, pos[2] + 0.5],
            [pos[0] + 0.5, pos[1] + 0.5, pos[2] + 0.5],
        ],
        Face::Back => [
            [pos[0] - 0.5, pos[1] - 0.5, pos[2] - 0.5],
            [pos[0] + 0.5, pos[1] + 0.5, pos[2] - 0.5],
            [pos[0] + 0.5, pos[1] - 0.5, pos[2] - 0.5],
            [pos[0] - 0.5, pos[1] + 0.5, pos[2] - 0.5],
        ],
    };
    let index = match block_type {
        BlockType::Grass => match face {
            Face::Left | Face::Right | Face::Back | Face::Front => {
                if grass_above {
                    1
                } else {
                    2
                }
            }
            Face::Top => 3,
            Face::Bottom => 1,
        },
        _ => todo!(),
    };

    let texture_coords = get_texture_coords(index);
    let mut vertices_array = vec![];
    for i in 0..4 {
        vertices_array.push(ModelVertex {
            position: vertices[i],
            tex_coords: texture_coords[i],
            normal: [0., 0., 0.],
        })
    }

    vertices_array
}
fn get_texture_coords(index: usize) -> [[f32; 2]; 4] {
    const NUM_SPRITES_IN_TEXTURE: usize = 16; //must be perfect square
    const SPRITE_SIZE: f32 = 1.0 / (NUM_SPRITES_IN_TEXTURE as f32);

    let row = index / NUM_SPRITES_IN_TEXTURE;
    let col = index % NUM_SPRITES_IN_TEXTURE;

    let min_x = col as f32 * SPRITE_SIZE;
    let max_x = min_x + SPRITE_SIZE;
    let min_y = row as f32 * SPRITE_SIZE;
    let max_y = min_y + SPRITE_SIZE;
    [
        [min_x, min_y],
        [max_x, max_y],
        [min_x, max_y],
        [max_x, min_y],
    ]
}

fn main() {
    pollster::block_on(run());
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    // State::new uses async code, so we're going to wait for it to finish
    let (state, event_loop) = State::new(true,env!("OUT_DIR")).await;
    //add models
    create_terrain(&state);
    //render loop
    run_event_loop(state, event_loop, update, keyboard_input);
}
fn update(state: &mut State) {
    
}
fn keyboard_input(state: &mut State, event: &KeyboardInput) {
    
}
fn create_terrain(state: &State) {
    //gen chunks
    for i in 0..256 {
        let row = (i / 16) * 16;
        let col = (i % 16) * 16;
        chunk_blocks_vec.push(chunk_gen(1, row, col));
    }
    //gen meshes
    for i in 0..256 {
        let row = (i / 16) * 16;
        let col = (i % 16) * 16;
        let blocks = &chunk_blocks_vec[i];
        let mesh = build_chunk(&mut state,
            blocks,
            row as f32,
            col as f32,
            match i.checked_sub(16) {
                //actually front
                Some(j) => chunk_blocks_vec.get(j),
                None => None,
            },
            match i.checked_add(16) {
                //actually back
                Some(j) => chunk_blocks_vec.get(j),
                None => None,
            },
            if (i + 1) % 16 == 0 {
                //actually left
                None
            } else {
                chunk_blocks_vec.get(i + 1)
            },
            match i.checked_sub(1) {
                Some(j) => {
                    if j % 16 == 15 {
                        //actually right
                        None
                    } else {
                        chunk_blocks_vec.get(j)
                    }
                }
                None => None,
            },
        );
        chunk_mesh_vec.push(mesh);
    }
    for _ in 0..256 {
        chunks.push(Chunk {
            blocks: chunk_blocks_vec.remove(0),
            mesh: chunk_mesh_vec.remove(0),
        }) //always takes out the first element
    }
}
fn chunk_gen(seed: u32, row: i32, col: i32) -> Vec<Vec<Vec<Block>>> {
    let mut test_blocks = vec![];
    let perlin = Perlin::new(seed);
    let x_scale = 0.03;
    let z_scale = 0.03;
    for x in 0..16 {
        //front back
        let mut vec1 = vec![];
        for z in 0..16 {
            //left right
            let mut vec2 = vec![];
            let noise_value =
                (perlin.get([(x + row) as f64 * x_scale, (z + col) as f64 * z_scale]) + 2.0) * 10.0;
            for y in 0..30 {
                //up down
                let block_type = if y < (noise_value) as usize {
                    BlockType::Grass
                } else {
                    BlockType::Air
                };

                vec2.push(Block::new(block_type));
            }
            vec1.push(vec2);
        }

        test_blocks.push(flip_2d_vector(vec1));
    }
    test_blocks
}
fn flip_2d_vector(input: Vec<Vec<Block>>) -> Vec<Vec<Block>> {
    if input.is_empty() {
        return Vec::new();
    }

    let num_rows = input.len();
    let num_cols = input[0].len();

    let mut flipped = vec![
        vec![
            Block::default();
            num_rows
        ];
        num_cols
    ];

    for i in 0..num_rows {
        for j in 0..num_cols {
            flipped[j][i] = input[i][j];
        }
    }

    flipped
}