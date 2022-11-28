use crate::point::IPoint2;

pub const EAST: IPoint2 = IPoint2 {
    x: 1,
    y: 0,
};

pub const NORTH: IPoint2 = IPoint2 {
    x: 0,
    y: -1,
};

pub const WEST: IPoint2 = IPoint2 {
    x: -1,
    y: 0,
};

pub const SOUTH: IPoint2 = IPoint2 {
    x: 0,
    y: 1,
};