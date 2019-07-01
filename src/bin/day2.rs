use std::convert::TryFrom;
use std::error::Error;
use std::fs;
use std::iter;
use std::num::ParseIntError;

const FILENAME: &str = "inputs/day2.txt";

/// Presents are perfect right rectangular prisms, with fields `length`, `width` and `height`.
struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    /// Create an iterator over the sides.
    fn sides(&self) -> impl Iterator<Item = u32> {
        iter::once(self.length).chain(iter::once(self.width).chain(iter::once(self.height)))
    }

    /// Compute the surface area of a present.
    fn surface_area(&self) -> u32 {
        2 * (self.length * self.width + self.length * self.height + self.width * self.height)
    }

    /// Compute the surface area of the smallest side.
    fn smallest_side_area(&self) -> u32 {
        self.volume() / self.sides().max().unwrap()
    }

    /// Compute the minimal amount of wrapping paper required to wrap a present, which is the
    /// surface area of the present plus the area of the smallest side (a bit extra).
    fn wrapping_paper(&self) -> u32 {
        self.surface_area() + self.smallest_side_area()
    }

    /// Compute the volume of the present.
    fn volume(&self) -> u32 {
        self.sides().product()
    }

    /// Compute the minimal amount of ribbon required to wrap a present, which is the shortest
    /// distance around its sides, plus a bow whose length is equal to the volume of the present.
    fn ribbon(&self) -> u32 {
        let d: u32 = self.sides().sum::<u32>() - self.sides().max().unwrap();
        2 * d + self.volume()
    }
}

impl TryFrom<&str> for Present {
    type Error = ParseIntError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut dims = input.split('x');

        Ok(Present {
            length: dims.next().unwrap_or("").parse()?,
            width: dims.next().unwrap_or("").parse()?,
            height: dims.next().unwrap_or("").parse()?,
        })
    }
}

/// The elves are running low on wrapping paper, and so they need to submit an order for more.
/// They have a list of the dimensions (length l, width w, and height h) of each present, and
/// only want to order exactly as much as they need.
fn main() -> std::result::Result<(), Box<Error>> {
    let presents: Vec<Present> = fs::read_to_string(FILENAME)?
        .lines()
        .map(Present::try_from)
        .collect::<Result<Vec<Present>, ParseIntError>>()?;

    part1(&presents);
    part2(&presents);

    Ok(())
}

/// All numbers in the elves' list are in feet. How many total square feet of wrapping paper
/// should they order?
fn part1(presents: &[Present]) {
    let result: u32 = presents.iter().map(Present::wrapping_paper).sum();
    println!("The first answer is: {}", result);
}

/// How many total feet of ribbon should they order?
fn part2(presents: &[Present]) {
    let result: u32 = presents.iter().map(Present::ribbon).sum();
    println!("The second answer is: {}", result);
}
