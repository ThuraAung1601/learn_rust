- [Chapter 1](#Chapter-1)
- [Chapter 2](#Chapter-2)
- [Chapter 3](#Chapter-3)
- [Chapter 4](#Chapter-4)
- [Chapter 5](#Chapter-5)
- 

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
- Use in this format ***lib::func** for example, ***str::len("abc")**

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
- **Array**, **Tuple** and **Vector** are structured ***sequence data*** in Rust. 
- Cannot contain objects of different types in Array and Vector.
```
let a = [true, false];
let b = [1, 2, 3, 4, 5];
print!("{}, {}.", a.len(), b.len());
//  2, 5.
```
