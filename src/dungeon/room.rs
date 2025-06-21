use std::collections::HashSet;

use crate::server::block::rotatable::Rotatable;
use crate::{dungeon::{crushers::Crusher, door::Door, room_data::{RoomData, RoomShape, RoomType}, DUNGEON_ORIGIN}, server::{block::{block_pos::BlockPos, blocks::Blocks}, utils::direction::Direction, world::World}};

pub struct Room {
    pub segments: Vec<(usize, usize)>,
    pub room_data: RoomData,
    pub rotation: Direction,

    pub tick_amount: u32,
    pub crushers: Vec<Crusher>,
}

impl Room {

    pub fn new(
        mut segments: Vec<(usize, usize)>,
        dungeon_doors: &Vec<Door>,
        room_data: RoomData
    ) -> Room {
        // Sort room segments by y and then x
        segments.sort_by(|a, b| a.1.cmp(&b.1));
        segments.sort_by(|a, b| a.0.cmp(&b.0));

        let rotation = Room::get_rotation_from_segments(&segments, dungeon_doors);
        let corner_pos = Room::get_corner_pos_from(&segments, &rotation, &room_data);

        let crushers = room_data.crusher_data.iter().map(|data| {
            let mut crusher = Crusher::from_json(data);
            
            crusher.direction = crusher.direction.rotate(rotation);
            crusher.block_pos = crusher.block_pos.rotate(rotation);

            // This is fucking aids
            match rotation {
                Direction::North => match crusher.direction {
                    Direction::East | Direction::West => crusher.block_pos.add_z(crusher.width - 1),
                    _ => crusher.block_pos.add_x(crusher.width - 1),
                },
                Direction::South => match crusher.direction {
                    Direction::East | Direction::West => crusher.block_pos.add_z(-crusher.width + 1),
                    _ => crusher.block_pos.add_x(-crusher.width + 1),
                }
                _ => crusher.block_pos,
            };

            crusher.block_pos = crusher.block_pos
                .add_x(corner_pos.x)
                .add_z(corner_pos.z);

            crusher
        }).collect::<Vec<Crusher>>();

        Room {
            segments,
            room_data,
            rotation,
            tick_amount: 0,
            crushers,
        }
    }

    pub fn get_corner_pos(&self) -> BlockPos {
        Room::get_corner_pos_from(&self.segments, &self.rotation, &self.room_data)
    }

    pub fn get_corner_pos_from(segments: &Vec<(usize, usize)>, rotation: &Direction, room_data: &RoomData) -> BlockPos {
        let min_x = segments.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
        let min_y = segments.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap().1;

        let x = min_x as i32 * 32 + DUNGEON_ORIGIN.0;
        let y = 68;
        let z = min_y as i32 * 32 + DUNGEON_ORIGIN.1;

        // BlockPos { x, y, z }
        match rotation {
            Direction::North => BlockPos { x, y, z },
            Direction::East => BlockPos { x: x + room_data.length - 1, y, z },
            Direction::South => BlockPos { x: x + room_data.length - 1, y, z: z + room_data.width - 1 },
            Direction::West => BlockPos { x: x, y, z: z + room_data.width - 1 },
            _ => unreachable!(),
        }
    }

    pub fn tick(&mut self) {
        self.tick_amount += 1;
    }

    pub fn get_1x1_shape_and_type(segments: &Vec<(usize, usize)>, dungeon_doors: &Vec<Door>) -> (RoomShape, Direction) {
        let center_x = segments[0].0 as i32 * 32 + 15;
        let center_z = segments[0].1 as i32 * 32 + 15;

        // Actual doors found in the world
        let doors_opt = [
            (center_x, center_z - 16),
            (center_x + 16, center_z),
            (center_x, center_z + 16),
            (center_x - 16, center_z)
        ].iter().map(|pos| {
            dungeon_doors.iter()
                .find(|door| door.x == pos.0 && door.z == pos.1)
                .is_some()
        }).collect::<Vec<bool>>();

        let mut num: u8 = 0;
        for i in 0..4 {
            num |= doors_opt[i] as u8;
            if i < 3 {
                num <<= 1;
            }
        }

        // println!("{:04b} {:?}", num, doors_opt);

        match num {
            // Doors on all sides, never changes
            0b1111 => (RoomShape::OneByOneCross, Direction::North),
            // Dead end 1x1
            0b1000 => (RoomShape::OneByOneEnd, Direction::North),
            0b0100 => (RoomShape::OneByOneEnd, Direction::East),
            0b0010 => (RoomShape::OneByOneEnd, Direction::South),
            0b0001 => (RoomShape::OneByOneEnd, Direction::West),
            // Opposite doors
            0b0101 => (RoomShape::OneByOneStraight, Direction::North),
            0b1010 => (RoomShape::OneByOneStraight, Direction::East),
            // L bend
            0b0011 => (RoomShape::OneByOneBend, Direction::North),
            0b1001 => (RoomShape::OneByOneBend, Direction::East),
            0b1100 => (RoomShape::OneByOneBend, Direction::South),
            0b0110 => (RoomShape::OneByOneBend, Direction::West),
            // Triple door
            0b1011 => (RoomShape::OneByOneTriple, Direction::North),
            0b1101 => (RoomShape::OneByOneTriple, Direction::East),
            0b1110 => (RoomShape::OneByOneTriple, Direction::South),
            0b0111 => (RoomShape::OneByOneTriple, Direction::West),
            
            _ => (RoomShape::OneByOne, Direction::North),
        }
    }

