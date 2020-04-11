/// TODO
/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    let mut moves = Vec::new();

    if num_discs % 2 == 0 {
        for 0..(2.pow(n) - 1) {
            moves.push(Move(Peg::A, Peg::B));
            moves.push(Move(Peg::A, Peg::C));
            moves.push(Move(Peg::B, Peg::C));
        }
    } else {
        for 0..(2.pow(n) - 1) {
            moves.push(Move(Peg::A, Peg::B));
            moves.push(Move(Peg::A, Peg::C));
            moves.push(Move(Peg::B, Peg::C));
        }
    }
    // TODO
    unimplemented!();
}
