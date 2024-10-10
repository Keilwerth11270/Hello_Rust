use std::io::{self, Write};

/*

#[derive(Clone, Copy, PartialEq)] is an attribute that automatically implements
the Clon, Copy, and PartialEq traits for the TemperatureScale enum.

Clone allows us to create a deep copy of the enum

Copy indicates that the type can be copied by simply copying bits (no special behavior needed)

PartialEq allows us to compare two instances of the enum for equality (==)

The TemperatureScale are represented in binary so Clone and Copy can be used without a doubt,
but I don't actually use them in my code. I just added the tags for practice and knowledge.
However, I do use PartialEq  when comparing temperature scales.

#[repr(u8)] is a representation hint to the compiler telling it to represent this enum
using a single byte (u8) where a byte is comprised of 8 bits in memory (0 or 1). This optimizes
memory usage and access speed.

*/

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
enum TemperatureScale {
    Celsius = 0b00,    // integer value 0
    Fahrenheit = 0b01, // integer value 1
    Kelvin = 0b10,     // integer value 2 but i'm cool and want to practice so we're using binary
}

/*

We explicitly assign values to enum variants using binary notation. I want to have consistent
memory usage and also in my mind I have an idea for how I want memory to look. I will give a basic
diagram. It's important to note that even though the enum value for each TemperatureScale is only
2 bits, Rust will still use a full byte to store these values internally. There is a tradeoff 
between memory usage and simplicity/speed of access. Using a full byte allows for faster access
and simpler memory operations at the cost of some unused bits. 

This is because apparently most modern processors are optimized for byte aligned memory access,
meaning they can read and write whole bytes or larger chunks like 4 or 8 bytes at once.
Accessing specific bits within a byte would require additional operations such as bitwise shifts
or masking, which introduces more computational overhead. Accessing memory at non-byte-aligned
boundaries can result in more complex memory operations, leading to cashe misses or slower
memory reads and writes. Basically, though our enums are only 2 bits, Rust will add some
unused bits as padding to make them complete bytes to align them such that the CPU will be
more fast and efficient, which we all love.

Memory layout of Temperature struct:
 +--------------------------------+------------------------+
 |             value              |    scale   | padding   |
 |             (f32)              |   (2 bits) | (6 bits)  |
 |          [32 bits]             | [2 bits]   | [6 bits]  |
 +--------------------------------+------------------------+
 | 31                           0 | 33      32 | 39     34 |
 +--------------------------------+------------------------+
                4 bytes                     1 byte

Also note that in this diagram, I only have one scale at 2 bits and then padding. I got lazy.
For the next scale, it would be to the right of the first scale's padding, and it would be
2 bits, and then there would be 6 bits of padding, and then another 2 bits and the last 6 bits.
*/

struct Temperature {
    value: f32,
    scale: TemperatureScale,
}

/*

The f32 type in Rust represents a 32-bit floating-point number, which provides a good balance 
between precision and memory usage for representing decimal values like temperatures. We use a 
Temperature struct in this conversion program to encapsulate both the numeric value (f32) and 
its associated scale (TemperatureScale enum) in a single, cohesive unit. This approach allows 
us to easily pass around complete temperature information (value and scale together) throughout 
the program, ensuring that we always know not just how hot or cold something is, but also which 
scale that measurement is in. This structure helps prevent errors that could arise from 
mismatching values and scales, and it makes our code more organized and easier to understand.

*/

impl Temperature {

    /*
	#[inline(always)] is a hint to the compiler to always inline this function.

	Inlining replaces the function call with the function's body at the call site.
	For example, if we have:
	
	fn add(a: i32, b: i32) -> i32 { a + b }
		let result = add(5, 3);
	
	Inlining would transform this to:
	
	let result = 5 + 3;
	
	This can improve performance by reducing function call overhead,
	especially for small, frequently called functions.

	However, inlining is not always beneficial. For example:
	1. Large functions: Inlining a large function can increase code size significantly,
	potentially hurting instruction cache performance.
	2. Recursive functions: Inlining recursive functions can lead to infinite expansion.
	3. Functions with complex control flow: These might not benefit from inlining.
	
	An example where inlining might cause issues:
	
	#[inline(always)]
	fn recursive_factorial(n: u64) -> u64 {
		if n == 0 { 1 } else { n * recursive_factorial(n - 1) }
	}
	
	This could potentially cause the compiler to enter an infinite loop trying to inline
	the recursive calls, or could dramatically increase compile times and binary size.

	In my case, `new` is a simple function that's likely to be called frequently,
	making it a good candidate for inlining.
  */  
    #[inline(always)]
    fn new(value: f32, scale: TemperatureScale) -> Self {
        Temperature { value, scale }
    }
