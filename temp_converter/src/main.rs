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

    /*
    	#[inline] is a hint to the compiler that this function is a good candidate
    	for inlining, but the compiler may choose not to inline it.
    	This is more flexible than #[inline(always)] as it allows the compiler
    	to make the final decision based on the context where the function is used.
    */

    #[inline]
    fn convert(&self, target_scale: TemperatureScale) -> f32 {

    /*
	These matrices are used to perform temperature conversions efficiently.
	Instead of using if-else statements or match expressions for each
	conversion type, we use these matrices as lookup tables.

	Using matrices for conersion offers several advantages over if-else statements:
	1. Performance: Matrix lookups are typically faster than branching (if-else).
	    This is because it avoids branch prediction misses, which can be costly
	2. Consistency: All conversions are handled uniformly, reducing the chance of errors.
	3. Maintainability: Adding new scales requires just adding new rows/columns to the matrices.
	4. Readability: The conversion logic is centralized and easy to understand at a glance.

	For example, an if-else approach might look like this:
	if self.scale == TemperatureScale::Celsius && target_scale == TemperatureScale::Fahrenheit{
	    return self.value * 1.8 + 32.0;
	} else if ...

	This would require 6 different conditions for all possible conversions, making the code
	longer and potentially slower due to branch prediction misses.

	The matrix approach allows us to perform any conversion with just two array lookups
	and a simple calculation, regardless of the scales involved.
    */

    const CONVERSION_FACTORS: [[f32; 3]; 3] = [
        [1.0, 1.0, 1.00],	//Celcius to X
        [5.0/9.0, 1.0, 5.0/9.0],	// Fahrenheit to X
        [1.0, 1.0, 1.0]];	// Kelvin to X

    const CONVERSION_OFFSETS: [[f32; 3]; 3] = [
        [0.0, 32.0, 273.15],	//Celcius to X
        [-32.0, 0.0, 459.67], 	// Fahrenheit to X
        [-273.15, -459.67, 0.0]]; // Kelvin to X

    if self.scale == target_scale {
        return self.value;
    }

    /*
	We use the enum variants as indices into our matrices.
	'from' represents the row (current scale), 'to' represents the column (target scale).
	This works because we've defined our TemperatureScale enum with explicit values
	that correspond to the matrix indices.
    */

    let from = self.scale as usize;
    let to = target_scale as usize;

    /*
	The conversion formula is:
	Converted Temperature = ((Original Temperature - Offset to Celsius) × Factor from Celsius to Target) + Offset from Celsius to Target
	We look up the appropriate offset and factor from our matrices
	and perform the calculation.
	This single line replaces what would otherwise be multiple if-else statments.
    */
    (self.value - CONVERSION_OFFSETS[from][0]) * CONVERSION_FACTORS[0][to] + CONVERSION_OFFSETS[0][to]
    }
    /*
	(self.value - CONVERSION_OFFSETS[from][0]): This adjusts the value as if we 
	were converting to Celsius, but we don't complete that conversion.
	* CONVERSION_FACTORS[0][to]: This multiplies by the factor that would 
	be used to convert from Celsius to the target scale.
	+ CONVERSION_OFFSETS[0][to]: This adds the offset that would be used when 
	converting from Celsius to the target scale.

	The clever part is that this combination of operations mathematically simplifies 
	to the correct direct conversion between the scales, without actually going 
	through Celsius as an intermediate value.
    */
		
}





























































