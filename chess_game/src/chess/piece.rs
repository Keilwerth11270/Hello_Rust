//! Chess piece definitions and behaviors.
//! 
//! This file includes:
//! - Enum for different piece types (Pawn, Rook, Knight, etc.)
//! - Traits or methods defining how each piece moves
//! - Logic for special moves (castling, en passant)
//! - Utility functions for piece-related operations

// The #[repr(u8)] attribute tells Rust to represent this enum using an 8-bit unsigned integer.
// This is an optimization that ensures each variant of the enum takes up only 1 byte of memory.
//
// In binary, this looks like:
// Pawn   = 000
// Knight = 001
// Bishop = 010
// Rook   = 011
// Queen  = 100
// King   = 101
#[repr(u8)]
pub enum PieceKind {
    Pawn = 0,   // We assign explicit values to each variant.
    Knight = 1, // This allows us to directly use these values
    Bishop = 2, // when encoding and decoding the piece information.
    Rook = 3,
    Queen = 4,
    King = 5,
}

// Similar to PieceKind, we use #[repr(u8)] to ensure PieceColor only uses 1 byte.
//
// In binary, this looks like:
// White = 0
// Black = 1
#[repr(u8)]
pub enum PieceColor {
    White = 0,
    Black = 1,
}

// This line automatically implements several traits for our Piece struct:
// - Clone: Allows us to create a deep copy of a Piece
// - Copy: Indicates that Piece can be copied by simply copying its bits (no need for deep copy)
// - PartialEq: Allows us to compare Pieces using == and !=
// - Eq: Indicates that == is an equivalence relation (reflexive, symmetric, and transitive)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    // Instead of storing the kind and color as separate fields,
    // we store all the information in a single byte (u8).
    // This significantly reduces the memory usage of each Piece.
    //
    // The byte is structured as follows:
    //Bit position: 7 6 5 4 3 2 1 0
    //              | | | | | | | |
    //              | | | | | +-+-+-- PieceKind (3 bits, values 0-5)
    //              | | | | |
    //              | +-+-+-+-------- Unused (4 bits)
    //              |
    //              +---------------- PieceColor (1 bit, 0 for White, 1 for Black)
    //
    // For example, a white knight would be: 0 0 0 0 0 0 0 1
    //               a black queen would be: 1 0 0 0 0 1 0 0
    data: u8,
}

impl Piece {
    // This method creates a new Piece from a given kind and color.
    pub fn new(kind: PieceKind, color: PieceColor) -> Self {
        Piece {
            // We combine the kind and color into a single byte.
            // The kind uses the 3 least significant bits (0-5 for the 6 piece types).
            // The color uses the 8th bit (0 for white, 1 for black).
            // 
            // We use bitwise OR (|) to combine these:
            // - (kind as u8) gives us the numerical value of the kind (0-5)
            // - ((color as u8) << 7) shifts the color bit to the 8th position
            //   Left shift (<<) by 7 is equivalent to multiplying by 2^7 = 128
            //
            // For example, creating a black knight:
            // kind (Knight) = 001
            // color (Black) = 1
            //
            // (kind as u8)       = 0 0 0 0 0 0 0 1
            // ((color as u8) << 7) = 1 0 0 0 0 0 0 0
            //                        ------------------
            // Bitwise OR result    = 1 0 0 0 0 0 0 1
            data: (kind as u8) | ((color as u8) << 7),
        }
    }

    // This method extracts the kind from the data byte.
    pub fn kind(&self) -> PieceKind {
        // We use bitwise AND (&) with 0b111 (which is 7 in decimal) to keep only
        // the 3 least significant bits, which represent the kind.
        //
        // For example, if we have a black knight (1 0 0 0 0 0 0 1):
        //   1 0 0 0 0 0 0 1  (our data)
        // & 0 0 0 0 0 1 1 1  (0b111)
        //   ---------------
        //   0 0 0 0 0 0 0 1  (result: 1, which corresponds to Knight)
        //
        // The `unsafe` block is used because `transmute` is an unsafe operation.
        // `transmute` reinterprets the bits of one type as another type.
        // It's unsafe because Rust can't guarantee that the conversion is valid.
        // We know it's safe here because we've ensured that the value is always 0-5.
        unsafe { std::mem::transmute(self.data & 0b111) }
    }

    // This method extracts the color from the data byte.
    pub fn color(&self) -> PieceColor {
        // We right-shift (>>) the data by 7 bits to move the color bit
        // to the least significant position. This is equivalent to integer
        // division by 2^7 = 128.
        //
        // For example, if we have a black knight (1 0 0 0 0 0 0 1):
        //   1 0 0 0 0 0 0 1 >> 7
        //   ---------------
        //   0 0 0 0 0 0 0 1  (result: 1, which corresponds to Black)
        //
        // After shifting, the value will be either 0 (White) or 1 (Black),
        // which corresponds to our PieceColor enum values.
        unsafe { std::mem::transmute(self.data >> 7) }
    }
}

// Note on binary notation:
// In Rust, numbers prefixed with 0b are in binary notation, not hexadecimal.
// 0b111 is binary for 7
// 0b1111 is binary for 15
// Hexadecimal notation in Rust uses 0x prefix, e.g., 0xFF for 255.

// This implementation is highly optimized for memory usage. Each Piece
// uses only 1 byte of memory, compared to a more naive implementation
// which might use 2 bytes (1 for kind and 1 for color) or even more
// if using larger integer types.
