# Beginning Rust: Get Started with Rust 2021 Edition
### Book Author: Carlo Milanesi

DOI: https://link.springer.com/book/10.1007/978-1-4842-7208-4
<br />
Notes were taken by Thura. <br />
You can try Rust using this playground online compiler: https://play.rust-lang.org/

- [Chapter 1: Getting Started](#Chapter-1)
- [Chapter 2: Doing Arithmetic and Writing More Code](#Chapter-2)
- [Chapter 3: Naming Objects](#Chapter-3)
- [Chapter 4: Controlling Executive Flow](#Chapter-4)
- [Chapter 5: Using Data Sequences](#Chapter-5)
- [Chapter 6: Using Primitive Types](#Chapter-6)
- [Chapter 7: Enumeration and Matching](#Chapter-7)
- [Chapter 8: Using Heterogeneous Data Structures](#Chapter-8)
  
## Chapter 1
The source string 000140 is converted to the binary format.
```
// This is comment.
/* This is
multiline comment */
print!("My number: {}", 000140);
```
## Chapter 2
By putting a dot after a literal number, it is transformed into a literal floating-point number.
```
print!("{}", (23. - 6.) % 5. + 20. * 30. / (3. + 4.));
// 87.71428571428571.
```
println!() will go new line.
```
print!("{} + ", 80);
print!("{} =", 34);
println!(" {}", 80 + 34);
// 80 + 34
// = 114.
```
By adding a backslash character (“\”) inside a literal string, just before the end of a line, the resulting string will contain neither that end-of-line character nor the following spaces; therefore, the leading spaces of the next line are omitted.
```
fn main() {
 println!("{}", "These
 are
 three lines");
}

fn main() {
 println!("{}", "This \
 is \
 just one line");
}
```
## Chapter 3
```
let number = 12;
let other_number = 53;
print!("{}", number + other_number);
```
A memory space shall be reserved to initially contain the value 12, and from now on, every time we use the word number, we will mean such memory space.
```
let mut number = 12;
print!("{}", number);
number = 53;
print!(" {}", number);
```
- That assignment does not allocate objects. It just modifies the value of an already allocated object.
- mut is mutable - needed to declare a variable
whose value can be changed.
- The simple keyword "let" declares an immutable variable, whereas the sequence "let mut" is required to declare a mutable variable.
- Compiler’s default behavior is to print warnings when some mutable variables are never changed.
  
```
let number1; 
let number2 = 22;
number1 = number2;
print!("{}", number1);
```
- First assignment is called initialization.
- Forther assignments are called reassignments.
  
```
let _number = 12;
print!("{}", _number);
```
_ underscore is the don’t-care symbol, that in this situation creates a ***throwaway*** variable. Mostly used for temporary assignments for example in the loop.
- You can used underscores in variables for better readibility, for example, 1_000_000 instead of 1000000.
```
let _n = 3_4.5;
println!("{}",_n);
// 34.5
```

Relational operators for Boolean
```
• ==: is equal to
• !=: is different from
• <: is less than
• <=: is less than or equal to
• >: is greater than
• >=: is greater than or equal to
• "!" read “not”
• "&&" read “logical-and”
• "||" read “logical-or”
```

```
print!("{}", true || true && ! true);
// true
```
Rust has ***Type Consistency*** in assignments.
```
let mut n = 1;
print!("{}", n); // 1
n = 2;
print!(" {}", n); // 2
n = 3;
print!(" {}", n); // 3
n = 3.14; // error: mismatched types
```
Redeclaration introduces a new variable that shadows the first one.
```
let mut n = 1;
print!("{}", n);
n = 2;
print!(" {}", n);
let n = 3.14;
print!(" {}", n);
// 1 2 3.14
```
Expressions like (a = a+1) may be abbreviated
```
let mut a = 12;
a += 1;
a -= 4;
a *= 7;
a /= 6;
```
Using third-party libraries, every Rust installation provides an official library, the so-called standard library. You can use other third-party libraries too.
```
print!("{} {}", str::len("abcde"), "abcde".len());
// 5 5
```
- str is the library for string processing and len() is a function built in str library.
- Use in this format ***lib::func*** for example, ***str::len("abc")***

## Chapter 4
Here is the **control flow** example in Rust syntax.
```
let n = 4;
if n > 1000 {
 print!("big");
}
else if n > 0 {
 print!("small");
}
else if n < 0 {
 print!("negative");
}
else {
 print!("neither positive nor negative");
}
```
**While** loop is called ***conditional loop*** because the looping is based on a ***conditional expression***
```
let mut n = 1;
while n <= 10 {
 print!("{} ", n * n);
 n += 1;
}
//  1 4 9 16 25 36 49 64 81 100 
```
You can also use ***continue*** and ***break*** too.
```
let mut n = 0;
while n < 50 {
 n += 1;
 if n % 3 != 0 {
 if n * n <= 400 {
 print!("{} ", n * n);
 }
 }
}
```
This code is equivalent version using continue and break.
```
let mut n = 0;
while n < 50 {
 n += 1;
 if n % 3 == 0 { continue; }
 if n * n > 400 { break; }
 print!("{} ", n * n);
}
```
**For loop** is called ***counting loop***
```
for n in 1..11 {
 print!("{} ", n * n);
} // 1 4 9 16 25 36 49 64 81 100
```
For loop will be in start..end-1 range. But you can limit the upper bound like this.
```
for n in 1..=10 {
 print!("{} ", n * n);
}
```

## Chapter 5
- **Array** and **Vector** are structured ***sequence data*** in Rust. 
- Cannot contain objects of different types in Array and Vector. Vector of numbers cannot be assigned to a vector of string.
```
let a = [true, false];
let b = [1, 2, 3, 4, 5];
print!("{}, {}.", a.len(), b.len());
//  2, 5.
```
- Unlike vectors, you cannot modify arrays.
```
let _array1: [char; 3] = ['x', 'y', 'z'];
let _array2: [f32; 200] = [0f32; 200];
let _vector1: Vec<char> = vec!['x', 'y', 'z'];
let _vector2: Vec<i32> = vec![0; 5000];
```
- Panic at runtime = stop immediately while compiling
- index out of bounds: the length is 1 but the index is 1.
```
let x = ["a"]; // array of strings
let _y = x[1] // out of range
```
- Attributes: begins with a “#” character followed by an expression in brackets, for example, Deny, Warn, Allow.
```
#[deny(unused_variables)]
let x = 1;
#[warn(unused_variables)]
let y = 2;
#[allow(unused_variables)]
let z = 3;
```
- ***Mutablity*** of array and vector
```
let mut x = ["This", "is", "a", "sentence"];
x[2] = "a nice";
print!("{} {} {} {}.", x[0], x[1], x[2], x[3]);
// This is a nice sentence.

let mut x = vec!["This", "is"]; print!("{}", x.len());
x.push("a"); print!(" {}", x.len())
for i in 0..x.len() { print!(" {}", x[i]); }
// 2 3 4 That is a sentence.
```
- Multidimensional
```
let mut x = [[[23; 4]; 8]; 15];
x[14][7][3] = 56;
print!("{}, {}", x[0][0][0], x[14][7][3]); // 23, 56
```
- The followings are *vector operations* in Rust.
```
let mut x = vec!["This", "is", "a", "sentence"];
for i in 0..x.len() { print!("{} ", x[i]); } println!();
// This is a sentence
x.insert(1, "line");
// This line is a sentence
for i in 0..x.len() { print!("{} ", x[i]); } println!();
x.insert(2, "contains");
// This line contains is a sentence
for i in 0..x.len() { print!("{} ", x[i]); } println!();
x.remove(3);
// This line contains a sentence
for i in 0..x.len() { print!("{} ", x[i]); } println!();
x.push("about Rust");
// This line contains a sentence about Rust
for i in 0..x.len() { print!("{} ", x[i]); } println!();
x.pop();
// This line contains a sentence
for i in 0..x.len() { print!("{} ", x[i]); } println!();
```
The *vector.push(item)*; statement is equivalent to *vector.
insert(vector.len(), item)*;, while the statement *vector.pop()* is equivalent to *vector.remove(vector.len() - 1)*.
- Empty arrays and vectors can be written as follows:
```
let _a = [""; 0];
let _v = vec![0; 0];
```
Copy entire array or vector
```
let mut a1 = vec![4, 56, -2];
let a2 = vec![7, 81, 12500];
println!("{:?} {:?}", a1, a2); // [4, 56, -2] [7, 81, 12500]
a1 = a2;
println!("{:?}", a1); // [7, 81, 12500]
a1[1] = 10;
println!("{:?}", a1); // [7, 10, 12500]
println!("{:?} {:?}", a1, a2); // error
```
- a2 variable hasn’t been simply copied. Actually it has been moved, which means copied and destroyed.
- Unlike arrays, when a vector is assigned to another vector, the original vector exists no more.

## Chapter 6

```
let hexadecimal = 0x10;
let octal = 0o10;
let binary = 0b10;
let mut n = 10;
print!("{} ", n);
n = hexadecimal;
print!("{} ", n);
n = octal;
print!("{} ", n);
n = binary;
print!("{}", n);
//  16 10 8 2
```
Exponential notation (e)
```
let one_thousand = 1e3;
print!("{} ", one_thousand); // 1000
```
### Various Kinds of Number Types
- Represent any number between 0 and 200 using only 8 bits, which is a quarter of 32 bits.
- The larger our objects are, the more cache space they use.
- The i8 type is the smallest, but it can only represent values between -128 and +127.
- The u8 type is unsigned 8-bit integer containing values between 0 and +225.
- If more than the range, error: literal out of range for `i8`.
- Only the usize type is allowed as an index of an array.
- **Cross compilation**, a compiler can generate machine code for a system having a different architecture from the one where the compiler is run. The system for which machine code is generated is named ***target*** (target folder in cargo package).

```
// Type inference
let a = [0];
let i = 0;
print!("{}", a[i]);
```
***Explicit annotation***: “i” is used to initialize “_j”, an operation allowed only to expressions of type u16, it determines that “i” is of such type. 
```
let i = 0;
let _j: u16 = i;
let _k: i16 = i;
```
***Explicit conversion***
```
let a: i16 = 12;
let b: u32 = 4;
let c: f32 = 3.7;
print!("{}", a as i8 + b as i8 + c as i8);
```
Print ASCII codes 
```
print!("{} {} {} {} {}", true as u8, false as u8,
 'A' as u32, 'à' as u32, '€' as u32);
// 1 0 65 224 8364
```

## Chapter 7
- **enum** introduced a new type defined in the program.
- Such type is called ***enumerative*** because it contains a set of items with different data types.
- enum types usually use with ***constructs*** such as **match**.
``` 
enum Continent {
 Europe,
 Asia,
 Africa,
 America,
 Oceania,
}

let contin = Continent::Asia;
// match is like switch in C++
match contin {
 Continent::Europe => print!("E"),
 Continent::Asia => print!("As"),
 Continent::Africa => print!("Af"),
 Continent::America => print!("Am"),
 Continent::Oceania => print!("O"),
} // As
```
- enum forbids all relational operators like "==", ">" and "<".
- ***Underscore in enum*** is used as *default* or *catch-all* case.
```
enum CardinalPoint { North, South, West, East }
let direction = CardinalPoint::South;
match direction {
 CardinalPoint::North => print!("NORTH"),
 CardinalPoint::South => print!("SOUTH"),
 _ => {},
}
```
If variable direction doesn't match with any of enums, it will return null tuple (). Here is another use case of match.
```
let a = 3;
match a {
 3 => print!("three "),
 4 => print!("four "),
 5 => print!("five "),
 _ => print!("other "),
} // three 
```
If variable a is not 3 or 4 or 5, it will print "other " which is a default value. 
<br />
**match** can also be used in the loops as control flow.
```
for n in -2..5 {
 println!("{} is {}.", n, match n {
 0 => "zero",
 1 => "one",
 _ if n < 0 => "negative",
 _ => "plural",
 });
}
// -2 is negative.
// -1 is negative.
// 0 is zero.
// 1 is one.
// 2 is plural.
// 3 is plural.
// 4 is plural.
```
There are other ***constructs*** called **if-let** and **while-let**.
```
enum E {
 Case1(u32),
 Case2(char),
 Case3(i64, bool),
}
```
Using match ...
```
let v = E::Case3(1234, true);
match v {
 E::Case3(n, b) => if b { print!("{}", n) }
 _ => {}
}
```
Using *if-let* ...
```
let v = E::Case3(1234, true);
if let E::Case3(n, b) = v {
 if b { print!("{}", n) }
} 
```
If there is else block and if is unsuccessful, it will continue to the else block. <br />
You can also use *while-let* for conditional loops with enum ...
```
enum E {
 Case1(u32),
 Case2(char),
}
let mut v = E::Case1(0);
while let E::Case1(n) = v {
 print!("{}", n);
 if n == 6 { break; }
 v = E::Case1(n + 1);
} // 0123456
```

## Chapter 8
- Tuples, structs and Tuple-structs are other ***composite*** types.
- These data types can accept different types of data.
**Tuples** are written with parenthesis ( ).
```
let data = (10000000, 183.19, 'Q');
// let data: (i32, f64, char) = (10000000, 183.19, 'Q');
let copy_of_data = data;
print!("{}, {}, {}",
 data.0, copy_of_data.1, data.2);
// 10000000, 183.19, Q
```
**struct** are written with *struct* keyword curly brackets { }.
```
struct SomeData {
 integer: i32,
 fractional: f32,
character: char,
 five_bytes: [u8; 5],
}
let data = SomeData {
 integer: 10_000_000,
 fractional: 183.19,
 character: 'Q',
 five_bytes: [9, 0, 250, 60, 200],
};
print!("{}, {}, {}, {}",
 data.five_bytes[3], data.integer,
 data.fractional, data.character);
// 60, 10000000, 183.19, Q
```
**Tuple-struct** are written with struct keyword and (). <br />
Differing from both tuples and structs, empty tuple-structs are not allowed. Tuple-structs are not widely used actually.
```
struct SomeData (
 i32,
 f32,
 char,
 [u8; 5],
);
let data = SomeData (
 10_000_000,
 183.19,
 'Q',
 [9, 0, 250, 60, 200],
);
print!("{}, {}, {}, {}",
 data.2, data.0, data.1, data.3[2]);
```
***Lexical conventions***
```
enum VehicleKind {
 Motorcycle,
 Car,
 Truck,
}
struct VehicleData {
 kind: VehicleKind,
 registration_year: u16,
 registration_month: u8,
 power: u16,
}
let vehicle = VehicleData {
 kind: VehicleKind::Car,
 registration_year: 2003,
 registration_month: 11,
 power: 120,
};
if vehicle.power > MAXIMUM_POWER {
 println!("Too powerful");
}
```

## Chapter 9
**Functions** are used to ***encapsulate*** the code in a block instead of writing several times.
```
// function definition
fn print_sum(addend1: f64, addend2: f64) {
 println!("{} + {} = {}", addend1, addend2,
 addend1 + addend2);
}
// function invocation
print_sum(3., 5.);
print_sum(3.2, 5.1);
// 3 + 5 = 8
// 3.2 + 5.1 = 8.3
let _x = 3.;
let _y = 5.;
print_sum(x, y);
```
Variable that is passed to the function, but the value of the variable. It is a so-called *pass-by-value*.
