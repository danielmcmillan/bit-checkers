mod bit_grid;
use bit_grid::BitGrid;

fn main() {
    let mut bs = BitGrid::new();

    println!("Bitset: {:?}", bs);

    bs.set(3, 7, true);
    println!("Bitset: {:?}", bs);
}
