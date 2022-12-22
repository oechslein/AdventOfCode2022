//#![allow(unused_imports)]
//#![allow(dead_code)]
//#![allow(unused_must_use)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::enum_glob_use,
    clippy::many_single_char_names,
    clippy::must_use_candidate
)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::unreadable_literal)]


#[derive(Debug, Clone)]
pub struct RockStruct<const WIDTH: usize, const SIZE: usize> {
    _x_offset: usize,
    _y_offset: usize,
    _data: [bool; SIZE],
}

impl<const WIDTH: usize, const SIZE: usize> RockStruct<WIDTH, SIZE> {
    fn new(data: [bool; SIZE], x: usize, y: usize) -> Self {
        Self {
            _x_offset: x,
            _y_offset: y,
            _data: data,
        }
    }

    #[inline]
    fn get_width(&self) -> usize {
        WIDTH
    }

    #[inline]
    fn get_height(&self) -> usize {
        SIZE / WIDTH
    }

    fn get(&self, x: usize, y: usize) -> bool {
        if (x >= self._x_offset && x < self.get_width() + self._x_offset)
            && (y >= self._y_offset && y < self.get_height() + self._y_offset)
        {
            return self._data[(y - self._y_offset) * WIDTH + (x - self._x_offset)];
        } else {
            return false;
        }
    }

    fn move_x(&mut self, delta_x: isize) {
        self._x_offset = (self._x_offset as isize + delta_x) as usize;
    }

    fn move_y(&mut self, delta_y: isize) {
        self._y_offset = (self._y_offset as isize + delta_y) as usize;
    }

    fn get_x_min(&self) -> usize {
        self._x_offset
    }

    fn get_x_max(&self) -> usize {
        self._x_offset + self.get_width() - 1
    }

    fn get_y_min(&self) -> usize {
        self._y_offset
    }

    fn get_y_max(&self) -> usize {
        self._y_offset + self.get_height() - 1
    }
}

const _ROCK1_WIDTH: usize = 4;
const _ROCK1_HEIGHT: usize = 1;
const _ROCK1_SIZE: usize = _ROCK1_WIDTH * _ROCK1_HEIGHT;
const _ROCK1_DATA: [bool; _ROCK1_SIZE] = [true, true, true, true];

const _ROCK2_WIDTH: usize = 3;
const _ROCK2_HEIGHT: usize = 3;
const _ROCK2_SIZE: usize = _ROCK2_WIDTH * _ROCK2_HEIGHT;
const _ROCK2_DATA: [bool; _ROCK2_SIZE] = [false, true, false, true, true, true, false, true, false];

const _ROCK3_WIDTH: usize = 3;
const _ROCK3_HEIGHT: usize = 3;
const _ROCK3_SIZE: usize = _ROCK3_WIDTH * _ROCK3_HEIGHT;
const _ROCK3_DATA: [bool; _ROCK3_SIZE] = [true, true, true, false, false, true, false, false, true];

const _ROCK4_WIDTH: usize = 1;
const _ROCK4_HEIGHT: usize = 4;
const _ROCK4_SIZE: usize = _ROCK4_WIDTH * _ROCK4_HEIGHT;
const _ROCK4_DATA: [bool; _ROCK4_SIZE] = [true, true, true, true];

const _ROCK5_WIDTH: usize = 2;
const _ROCK5_HEIGHT: usize = 2;
const _ROCK5_SIZE: usize = _ROCK5_WIDTH * _ROCK5_HEIGHT;
const _ROCK5_DATA: [bool; _ROCK5_SIZE] = [true, true, true, true];

pub enum RockEnum {
    Rock1(RockStruct<_ROCK1_WIDTH, _ROCK1_SIZE>),
    Rock2(RockStruct<_ROCK2_WIDTH, _ROCK2_SIZE>),
    Rock3(RockStruct<_ROCK3_WIDTH, _ROCK3_SIZE>),
    Rock4(RockStruct<_ROCK4_WIDTH, _ROCK4_SIZE>),
    Rock5(RockStruct<_ROCK5_WIDTH, _ROCK5_SIZE>),
}

impl RockEnum {
    pub fn new(rock_type: usize, x: usize, y: usize) -> Self {
        match rock_type {
            0 => RockEnum::Rock1(RockStruct::new(_ROCK1_DATA, x, y)),
            1 => RockEnum::Rock2(RockStruct::new(_ROCK2_DATA, x, y)),
            2 => RockEnum::Rock3(RockStruct::new(_ROCK3_DATA, x, y)),
            3 => RockEnum::Rock4(RockStruct::new(_ROCK4_DATA, x, y)),
            4 => RockEnum::Rock5(RockStruct::new(_ROCK5_DATA, x, y)),
            _ => panic!("rock_number out of range: {}", rock_type),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        match self {
            RockEnum::Rock1(rock) => rock.get(x, y),
            RockEnum::Rock2(rock) => rock.get(x, y),
            RockEnum::Rock3(rock) => rock.get(x, y),
            RockEnum::Rock4(rock) => rock.get(x, y),
            RockEnum::Rock5(rock) => rock.get(x, y),
        }
    }

    pub fn move_x(&mut self, delta_x: isize) {
        match self {
            RockEnum::Rock1(rock) => rock.move_x(delta_x),
            RockEnum::Rock2(rock) => rock.move_x(delta_x),
            RockEnum::Rock3(rock) => rock.move_x(delta_x),
            RockEnum::Rock4(rock) => rock.move_x(delta_x),
            RockEnum::Rock5(rock) => rock.move_x(delta_x),
        }
    }

    pub fn move_y(&mut self, delta_y: isize) {
        match self {
            RockEnum::Rock1(rock) => rock.move_y(delta_y),
            RockEnum::Rock2(rock) => rock.move_y(delta_y),
            RockEnum::Rock3(rock) => rock.move_y(delta_y),
            RockEnum::Rock4(rock) => rock.move_y(delta_y),
            RockEnum::Rock5(rock) => rock.move_y(delta_y),
        }
    }

    pub fn get_x_min(&self) -> usize {
        match self {
            RockEnum::Rock1(rock) => rock.get_x_min(),
            RockEnum::Rock2(rock) => rock.get_x_min(),
            RockEnum::Rock3(rock) => rock.get_x_min(),
            RockEnum::Rock4(rock) => rock.get_x_min(),
            RockEnum::Rock5(rock) => rock.get_x_min(),
        }
    }

    pub fn get_x_max(&self) -> usize {
        match self {
            RockEnum::Rock1(rock) => rock.get_x_max(),
            RockEnum::Rock2(rock) => rock.get_x_max(),
            RockEnum::Rock3(rock) => rock.get_x_max(),
            RockEnum::Rock4(rock) => rock.get_x_max(),
            RockEnum::Rock5(rock) => rock.get_x_max(),
        }
    }

    pub fn get_y_min(&self) -> usize {
        match self {
            RockEnum::Rock1(rock) => rock.get_y_min(),
            RockEnum::Rock2(rock) => rock.get_y_min(),
            RockEnum::Rock3(rock) => rock.get_y_min(),
            RockEnum::Rock4(rock) => rock.get_y_min(),
            RockEnum::Rock5(rock) => rock.get_y_min(),
        }
    }

    pub fn get_y_max(&self) -> usize {
        match self {
            RockEnum::Rock1(rock) => rock.get_y_max(),
            RockEnum::Rock2(rock) => rock.get_y_max(),
            RockEnum::Rock3(rock) => rock.get_y_max(),
            RockEnum::Rock4(rock) => rock.get_y_max(),
            RockEnum::Rock5(rock) => rock.get_y_max(),
        }
    }
}
