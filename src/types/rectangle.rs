use bevy_fortress::prelude::*;

pub struct Rectangle {
    pub start: Coord,
    pub size: Size,
}

impl Rectangle {
    pub fn new(start: Coord, size: Size) -> Self { Self { start, size } }

    pub fn x_min(&self) -> i32 { self.start.x }

    pub fn x_max(&self) -> i32 { self.start.x + self.size.width() as i32 }

    pub fn y_min(&self) -> i32 { self.start.y }

    pub fn y_max(&self) -> i32 { self.start.y + self.size.height() as i32 }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        self.x_min() <= other.x_max() &&
            self.x_max() >= other.x_min() &&
            self.y_min() <= other.y_max() &&
            self.y_max() >= other.y_min()
    }

    pub fn center(&self) -> Coord {
        Coord::new(
            (self.x_min() + self.x_max()) / 2,
            (self.y_min() + self.y_max()) / 2,
        )
    }
}
