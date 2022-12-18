use std::fmt::Display;

use crate::WIDTH;

pub enum RockEnum {
    Rock1(Rock1),
    Rock2(Rock2),
    Rock3(Rock3),
    Rock4(Rock4),
    Rock5(Rock5),
}

impl RockEnum {
    pub fn new(rock_type: usize, z_min: usize) -> Self {
        match rock_type {
            0 => RockEnum::Rock1(Rock1::new(z_min)),
            1 => RockEnum::Rock2(Rock2::new(z_min)),
            2 => RockEnum::Rock3(Rock3::new(z_min)),
            3 => RockEnum::Rock4(Rock4::new(z_min)),
            4 => RockEnum::Rock5(Rock5::new(z_min)),
            _ => panic!("rock_number out of range: {}", rock_type),
        }
    }

    pub fn get_total_z_height_min(&self) -> usize {
        (0..WIDTH)
            .filter_map(|x| self.get_z_height_min(x))
            .min()
            .unwrap()
    }
    pub fn get_total_z_height_max(&self) -> usize {
        (0..WIDTH)
            .filter_map(|x| self.get_z_height_max(x))
            .max()
            .unwrap()
    }

    pub fn get_z_height_min(&self, x: usize) -> Option<usize> {
        match self {
            RockEnum::Rock1(rock) => rock.get_z_height_min(x),
            RockEnum::Rock2(rock) => rock.get_z_height_min(x),
            RockEnum::Rock3(rock) => rock.get_z_height_min(x),
            RockEnum::Rock4(rock) => rock.get_z_height_min(x),
            RockEnum::Rock5(rock) => rock.get_z_height_min(x),
        }
    }

    pub fn get_z_height_max(&self, x: usize) -> Option<usize> {
        match self {
            RockEnum::Rock1(rock) => rock.get_z_height_max(x),
            RockEnum::Rock2(rock) => rock.get_z_height_max(x),
            RockEnum::Rock3(rock) => rock.get_z_height_max(x),
            RockEnum::Rock4(rock) => rock.get_z_height_max(x),
            RockEnum::Rock5(rock) => rock.get_z_height_max(x),
        }
    }

    pub fn push_left(&mut self) {
        match self {
            RockEnum::Rock1(rock) => rock.push_left(),
            RockEnum::Rock2(rock) => rock.push_left(),
            RockEnum::Rock3(rock) => rock.push_left(),
            RockEnum::Rock4(rock) => rock.push_left(),
            RockEnum::Rock5(rock) => rock.push_left(),
        }
    }

    pub fn push_right(&mut self) {
        match self {
            RockEnum::Rock1(rock) => rock.push_right(),
            RockEnum::Rock2(rock) => rock.push_right(),
            RockEnum::Rock3(rock) => rock.push_right(),
            RockEnum::Rock4(rock) => rock.push_right(),
            RockEnum::Rock5(rock) => rock.push_right(),
        }
    }

    pub fn do_fall(&mut self) {
        match self {
            RockEnum::Rock1(rock) => rock.do_fall(),
            RockEnum::Rock2(rock) => rock.do_fall(),
            RockEnum::Rock3(rock) => rock.do_fall(),
            RockEnum::Rock4(rock) => rock.do_fall(),
            RockEnum::Rock5(rock) => rock.do_fall(),
        }
    }
}

pub trait Rock: std::fmt::Debug + Clone + Sized {
    fn new(z_min: usize) -> Self;
    fn get_z_height_min(&self, x: usize) -> Option<usize>;
    fn get_z_height_max(&self, x: usize) -> Option<usize>;
    fn push_left(&mut self);
    fn push_right(&mut self);
    fn do_fall(&mut self);
}

// ####
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Rock1 {
    z_min: usize,
    x: usize, // leftmost x
}

impl Display for Rock1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.x - 1 {
            write!(f, " ")?;
        }
        write!(f, "####")
    }
}

impl Rock for Rock1 {
    fn new(z_min: usize) -> Self {
        Self { z_min, x: 2 }
    }
    fn get_z_height_min(&self, x: usize) -> Option<usize> {
        if x < self.x || x >= self.x + 4 {
            return None;
        } else {
            return Some(self.z_min);
        }
    }