    pub fn get_rotation_from_segments(segments: &Vec<(usize, usize)>, dungeon_doors: &Vec<Door>) -> Direction {

        let unique_x = segments.iter()
            .map(|x| x.0)
            .collect::<HashSet<usize>>();

        let unique_z = segments.iter()
            .map(|x| x.1)
            .collect::<HashSet<usize>>();

        let not_long = unique_x.len() > 1 && unique_z.len() > 1;

        match segments.len() {
            1 => {
                let (_, direction) = Room::get_1x1_shape_and_type(segments, dungeon_doors);

                return direction
            },
            2 => match unique_z.len() == 1 {
                true => Direction::North,
                false => Direction::East,
            },
            3 => {  
                // L room
                if not_long {
                    let corner_value = segments.iter().find(|x| {
                        segments.iter().all(|y| {
                            x.0.abs_diff(y.0) + x.1.abs_diff(y.1) <= 1
                        })
                    }).expect(format!("Invalid L room: Segments: {:?}", segments).as_str());

                    let min_x = segments.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
                    let min_y = segments.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap().1;
                    let max_x = segments.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
                    let max_y = segments.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;

                    if corner_value == &(min_x, min_y) {
                        return Direction::East
                    }
                    if corner_value == &(max_x, min_y) {
                        return Direction::South
                    }
                    if corner_value == &(max_x, max_y) {
                        return Direction::West
                    }

                    return Direction::North
                }

                match unique_z.len() == 1 {
                    true => Direction::North,
                    false => Direction::East,
                }
            },
            4 => {
                if unique_x.len() == 2 && unique_z.len() == 2 {
                    return Direction::North
                }

                match unique_z.len() == 1 {
                    true => Direction::North,
                    false => Direction::East,
                }
            },
            _ => unreachable!(),
        }
    }

    fn load_default(&self, world: &mut World) {
        for (x, z) in self.segments.iter() {
            
            // Temporary for room colors, will be changed later on to paste saved room block states
            let block = match self.room_data.room_type {
                RoomType::Normal => Blocks::Stone { variant: 0 },
                RoomType::Blood => Blocks::Stone { variant: 0 },
                RoomType::Entrance => Blocks::Stone { variant: 0 },
                RoomType::Fairy => Blocks::Stone { variant: 0 },
                RoomType::Trap => Blocks::Stone { variant: 0 },
                RoomType::Yellow => Blocks::Stone { variant: 0 },
                RoomType::Puzzle => Blocks::Stone { variant: 0 },
                RoomType::Rare => Blocks::Stone { variant: 0 },
            };

            world.fill_blocks(
                block,
                (
                    *x as i32 * 32 + DUNGEON_ORIGIN.0,
                    self.room_data.bottom,
                    *z as i32 * 32 + DUNGEON_ORIGIN.1,
                ),
                (
                    *x as i32 * 32 + DUNGEON_ORIGIN.0 + 30,
                    self.room_data.bottom,
                    *z as i32 * 32 + DUNGEON_ORIGIN.1 + 30,
                )
            );

            // Merge to the side
            if self.segments.contains(&(x+1, *z)) {
                world.fill_blocks(
                    block,
                    (
                        *x as i32 * 32 + 31 + DUNGEON_ORIGIN.0,
                        self.room_data.bottom,
                        *z as i32 * 32 + DUNGEON_ORIGIN.1,
                    ),
                    (
                        *x as i32 * 32 + 31 + DUNGEON_ORIGIN.0,
                        self.room_data.bottom,
                        *z as i32 * 32 + DUNGEON_ORIGIN.1 + 30,
                    )
                );
            }
            
            // // Merge below
            if self.segments.contains(&(*x, z+1)) {
                world.fill_blocks(
                    block,
                    (
                        *x as i32 * 32 + DUNGEON_ORIGIN.0,
                        self.room_data.bottom,
                        *z as i32 * 32 + 31 + DUNGEON_ORIGIN.1,
                    ),
                    (
                        *x as i32 * 32 + DUNGEON_ORIGIN.0 + 30,
                        self.room_data.bottom,
                        *z as i32 * 32 + 31 + DUNGEON_ORIGIN.1 + 30,
                    )
                );
            }
        }
    }

    pub fn load_into_world(&self, world: &mut World) {
        if self.room_data.block_data.len() == 0 {
            self.load_default(world);
            return;
        }
        // self.load_default(world);
        // return;

        let corner = self.get_corner_pos();

        for (i, block) in self.room_data.block_data.iter().enumerate() {
            if *block == Blocks::Air {
                continue;
            }
            // not sure if editing room data might ruin something,
            // so to be safe im just cloning it
            let mut block = block.clone();
            block.rotate(self.rotation);

            let ind = i as i32;

            let x = ind % self.room_data.width;
            let z = (ind / self.room_data.width) % self.room_data.length;
            let y = self.room_data.bottom + ind / (self.room_data.width * self.room_data.length);

            let bp = BlockPos { x, y, z }.rotate(self.rotation);

            world.set_block_at(block, corner.x + bp.x, y, corner.z + bp.z);
        }

    }
}