    fn get_z_height_max(&self, x: usize) -> Option<usize> {
        if x < self.x || x >= self.x + 4 {
            return None;
        } else {
            return Some(self.z_min);
        }
    }

    fn push_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    fn push_right(&mut self) {
        if self.x + 4 < WIDTH {
            self.x += 1;
        }
    }

    fn do_fall(&mut self) {
        self.z_min -= 1;
    }
}

//.#.
//###
//.#.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Rock2 {
    z_min: usize,
    x: usize, // center x
}

impl Rock for Rock2 {
    fn new(z_min: usize) -> Self {
        Self { z_min, x: 3 }
    }
    fn get_z_height_min(&self, x: usize) -> Option<usize> {
        match x {
            _x if _x == self.x - 1 => Some(self.z_min + 1),
            _x if _x == self.x => Some(self.z_min),
            _x if _x == self.x + 1 => Some(self.z_min + 1),
            _ => None,
        }
    }

    fn get_z_height_max(&self, x: usize) -> Option<usize> {
        match x {
            _x if _x == self.x - 1 => Some(self.z_min + 1),
            _x if _x == self.x => Some(self.z_min + 2),
            _x if _x == self.x + 1 => Some(self.z_min + 1),
            _ => None,
        }
    }

    fn push_left(&mut self) {
        if self.x > 1 {
            self.x -= 1;
        }
    }

    fn push_right(&mut self) {
        if self.x + 3 < WIDTH {
            self.x += 1;
        }
    }

    fn do_fall(&mut self) {
        self.z_min -= 1;
    }
}

//..#
//..#
//###
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Rock3 {
    z_min: usize,
    x: usize, // leftmost x
}

impl Rock for Rock3 {
    fn new(z_min: usize) -> Self {
        Self { z_min, x: 2 }
    }
    fn get_z_height_min(&self, x: usize) -> Option<usize> {
        match x {
            _x if _x == self.x => Some(self.z_min),
            _x if _x == self.x + 1 => Some(self.z_min),
            _x if _x == self.x + 2 => Some(self.z_min),
            _ => None,
        }
    }

    fn get_z_height_max(&self, x: usize) -> Option<usize> {
        match x {
            _x if _x == self.x => Some(self.z_min),
            _x if _x == self.x + 1 => Some(self.z_min),
            _x if _x == self.x + 2 => Some(self.z_min + 2),
            _ => None,
        }
    }

    fn push_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    fn push_right(&mut self) {
        if self.x + 3 < WIDTH {
            self.x += 1;
        }
    }

    fn do_fall(&mut self) {
        self.z_min -= 1;
    }
}

//#
//#
//#
//#
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Rock4 {
    z_min: usize,
    x: usize,
}

impl Rock for Rock4 {
    fn new(z_min: usize) -> Self {
        Self { z_min, x: 2 }
    }
    fn get_z_height_min(&self, x: usize) -> Option<usize> {
        match x {
            _x if _x == self.x => Some(self.z_min),
            _ => None,
        }
    }

    fn get_z_height_max(&self, x: usize) -> Option<usize> {
        match x {
            _x if _x == self.x => Some(self.z_min + 3),
            _ => None,
        }
    }

    fn push_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    fn push_right(&mut self) {
        if self.x + 1 < WIDTH {
            self.x += 1;
        }
    }

    fn do_fall(&mut self) {
        self.z_min -= 1;
    }
}

//##
//##
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Rock5 {
    z_min: usize,
    x: usize, // leftmost x
}

impl Rock for Rock5 {
    fn new(z_min: usize) -> Self {
        Self { z_min, x: 2 }
    }
    fn get_z_height_min(&self, x: usize) -> Option<usize> {
        match x {
            _x if _x == self.x => Some(self.z_min),
            _x if _x == self.x + 1 => Some(self.z_min),
            _ => None,
        }
    }

    fn get_z_height_max(&self, x: usize) -> Option<usize> {
        match x {
            _x if _x == self.x => Some(self.z_min + 1),
            _x if _x == self.x + 1 => Some(self.z_min + 1),
            _ => None,
        }
    }

    fn push_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    fn push_right(&mut self) {
        if self.x + 2 < WIDTH {
            self.x += 1;
        }
    }

    fn do_fall(&mut self) {
        self.z_min -= 1;
    }
}
