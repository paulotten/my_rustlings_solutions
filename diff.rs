diff --git a/exercises/clippy/clippy1.rs b/exercises/clippy/clippy1.rs
index bdb5dd2..8832feb 100644
--- a/exercises/clippy/clippy1.rs
+++ b/exercises/clippy/clippy1.rs
@@ -6,12 +6,10 @@
 // check clippy's suggestions from the output to solve the exercise.
 // Execute `rustlings hint clippy1` for hints :)
 
-// I AM NOT DONE
-
 fn main() {
     let x = 1.2331f64;
     let y = 1.2332f64;
-    if y != x {
+    if (y - x).abs() > 0.00001f64 {
         println!("Success!");
     }
 }
diff --git a/exercises/clippy/clippy2.rs b/exercises/clippy/clippy2.rs
index 37af9ed..e100369 100644
--- a/exercises/clippy/clippy2.rs
+++ b/exercises/clippy/clippy2.rs
@@ -1,13 +1,11 @@
 // clippy2.rs
 // Make me compile! Execute `rustlings hint clippy2` for hints :)
 
-// I AM NOT DONE
-
 fn main() {
     let mut res = 42;
     let option = Some(12);
-    for x in option {
+    if let Some(x) = option {
         res += x;
-    }
+    };
     println!("{}", res);
 }
diff --git a/exercises/conversions/as_ref_mut.rs b/exercises/conversions/as_ref_mut.rs
index 963c0f2..f1e5072 100644
--- a/exercises/conversions/as_ref_mut.rs
+++ b/exercises/conversions/as_ref_mut.rs
@@ -2,16 +2,15 @@
 // Read more about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html
 // and https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
 
-// I AM NOT DONE
 // Obtain the number of bytes (not characters) in the given argument
 // Add the AsRef trait appropriately as a trait bound
-fn byte_counter<T>(arg: T) -> usize {
+fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
     arg.as_ref().as_bytes().len()
 }
 
 // Obtain the number of characters (not bytes) in the given argument
 // Add the AsRef trait appropriately as a trait bound
-fn char_counter<T>(arg: T) -> usize {
+fn char_counter<T: AsRef<str>>(arg: T) -> usize {
     arg.as_ref().chars().count()
 }
 
diff --git a/exercises/conversions/from_into.rs b/exercises/conversions/from_into.rs
index 8fb9eb0..06155e9 100644
--- a/exercises/conversions/from_into.rs
+++ b/exercises/conversions/from_into.rs
@@ -18,7 +18,6 @@ impl Default for Person {
     }
 }
 
-// I AM NOT DONE
 // Your task is to complete this implementation
 // in order for the line `let p = Person::from("Mark,20")` to compile
 // Please note that you'll need to parse the age component into a `usize`
@@ -35,6 +34,29 @@ impl Default for Person {
 // Otherwise, then return an instantiated Person object with the results
 impl From<&str> for Person {
     fn from(s: &str) -> Person {
+        let parts: Vec<&str> = s.split(',').collect();
+        
+        // two parts
+        if parts.len() != 2 {
+            return Person::default();
+        }
+        
+        // name
+        if parts[0].len() == 0 {
+            return Person::default();
+        }
+        let name = parts[0].to_string();
+        
+        // age
+        if parts[1].len() == 0 {
+            return Person::default();
+        }
+        let age = match parts[1].parse::<usize>() {
+            Ok(age) => age,
+            Err(_) => return Person::default()
+        };
+        
+        Person {name, age}
     }
 }
 
diff --git a/exercises/conversions/from_str.rs b/exercises/conversions/from_str.rs
index 014d054..de25193 100644
--- a/exercises/conversions/from_str.rs
+++ b/exercises/conversions/from_str.rs
@@ -10,7 +10,6 @@ struct Person {
     age: usize,
 }
 
-// I AM NOT DONE
 // Steps:
 // 1. If the length of the provided string is 0, then return an error
 // 2. Split the given string on the commas present in it
@@ -23,6 +22,36 @@ struct Person {
 impl FromStr for Person {
     type Err = String;
     fn from_str(s: &str) -> Result<Person, Self::Err> {
+        // 1. If the length of the provided string is 0, then return an error
+        if s.len() == 0 {
+            return Err("empty string".to_string())
+        }
+        
+        // 2. Split the given string on the commas present in it
+        let parts: Vec<&str> = s.split(",").collect();
+        
+        if parts.len() != 2 {
+            return Err("wrong number of parts".to_string());
+        }
+        
+        // 3. Extract the first element from the split operation and use it as the name
+        let name = parts[0].to_string();
+        
+        // 4. If the name is empty, then return an error
+        if name.len() == 0 {
+            return Err("empty name".to_string());
+        }
+        
+        // 5. Extract the other element from the split operation and parse it into a `usize` as the age
+        //    with something like `"4".parse::<usize>()`.
+        // If while parsing the age, something goes wrong, then return an error
+        let age = match parts[1].parse::<usize>() {
+            Ok(age) => age,
+            Err(_) => return Err("invalid age".to_string())
+        };
+        
+        // Otherwise, then return a Result of a Person object
+        Ok(Person {name, age})
     }
 }
 
diff --git a/exercises/conversions/try_from_into.rs b/exercises/conversions/try_from_into.rs
index dbdbe00..870ab6a 100644
--- a/exercises/conversions/try_from_into.rs
+++ b/exercises/conversions/try_from_into.rs
@@ -11,8 +11,6 @@ struct Color {
     blue: u8,
 }
 
-// I AM NOT DONE
-
 // Your task is to complete this implementation
 // and return an Ok result of inner type Color.
 // You need create implementation for a tuple of three integer,
@@ -26,6 +24,20 @@ struct Color {
 impl TryFrom<(i16, i16, i16)> for Color {
     type Error = String;
     fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
+        let red = match u8::try_from(tuple.0) {
+            Ok(red) => red,
+            Err(_) => return Err("invalid red value".to_string()),
+        };
+        let green = match u8::try_from(tuple.1) {
+            Ok(green) => green,
+            Err(_) => return Err("invalid green value".to_string()),
+        };
+        let blue = match u8::try_from(tuple.2) {
+            Ok(blue) => blue,
+            Err(_) => return Err("invalid blue value".to_string()),
+        };
+        
+        Ok(Color {red, green, blue})
     }
 }
 
@@ -33,6 +45,20 @@ impl TryFrom<(i16, i16, i16)> for Color {
 impl TryFrom<[i16; 3]> for Color {
     type Error = String;
     fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
+        let red = match u8::try_from(arr[0]) {
+            Ok(red) => red,
+            Err(_) => return Err("invalid red value".to_string()),
+        };
+        let green = match u8::try_from(arr[1]) {
+            Ok(green) => green,
+            Err(_) => return Err("invalid green value".to_string()),
+        };
+        let blue = match u8::try_from(arr[2]) {
+            Ok(blue) => blue,
+            Err(_) => return Err("invalid blue value".to_string()),
+        };
+        
+        Ok(Color {red, green, blue})
     }
 }
 
@@ -40,6 +66,24 @@ impl TryFrom<[i16; 3]> for Color {
 impl TryFrom<&[i16]> for Color {
     type Error = String;
     fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
+        if slice.len() != 3 {
+            return Err("invalid slice length".to_string());
+        }
+        
+        let red = match u8::try_from(slice[0]) {
+            Ok(red) => red,
+            Err(_) => return Err("invalid red value".to_string()),
+        };
+        let green = match u8::try_from(slice[1]) {
+            Ok(green) => green,
+            Err(_) => return Err("invalid green value".to_string()),
+        };
+        let blue = match u8::try_from(slice[2]) {
+            Ok(blue) => blue,
+            Err(_) => return Err("invalid blue value".to_string()),
+        };
+        
+        Ok(Color {red, green, blue})
     }
 }
 
diff --git a/exercises/conversions/using_as.rs b/exercises/conversions/using_as.rs
index 54f9651..1d974bd 100644
--- a/exercises/conversions/using_as.rs
+++ b/exercises/conversions/using_as.rs
@@ -2,13 +2,12 @@
 // Please note that the `as` operator is not only used when type casting.
 // It also helps with renaming imports.
 
-// I AM NOT DONE
 // The goal is to make sure that the division does not fail to compile
 fn average(values: &[f64]) -> f64 {
     let total = values
         .iter()
         .fold(0.0, |a, b| a + b);
-    total / values.len()
+    total / values.len() as f64
 }
 
 fn main() {
diff --git a/exercises/enums/enums1.rs b/exercises/enums/enums1.rs
index a2223d3..2baa8f2 100644
--- a/exercises/enums/enums1.rs
+++ b/exercises/enums/enums1.rs
@@ -1,11 +1,13 @@
 // enums1.rs
 // Make me compile! Execute `rustlings hint enums1` for hints!
 
-// I AM NOT DONE
-
 #[derive(Debug)]
 enum Message {
     // TODO: define a few types of messages as used below
+    Quit,
+    Echo,
+    Move,
+    ChangeColor,
 }
 
 fn main() {
diff --git a/exercises/enums/enums2.rs b/exercises/enums/enums2.rs
index 52ccb22..1b6ff07 100644
--- a/exercises/enums/enums2.rs
+++ b/exercises/enums/enums2.rs
@@ -1,11 +1,13 @@
 // enums2.rs
 // Make me compile! Execute `rustlings hint enums2` for hints!
 
-// I AM NOT DONE
-
 #[derive(Debug)]
 enum Message {
     // TODO: define the different variants used below
+    Move {x: i32, y: i32},
+    Echo (String),
+    ChangeColor (i32, i32, i32),
+    Quit,
 }
 
 impl Message {
diff --git a/exercises/enums/enums3.rs b/exercises/enums/enums3.rs
index 4b0be97..a5e3e27 100644
--- a/exercises/enums/enums3.rs
+++ b/exercises/enums/enums3.rs
@@ -1,10 +1,12 @@
 // enums3.rs
 // Address all the TODOs to make the tests pass!
 
-// I AM NOT DONE
-
 enum Message {
     // TODO: implement the message variant types based on their usage below
+    Move (Point),
+    Echo (String),
+    ChangeColor (u8, u8, u8),
+    Quit,
 }
 
 struct Point {
@@ -37,6 +39,12 @@ impl State {
 
     fn process(&mut self, message: Message) {
         // TODO: create a match expression to process the different message variants
+        match message {
+            Message::ChangeColor(r, b, g) => self.change_color((r, b, g)),
+            Message::Echo(s) => self.echo(s),
+            Message::Move(p) => self.move_position(p),
+            Message::Quit => self.quit(),
+        };
     }
 }
 
diff --git a/exercises/error_handling/errors1.rs b/exercises/error_handling/errors1.rs
index 9c24d85..481cc28 100644
--- a/exercises/error_handling/errors1.rs
+++ b/exercises/error_handling/errors1.rs
@@ -6,14 +6,12 @@
 // this function to have.
 // Execute `rustlings hint errors1` for hints!
 
-// I AM NOT DONE
-
-pub fn generate_nametag_text(name: String) -> Option<String> {
+pub fn generate_nametag_text(name: String) -> Result<String, String> {
     if name.len() > 0 {
-        Some(format!("Hi! My name is {}", name))
+        Ok(format!("Hi! My name is {}", name))
     } else {
         // Empty names aren't allowed.
-        None
+        Err("`name` was empty; it must be nonempty.".into())
     }
 }
 
@@ -28,7 +26,7 @@ mod tests {
     fn generates_nametag_text_for_a_nonempty_name() {
         assert_eq!(
             generate_nametag_text("Beyonc├⌐".into()),
-            Some("Hi! My name is Beyonc├⌐".into())
+            Ok("Hi! My name is Beyonc├⌐".into())
         );
     }
 
diff --git a/exercises/error_handling/errors2.rs b/exercises/error_handling/errors2.rs
index aad3a93..8d00e60 100644
--- a/exercises/error_handling/errors2.rs
+++ b/exercises/error_handling/errors2.rs
@@ -16,16 +16,15 @@
 // There are at least two ways to implement this that are both correct-- but
 // one is a lot shorter! Execute `rustlings hint errors2` for hints to both ways.
 
-// I AM NOT DONE
-
 use std::num::ParseIntError;
 
 pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
     let processing_fee = 1;
     let cost_per_item = 5;
-    let qty = item_quantity.parse::<i32>();
-
-    Ok(qty * cost_per_item + processing_fee)
+    
+    let qty = item_quantity.parse::<i32>()?;
+    
+    Result::Ok(qty * cost_per_item + processing_fee)
 }
 
 #[cfg(test)]
diff --git a/exercises/error_handling/errors3.rs b/exercises/error_handling/errors3.rs
index 460ac5c..07513a7 100644
--- a/exercises/error_handling/errors3.rs
+++ b/exercises/error_handling/errors3.rs
@@ -4,15 +4,16 @@
 // Why not? What should we do to fix it?
 // Execute `rustlings hint errors3` for hints!
 
-// I AM NOT DONE
-
 use std::num::ParseIntError;
 
 fn main() {
     let mut tokens = 100;
     let pretend_user_input = "8";
 
-    let cost = total_cost(pretend_user_input)?;
+    let cost = match total_cost(pretend_user_input) {
+        Ok(cost) => cost,
+        Err(_) => panic!(),
+    };
 
     if cost > tokens {
         println!("You can't afford that many!");
diff --git a/exercises/error_handling/errorsn.rs b/exercises/error_handling/errorsn.rs
index 5fe212b..5dcab6e 100644
--- a/exercises/error_handling/errorsn.rs
+++ b/exercises/error_handling/errorsn.rs
@@ -17,19 +17,21 @@
 //
 // Execute `rustlings hint errorsn` for hints :)
 
-// I AM NOT DONE
-
 use std::error;
 use std::fmt;
 use std::io;
 
 // PositiveNonzeroInteger is a struct defined below the tests.
-fn read_and_validate(b: &mut dyn io::BufRead) -> Result<PositiveNonzeroInteger, ???> {
+fn read_and_validate(b: &mut dyn io::BufRead) -> Result<PositiveNonzeroInteger, Box<dyn error::Error>> {
     let mut line = String::new();
     b.read_line(&mut line);
-    let num: i64 = line.trim().parse();
-    let answer = PositiveNonzeroInteger::new(num);
-    answer
+    let num: i64 = match line.trim().parse() {
+        Ok(num) => num,
+        Err(_) => return Err(Box::new(io::Error::new(io::ErrorKind::BrokenPipe, "uh-oh!"))),
+    };
+    let answer = PositiveNonzeroInteger::new(num)?;
+    
+    Ok(answer)
 }
 
 //
diff --git a/exercises/error_handling/result1.rs b/exercises/error_handling/result1.rs
index b978001..2a607ce 100644
--- a/exercises/error_handling/result1.rs
+++ b/exercises/error_handling/result1.rs
@@ -1,8 +1,6 @@
 // result1.rs
 // Make this test pass! Execute `rustlings hint result1` for hints :)
 
-// I AM NOT DONE
-
 #[derive(PartialEq, Debug)]
 struct PositiveNonzeroInteger(u64);
 
@@ -14,6 +12,12 @@ enum CreationError {
 
 impl PositiveNonzeroInteger {
     fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
+        if value == 0 {
+            return Err(CreationError::Zero);
+        }
+        if value < 0 {
+            return Err(CreationError::Negative);
+        }
         Ok(PositiveNonzeroInteger(value as u64))
     }
 }
diff --git a/exercises/functions/functions1.rs b/exercises/functions/functions1.rs
index 3112527..8eb4a76 100644
--- a/exercises/functions/functions1.rs
+++ b/exercises/functions/functions1.rs
@@ -1,7 +1,9 @@
 // functions1.rs
 // Make me compile! Execute `rustlings hint functions1` for hints :)
 
-// I AM NOT DONE
+fn call_me() {
+    // maybe
+}
 
 fn main() {
     call_me();
diff --git a/exercises/functions/functions2.rs b/exercises/functions/functions2.rs
index 108ba38..cfc3cc1 100644
--- a/exercises/functions/functions2.rs
+++ b/exercises/functions/functions2.rs
@@ -1,13 +1,11 @@
 // functions2.rs
 // Make me compile! Execute `rustlings hint functions2` for hints :)
 
-// I AM NOT DONE
-
 fn main() {
     call_me(3);
 }
 
-fn call_me(num) {
+fn call_me(num: i32) {
     for i in 0..num {
         println!("Ring! Call number {}", i + 1);
     }
diff --git a/exercises/functions/functions3.rs b/exercises/functions/functions3.rs
index e3c1bf7..4ae2f9a 100644
--- a/exercises/functions/functions3.rs
+++ b/exercises/functions/functions3.rs
@@ -1,10 +1,8 @@
 // functions3.rs
 // Make me compile! Execute `rustlings hint functions3` for hints :)
 
-// I AM NOT DONE
-
 fn main() {
-    call_me();
+    call_me(3);
 }
 
 fn call_me(num: i32) {
diff --git a/exercises/functions/functions4.rs b/exercises/functions/functions4.rs
index 58637e4..b6af7c4 100644
--- a/exercises/functions/functions4.rs
+++ b/exercises/functions/functions4.rs
@@ -4,14 +4,12 @@
 // This store is having a sale where if the price is an even number, you get
 // 10 Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
 
-// I AM NOT DONE
-
 fn main() {
     let original_price = 51;
     println!("Your sale price is {}", sale_price(original_price));
 }
 
-fn sale_price(price: i32) -> {
+fn sale_price(price: i32) -> i32 {
     if is_even(price) {
         price - 10
     } else {
diff --git a/exercises/functions/functions5.rs b/exercises/functions/functions5.rs
index d22aa6c..059e929 100644
--- a/exercises/functions/functions5.rs
+++ b/exercises/functions/functions5.rs
@@ -1,13 +1,11 @@
 // functions5.rs
 // Make me compile! Execute `rustlings hint functions5` for hints :)
 
-// I AM NOT DONE
-
 fn main() {
     let answer = square(3);
     println!("The answer is {}", answer);
 }
 
 fn square(num: i32) -> i32 {
-    num * num;
+    num * num
 }
diff --git a/exercises/generics/generics1.rs b/exercises/generics/generics1.rs
index d075a4d..62e2f00 100644
--- a/exercises/generics/generics1.rs
+++ b/exercises/generics/generics1.rs
@@ -1,10 +1,8 @@
 // This shopping list program isn't compiling! 
 // Use your knowledge of generics to fix it.
 
-// I AM NOT DONE
-
 fn main() {
-    let mut shopping_list: Vec<?> = Vec::new();
+    let mut shopping_list: Vec<&str> = Vec::new();
     shopping_list.push("milk");
 }
 
diff --git a/exercises/generics/generics2.rs b/exercises/generics/generics2.rs
index 23025aa..2ed3171 100644
--- a/exercises/generics/generics2.rs
+++ b/exercises/generics/generics2.rs
@@ -1,13 +1,12 @@
 // This powerful wrapper provides the ability to store a positive integer value.
 // Rewrite it using generics so that it supports wrapping ANY type.
 
-// I AM NOT DONE
-struct Wrapper {
-    value: u32
+struct Wrapper<T> {
+    value: T
 }
 
-impl Wrapper {
-    pub fn new(value: u32) -> Self {
+impl<T> Wrapper<T> {
+    pub fn new(value: T) -> Self {
         Wrapper { value }
     }
 }
diff --git a/exercises/generics/generics3.rs b/exercises/generics/generics3.rs
index c76425c..474e17c 100644
--- a/exercises/generics/generics3.rs
+++ b/exercises/generics/generics3.rs
@@ -7,17 +7,16 @@
 // Make the necessary code changes to support alphabetical report cards, thereby making 
 // the second test pass.
 
-// I AM NOT DONE
-pub struct ReportCard {
-    pub grade: f32,
+pub struct ReportCard<T: ToString> {
+    pub grade: T,
     pub student_name: String,
     pub student_age: u8,
 }
 
-impl ReportCard {
+impl<T: ToString> ReportCard<T> {
     pub fn print(&self) -> String {
         format!("{} ({}) - achieved a grade of {}", 
-            &self.student_name, &self.student_age, &self.grade)
+            &self.student_name, &self.student_age, &self.grade.to_string())
     }
 }
 
@@ -27,7 +26,7 @@ mod tests {
 
     #[test]
     fn generate_numeric_report_card() {
-        let report_card = ReportCard {
+        let report_card = ReportCard::<f64> {
             grade: 2.1, 
             student_name: "Tom Wriggle".to_string(), 
             student_age: 12,
@@ -38,8 +37,8 @@ mod tests {
     #[test]
     fn generate_alphabetic_report_card() {
         // TODO: Make sure to change the grade here after you finish the exercise.
-        let report_card = ReportCard {
-            grade: 2.1, 
+        let report_card = ReportCard::<String> {
+            grade: "A+".to_string(), 
             student_name: "Gary Plotter".to_string(), 
             student_age: 11,
         };
diff --git a/exercises/if/if1.rs b/exercises/if/if1.rs
index 9086754..1fe2b36 100644
--- a/exercises/if/if1.rs
+++ b/exercises/if/if1.rs
@@ -1,13 +1,17 @@
 // if1.rs
 
-// I AM NOT DONE
-
 pub fn bigger(a: i32, b: i32) -> i32 {
     // Complete this function to return the bigger number!
     // Do not use:
     // - another function call
     // - additional variables
     // Execute `rustlings hint if1` for hints
+    
+    if a > b {
+        a
+    } else {
+        b
+    }
 }
 
 // Don't mind this for now :)
diff --git a/exercises/if/if2.rs b/exercises/if/if2.rs
index 80effbd..545083a 100644
--- a/exercises/if/if2.rs
+++ b/exercises/if/if2.rs
@@ -4,13 +4,13 @@
 // Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
 // Execute the command `rustlings hint if2` if you want a hint :)
 
-// I AM NOT DONE
-
 pub fn fizz_if_foo(fizzish: &str) -> &str {
     if fizzish == "fizz" {
         "foo"
+    } else if fizzish == "fuzz" {
+        "bar"
     } else {
-        1
+        "baz"
     }
 }
 
diff --git a/exercises/macros/macros1.rs b/exercises/macros/macros1.rs
index ed0dac8..de2584a 100644
--- a/exercises/macros/macros1.rs
+++ b/exercises/macros/macros1.rs
@@ -1,8 +1,6 @@
 // macros1.rs
 // Make me compile! Execute `rustlings hint macros1` for hints :)
 
-// I AM NOT DONE
-
 macro_rules! my_macro {
     () => {
         println!("Check out my macro!");
@@ -10,5 +8,5 @@ macro_rules! my_macro {
 }
 
 fn main() {
-    my_macro();
+    my_macro!();
 }
diff --git a/exercises/macros/macros2.rs b/exercises/macros/macros2.rs
index d0be123..a931064 100644
--- a/exercises/macros/macros2.rs
+++ b/exercises/macros/macros2.rs
@@ -1,14 +1,12 @@
 // macros2.rs
 // Make me compile! Execute `rustlings hint macros2` for hints :)
 
-// I AM NOT DONE
-
-fn main() {
-    my_macro!();
-}
-
 macro_rules! my_macro {
     () => {
         println!("Check out my macro!");
     };
 }
+
+fn main() {
+    my_macro!();
+}
diff --git a/exercises/macros/macros3.rs b/exercises/macros/macros3.rs
index 93a4311..9caa0df 100644
--- a/exercises/macros/macros3.rs
+++ b/exercises/macros/macros3.rs
@@ -2,9 +2,8 @@
 // Make me compile, without taking the macro out of the module!
 // Execute `rustlings hint macros3` for hints :)
 
-// I AM NOT DONE
-
 mod macros {
+    #[macro_export]
     macro_rules! my_macro {
         () => {
             println!("Check out my macro!");
diff --git a/exercises/macros/macros4.rs b/exercises/macros/macros4.rs
index 3a74807..5608ecc 100644
--- a/exercises/macros/macros4.rs
+++ b/exercises/macros/macros4.rs
@@ -1,15 +1,13 @@
 // macros4.rs
 // Make me compile! Execute `rustlings hint macros4` for hints :)
 
-// I AM NOT DONE
-
 macro_rules! my_macro {
     () => {
         println!("Check out my macro!");
-    }
+    };
     ($val:expr) => {
         println!("Look at this other macro: {}", $val);
-    }
+    };
 }
 
 fn main() {
diff --git a/exercises/modules/modules1.rs b/exercises/modules/modules1.rs
index 812dfee..3af7f48 100644
--- a/exercises/modules/modules1.rs
+++ b/exercises/modules/modules1.rs
@@ -1,10 +1,8 @@
 // modules1.rs
 // Make me compile! Execute `rustlings hint modules1` for hints :)
 
-// I AM NOT DONE
-
 mod sausage_factory {
-    fn make_sausage() {
+    pub fn make_sausage() {
         println!("sausage!");
     }
 }
diff --git a/exercises/modules/modules2.rs b/exercises/modules/modules2.rs
index fde439d..50a3f08 100644
--- a/exercises/modules/modules2.rs
+++ b/exercises/modules/modules2.rs
@@ -1,11 +1,9 @@
 // modules2.rs
 // Make me compile! Execute `rustlings hint modules2` for hints :)
 
-// I AM NOT DONE
-
 mod delicious_snacks {
-    use self::fruits::PEAR as fruit;
-    use self::veggies::CUCUMBER as veggie;
+    pub use self::fruits::PEAR as fruit;
+    pub use self::veggies::CUCUMBER as veggie;
 
     mod fruits {
         pub const PEAR: &'static str = "Pear";
diff --git a/exercises/move_semantics/move_semantics1.rs b/exercises/move_semantics/move_semantics1.rs
index e2f5876..a95f851 100644
--- a/exercises/move_semantics/move_semantics1.rs
+++ b/exercises/move_semantics/move_semantics1.rs
@@ -1,12 +1,10 @@
 // move_semantics1.rs
 // Make me compile! Execute `rustlings hint move_semantics1` for hints :)
 
-// I AM NOT DONE
-
 fn main() {
     let vec0 = Vec::new();
 
-    let vec1 = fill_vec(vec0);
+    let mut vec1 = fill_vec(vec0);
 
     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
 
diff --git a/exercises/move_semantics/move_semantics2.rs b/exercises/move_semantics/move_semantics2.rs
index bd21fbb..47fcafe 100644
--- a/exercises/move_semantics/move_semantics2.rs
+++ b/exercises/move_semantics/move_semantics2.rs
@@ -2,16 +2,14 @@
 // Make me compile without changing line 13!
 // Execute `rustlings hint move_semantics2` for hints :)
 
-// I AM NOT DONE
-
 fn main() {
     let vec0 = Vec::new();
 
-    let mut vec1 = fill_vec(vec0);
-
     // Do not change the following line!
     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
 
+    let mut vec1 = fill_vec(vec0);
+    
     vec1.push(88);
 
     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
diff --git a/exercises/move_semantics/move_semantics3.rs b/exercises/move_semantics/move_semantics3.rs
index 43fef74..a881222 100644
--- a/exercises/move_semantics/move_semantics3.rs
+++ b/exercises/move_semantics/move_semantics3.rs
@@ -3,10 +3,8 @@
 // (no lines with multiple semicolons necessary!)
 // Execute `rustlings hint move_semantics3` for hints :)
 
-// I AM NOT DONE
-
 fn main() {
-    let vec0 = Vec::new();
+    let mut vec0 = Vec::new();
 
     let mut vec1 = fill_vec(vec0);
 
@@ -17,7 +15,7 @@ fn main() {
     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
 }
 
-fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
+fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
     vec.push(22);
     vec.push(44);
     vec.push(66);
diff --git a/exercises/move_semantics/move_semantics4.rs b/exercises/move_semantics/move_semantics4.rs
index a1c4a41..e6e780b 100644
--- a/exercises/move_semantics/move_semantics4.rs
+++ b/exercises/move_semantics/move_semantics4.rs
@@ -4,12 +4,8 @@
 // freshly created vector from fill_vec to its caller.
 // Execute `rustlings hint move_semantics4` for hints!
 
-// I AM NOT DONE
-
 fn main() {
-    let vec0 = Vec::new();
-
-    let mut vec1 = fill_vec(vec0);
+    let mut vec1 = fill_vec();
 
     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
 
@@ -20,7 +16,7 @@ fn main() {
 
 // `fill_vec()` no longer take `vec: Vec<i32>` as argument
 fn fill_vec() -> Vec<i32> {
-    let mut vec = vec;
+    let mut vec = Vec::new();
 
     vec.push(22);
     vec.push(44);
diff --git a/exercises/option/option1.rs b/exercises/option/option1.rs
index 602ff1a..7500319 100644
--- a/exercises/option/option1.rs
+++ b/exercises/option/option1.rs
@@ -1,23 +1,21 @@
 // option1.rs
 // Make me compile! Execute `rustlings hint option1` for hints
 
-// I AM NOT DONE
-
 // you can modify anything EXCEPT for this function's sig
 fn print_number(maybe_number: Option<u16>) {
     println!("printing: {}", maybe_number.unwrap());
 }
 
 fn main() {
-    print_number(13);
-    print_number(99);
+    print_number(Some(13));
+    print_number(Some(99));
 
-    let mut numbers: [Option<u16>; 5];
+    let mut numbers: [Option<u16>; 5] = [None; 5];
     for iter in 0..5 {
         let number_to_add: u16 = {
             ((iter * 1235) + 2) / (4 * 16)
         };
 
-        numbers[iter as usize] = number_to_add;
+        numbers[iter as usize] = Some(number_to_add);
     }
 }
diff --git a/exercises/option/option2.rs b/exercises/option/option2.rs
index a1517d7..e364a0f 100644
--- a/exercises/option/option2.rs
+++ b/exercises/option/option2.rs
@@ -1,16 +1,14 @@
 // option2.rs
 // Make me compile! Execute `rustlings hint option2` for hints
 
-// I AM NOT DONE
-
 fn main() {
     let optional_value = Some(String::from("rustlings"));
     // TODO: Make this an if let statement whose value is "Some" type
-    value = optional_value {
+    if let Some(value) = optional_value {
         println!("the value of optional value is: {}", value);
     } else {
         println!("The optional value doesn't contain anything!");
-    }
+    };
 
     let mut optional_values_vec: Vec<Option<i8>> = Vec::new();
     for x in 1..10 {
@@ -19,7 +17,9 @@ fn main() {
 
     // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
     // You can stack `Option<T>`'s into while let and if let
-    value = optional_values_vec.pop() {
-        println!("current value: {}", value);
+    while let Some(value) = optional_values_vec.pop() {
+        if let Some(value) = value {
+            println!("current value: {}", value);
+        }
     }
 }
diff --git a/exercises/primitive_types/primitive_types1.rs b/exercises/primitive_types/primitive_types1.rs
index 0912139..91734c6 100644
--- a/exercises/primitive_types/primitive_types1.rs
+++ b/exercises/primitive_types/primitive_types1.rs
@@ -2,8 +2,6 @@
 // Fill in the rest of the line that has code missing!
 // No hints, there's no tricks, just get used to typing these :)
 
-// I AM NOT DONE
-
 fn main() {
     // Booleans (`bool`)
 
@@ -12,7 +10,7 @@ fn main() {
         println!("Good morning!");
     }
 
-    let // Finish the rest of this line like the example! Or make it be false!
+    let is_evening = false; // Finish the rest of this line like the example! Or make it be false!
     if is_evening {
         println!("Good evening!");
     }
diff --git a/exercises/primitive_types/primitive_types2.rs b/exercises/primitive_types/primitive_types2.rs
index 6576a4d..89fd06a 100644
--- a/exercises/primitive_types/primitive_types2.rs
+++ b/exercises/primitive_types/primitive_types2.rs
@@ -2,8 +2,6 @@
 // Fill in the rest of the line that has code missing!
 // No hints, there's no tricks, just get used to typing these :)
 
-// I AM NOT DONE
-
 fn main() {
     // Characters (`char`)
 
@@ -16,7 +14,7 @@ fn main() {
         println!("Neither alphabetic nor numeric!");
     }
 
-    let // Finish this line like the example! What's your favorite character?
+    let your_character = '0'; // Finish this line like the example! What's your favorite character?
     // Try a letter, try a number, try a special character, try a character
     // from a different language than your own, try an emoji!
     if your_character.is_alphabetic() {
diff --git a/exercises/primitive_types/primitive_types3.rs b/exercises/primitive_types/primitive_types3.rs
index dfd6351..7765464 100644
--- a/exercises/primitive_types/primitive_types3.rs
+++ b/exercises/primitive_types/primitive_types3.rs
@@ -2,11 +2,12 @@
 // Create an array with at least 100 elements in it where the ??? is. 
 // Execute `rustlings hint primitive_types3` for hints!
 
-// I AM NOT DONE
-
 fn main() {
-    let a = ???
+    let a = [0; 100];
 
+    //println!("{:?}", a);
+    println!("{}", a.len());
+    
     if a.len() >= 100 {
         println!("Wow, that's a big array!");
     } else {
diff --git a/exercises/primitive_types/primitive_types4.rs b/exercises/primitive_types/primitive_types4.rs
index 10b553e..bf49c5d 100644
--- a/exercises/primitive_types/primitive_types4.rs
+++ b/exercises/primitive_types/primitive_types4.rs
@@ -2,13 +2,13 @@
 // Get a slice out of Array a where the ??? is so that the test passes.
 // Execute `rustlings hint primitive_types4` for hints!!
 
-// I AM NOT DONE
-
 #[test]
 fn slice_out_of_array() {
     let a = [1, 2, 3, 4, 5];
 
-    let nice_slice = ???
+    let nice_slice = &a[1..4];
+    
+    println!("{:?}", nice_slice);
 
     assert_eq!([2, 3, 4], nice_slice)
 }
diff --git a/exercises/primitive_types/primitive_types5.rs b/exercises/primitive_types/primitive_types5.rs
index 680d8d2..85f42cf 100644
--- a/exercises/primitive_types/primitive_types5.rs
+++ b/exercises/primitive_types/primitive_types5.rs
@@ -2,11 +2,9 @@
 // Destructure the `cat` tuple so that the println will work.
 // Execute `rustlings hint primitive_types5` for hints!
 
-// I AM NOT DONE
-
 fn main() {
     let cat = ("Furry McFurson", 3.5);
-    let /* your pattern here */ = cat;
+    let (name, age) = cat;
 
     println!("{} is {} years old.", name, age);
 }
diff --git a/exercises/primitive_types/primitive_types6.rs b/exercises/primitive_types/primitive_types6.rs
index 2bc817e..c8d010a 100644
--- a/exercises/primitive_types/primitive_types6.rs
+++ b/exercises/primitive_types/primitive_types6.rs
@@ -3,9 +3,7 @@
 // You can put this right into the `println!` where the ??? is.
 // Execute `rustlings hint primitive_types6` for hints!
 
-// I AM NOT DONE
-
 fn main() {
     let numbers = (1, 2, 3);
-    println!("The second number is {}", ???);
+    println!("The second number is {}", numbers.1);
 }
diff --git a/exercises/quiz1.rs b/exercises/quiz1.rs
index 5c5c355..4772c31 100644
--- a/exercises/quiz1.rs
+++ b/exercises/quiz1.rs
@@ -7,11 +7,17 @@
 // more than 40 at once, each apple only costs 1! Write a function that calculates
 // the price of an order of apples given the order amount. No hints this time!
 
-// I AM NOT DONE
-
 // Put your function here!
 // fn ..... {
 
+fn calculate_apple_price(num: i32) -> i32 {
+    if(num <= 40) {
+        num * 2
+    } else {
+        num
+    }
+}
+
 // Don't modify this function!
 #[test]
 fn verify_test() {
diff --git a/exercises/quiz2.rs b/exercises/quiz2.rs
index 8caeaa9..613c1d9 100644
--- a/exercises/quiz2.rs
+++ b/exercises/quiz2.rs
@@ -7,8 +7,6 @@
 // you think each value is. That is, add either `string_slice` or `string`
 // before the parentheses on each line. If you're right, it will compile!
 
-// I AM NOT DONE
-
 fn string_slice(arg: &str) {
     println!("{}", arg);
 }
@@ -17,14 +15,14 @@ fn string(arg: String) {
 }
 
 fn main() {
-    ("blue");
-    ("red".to_string());
-    (String::from("hi"));
-    ("rust is fun!".to_owned());
-    ("nice weather".into());
-    (format!("Interpolation {}", "Station"));
-    (&String::from("abc")[0..1]);
-    ("  hello there ".trim());
-    ("Happy Monday!".to_string().replace("Mon", "Tues"));
-    ("mY sHiFt KeY iS sTiCkY".to_lowercase());
+    string_slice("blue");
+    string("red".to_string());
+    string(String::from("hi"));
+    string("rust is fun!".to_owned());
+    string_slice("nice weather".into());
+    string(format!("Interpolation {}", "Station"));
+    string_slice(&String::from("abc")[0..1]);
+    string_slice("  hello there ".trim());
+    string("Happy Monday!".to_string().replace("Mon", "Tues"));
+    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
 }
diff --git a/exercises/quiz3.rs b/exercises/quiz3.rs
index a0cd371..0bad189 100644
--- a/exercises/quiz3.rs
+++ b/exercises/quiz3.rs
@@ -7,8 +7,6 @@
 // we expect to get when we call `times_two` with a negative number.
 // No hints, you can do this :)
 
-// I AM NOT DONE
-
 pub fn times_two(num: i32) -> i32 {
     num * 2
 }
@@ -19,11 +17,11 @@ mod tests {
 
     #[test]
     fn returns_twice_of_positive_numbers() {
-        assert_eq!(times_two(4), ???);
+        assert_eq!(times_two(4), 8);
     }
 
     #[test]
     fn returns_twice_of_negative_numbers() {
-        // TODO write an assert for `times_two(-4)`
+        assert_eq!(times_two(-4), -8);
     }
 }
diff --git a/exercises/quiz4.rs b/exercises/quiz4.rs
index 6c47480..9d4ed4f 100644
--- a/exercises/quiz4.rs
+++ b/exercises/quiz4.rs
@@ -5,7 +5,15 @@
 
 // Write a macro that passes the quiz! No hints this time, you can do it!
 
-// I AM NOT DONE
+macro_rules! my_macro{
+    ($val:expr) => {
+        {
+            let mut s = String::from("Hello ");
+            s.push_str($val);
+            s
+        }
+    };
+}
 
 #[cfg(test)]
 mod tests {
diff --git a/exercises/standard_library_types/arc1.rs b/exercises/standard_library_types/arc1.rs
index 07932c6..180d85a 100644
--- a/exercises/standard_library_types/arc1.rs
+++ b/exercises/standard_library_types/arc1.rs
@@ -4,18 +4,17 @@
 // somewhere. Try not to create any copies of the `numbers` Vec!
 // Execute `rustlings hint arc1` for hints :)
 
-// I AM NOT DONE
-
 #![forbid(unused_imports)] // Do not change this, (or the next) line.
 use std::sync::Arc;
 use std::thread;
 
 fn main() {
     let numbers: Vec<_> = (0..100u32).collect();
-    let shared_numbers = // TODO
+    let shared_numbers = Arc::new(numbers);
     let mut joinhandles = Vec::new();
 
     for offset in 0..8 {
+        let child_numbers = Arc::clone(&shared_numbers);
         joinhandles.push(thread::spawn(move || {
             let mut i = offset;
             let mut sum = 0;
diff --git a/exercises/standard_library_types/box1.rs b/exercises/standard_library_types/box1.rs
index f2654ce..2eaa0bb 100644
--- a/exercises/standard_library_types/box1.rs
+++ b/exercises/standard_library_types/box1.rs
@@ -16,11 +16,9 @@
 //
 // Execute `rustlings hint box1` for hints :)
 
-// I AM NOT DONE
-
 #[derive(PartialEq, Debug)]
 pub enum List {
-    Cons(i32, List),
+    Cons(i32, Box<List>),
     Nil,
 }
 
@@ -30,11 +28,11 @@ fn main() {
 }
 
 pub fn create_empty_list() -> List {
-    unimplemented!()
+    List::Nil
 }
 
 pub fn create_non_empty_list() -> List {
-    unimplemented!()
+    List::Cons(1, Box::new(List::Nil))
 }
 
 #[cfg(test)]
diff --git a/exercises/standard_library_types/iterators2.rs b/exercises/standard_library_types/iterators2.rs
index 837725f..3aac6e3 100644
--- a/exercises/standard_library_types/iterators2.rs
+++ b/exercises/standard_library_types/iterators2.rs
@@ -7,14 +7,33 @@
 //         Try to ensure it returns a single string.
 // As always, there are hints if you execute `rustlings hint iterators2`!
 
-// I AM NOT DONE
-
 pub fn capitalize_first(input: &str) -> String {
     let mut c = input.chars();
     match c.next() {
         None => String::new(),
-        Some(first) => first.collect::<String>() + c.as_str(),
+        Some(first) => {
+            let first = match first.to_uppercase().next() {
+                None => first,
+                Some(first) => first,
+            };
+            
+            let mut s = String::new();
+            s.push(first);
+            s.push_str(c.as_str());
+            
+            s
+        },
+    }
+}
+
+fn capitalize_words(words: Vec<&str>) -> Vec<String> {
+    let mut cap_words = Vec::new();
+    
+    for word in words.iter() {
+        cap_words.push(capitalize_first(word));
     }
+    
+    cap_words
 }
 
 #[cfg(test)]
@@ -37,14 +56,14 @@ mod tests {
     #[test]
     fn test_iterate_string_vec() {
         let words = vec!["hello", "world"];
-        let capitalized_words: Vec<String> = // TODO
+        let capitalized_words: Vec<String> = capitalize_words(words);
         assert_eq!(capitalized_words, ["Hello", "World"]);
     }
 
     #[test]
     fn test_iterate_into_string() {
         let words = vec!["hello", " ", "world"];
-        let capitalized_words = // TODO
+        let capitalized_words = capitalize_words(words).join("");
         assert_eq!(capitalized_words, "Hello World");
     }
 }
diff --git a/exercises/standard_library_types/iterators3.rs b/exercises/standard_library_types/iterators3.rs
index 353cea6..f5280a2 100644
--- a/exercises/standard_library_types/iterators3.rs
+++ b/exercises/standard_library_types/iterators3.rs
@@ -7,8 +7,6 @@
 // Execute `rustlings hint iterators3` to get some hints!
 // Have fun :-)
 
-// I AM NOT DONE
-
 #[derive(Debug, PartialEq, Eq)]
 pub enum DivisionError {
     NotDivisible(NotDivisibleError),
@@ -24,7 +22,16 @@ pub struct NotDivisibleError {
 // This function should calculate `a` divided by `b` if `a` is
 // evenly divisible by b.
 // Otherwise, it should return a suitable error.
-pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {}
+pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
+    if b == 0 {
+        return Result::Err(DivisionError::DivideByZero);
+    }
+    if a % b != 0 {
+        return Result::Err(DivisionError::NotDivisible(NotDivisibleError{dividend: a, divisor:b}));
+    }
+    
+    Result::Ok(a/b)
+}
 
 #[cfg(test)]
 mod tests {
@@ -58,12 +65,11 @@ mod tests {
     }
 
     // Iterator exercises using your `divide` function
-    /*
     #[test]
     fn result_with_list() {
         let numbers = vec![27, 297, 38502, 81];
         let division_results = numbers.into_iter().map(|n| divide(n, 27));
-        let x //... Fill in here!
+        let x = division_results.collect::<Result<Vec<i32>, DivisionError>>();//... Fill in here!
         assert_eq!(format!("{:?}", x), "Ok([1, 11, 1426, 3])");
     }
 
@@ -71,8 +77,7 @@ mod tests {
     fn list_of_results() {
         let numbers = vec![27, 297, 38502, 81];
         let division_results = numbers.into_iter().map(|n| divide(n, 27));
-        let x //... Fill in here!
+        let x = division_results.collect::<Vec<Result<i32, DivisionError>>>();//... Fill in here!
         assert_eq!(format!("{:?}", x), "[Ok(1), Ok(11), Ok(1426), Ok(3)]");
     }
-    */
 }
diff --git a/exercises/standard_library_types/iterators4.rs b/exercises/standard_library_types/iterators4.rs
index 8886283..fc6098e 100644
--- a/exercises/standard_library_types/iterators4.rs
+++ b/exercises/standard_library_types/iterators4.rs
@@ -1,7 +1,5 @@
 // iterators4.rs
 
-// I AM NOT DONE
-
 pub fn factorial(num: u64) -> u64 {
     // Complete this function to return the factorial of num
     // Do not use:
@@ -12,6 +10,13 @@ pub fn factorial(num: u64) -> u64 {
     // For an extra challenge, don't use:
     // - recursion
     // Execute `rustlings hint iterators4` for hints.
+    let mut f = num;
+    
+    for n in (1..num) {
+        f *= n;
+    }
+    
+    f
 }
 
 #[cfg(test)]
diff --git a/exercises/strings/strings1.rs b/exercises/strings/strings1.rs
index 8090244..e43c88a 100644
--- a/exercises/strings/strings1.rs
+++ b/exercises/strings/strings1.rs
@@ -2,13 +2,11 @@
 // Make me compile without changing the function signature!
 // Execute `rustlings hint strings1` for hints ;)
 
-// I AM NOT DONE
-
 fn main() {
     let answer = current_favorite_color();
     println!("My current favorite color is {}", answer);
 }
 
 fn current_favorite_color() -> String {
-    "blue"
+    String::from("blue")
 }
diff --git a/exercises/strings/strings2.rs b/exercises/strings/strings2.rs
index 5a2ce74..a048f88 100644
--- a/exercises/strings/strings2.rs
+++ b/exercises/strings/strings2.rs
@@ -2,11 +2,9 @@
 // Make me compile without changing the function signature!
 // Execute `rustlings hint strings2` for hints :)
 
-// I AM NOT DONE
-
 fn main() {
     let word = String::from("green"); // Try not changing this line :)
-    if is_a_color_word(word) {
+    if is_a_color_word(&word) {
         println!("That is a color word I know!");
     } else {
         println!("That is not a color word I know.");
diff --git a/exercises/structs/structs1.rs b/exercises/structs/structs1.rs
index 6d0b2f4..ace6e69 100644
--- a/exercises/structs/structs1.rs
+++ b/exercises/structs/structs1.rs
@@ -1,13 +1,12 @@
 // structs1.rs
 // Address all the TODOs to make the tests pass!
 
-// I AM NOT DONE
-
 struct ColorClassicStruct {
-    // TODO: Something goes here
+    name : String,
+    hex : String,
 }
 
-struct ColorTupleStruct(/* TODO: Something goes here */);
+struct ColorTupleStruct(String, String);
 
 #[derive(Debug)]
 struct UnitStruct;
@@ -18,8 +17,10 @@ mod tests {
 
     #[test]
     fn classic_c_structs() {
-        // TODO: Instantiate a classic c struct!
-        // let green =
+        let green = ColorClassicStruct {
+            name: String::from("green"),
+            hex: String::from("#00FF00"),
+        };
 
         assert_eq!(green.name, "green");
         assert_eq!(green.hex, "#00FF00");
@@ -27,8 +28,7 @@ mod tests {
 
     #[test]
     fn tuple_structs() {
-        // TODO: Instantiate a tuple struct!
-        // let green =
+        let green = ("green", "#00FF00");
 
         assert_eq!(green.0, "green");
         assert_eq!(green.1, "#00FF00");
@@ -36,8 +36,7 @@ mod tests {
 
     #[test]
     fn unit_structs() {
-        // TODO: Instantiate a unit struct!
-        // let unit_struct =
+        let unit_struct = UnitStruct;
         let message = format!("{:?}s are fun!", unit_struct);
 
         assert_eq!(message, "UnitStructs are fun!");
diff --git a/exercises/structs/structs2.rs b/exercises/structs/structs2.rs
index f9c6427..df2e265 100644
--- a/exercises/structs/structs2.rs
+++ b/exercises/structs/structs2.rs
@@ -1,8 +1,6 @@
 // structs2.rs
 // Address all the TODOs to make the tests pass!
 
-// I AM NOT DONE
-
 #[derive(Debug)]
 struct Order {
     name: String,
@@ -34,7 +32,15 @@ mod tests {
     fn your_order() {
         let order_template = create_order_template();
         // TODO: Create your own order using the update syntax and template above!
-        // let your_order =
+        let your_order = Order {
+            name: String::from("Hacker in Rust"),
+            year: 2019,
+            made_by_phone: false,
+            made_by_mobile: false,
+            made_by_email: true,
+            item_number: 123,
+            count: 1,
+        };
         assert_eq!(your_order.name, "Hacker in Rust");
         assert_eq!(your_order.year, order_template.year);
         assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
diff --git a/exercises/structs/structs3.rs b/exercises/structs/structs3.rs
index 5503ce1..d53253f 100644
--- a/exercises/structs/structs3.rs
+++ b/exercises/structs/structs3.rs
@@ -3,8 +3,6 @@
 // exercise we have defined the Package struct and we want to test some logic attached to it,
 // make the code compile and the tests pass! If you have issues execute `rustlings hint structs3`
 
-// I AM NOT DONE
-
 #[derive(Debug)]
 struct Package {
     sender_country: String,
@@ -16,17 +14,20 @@ impl Package {
     fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
         if weight_in_grams <= 0 {
             // Something goes here...
+            panic!();
         } else {
             return Package {sender_country, recipient_country, weight_in_grams};
         }
     }
 
-    fn is_international(&self) -> ??? {
+    fn is_international(&self) -> bool {
         // Something goes here...
+        !self.sender_country.eq(&self.recipient_country)
     }
 
-    fn get_fees(&self, cents_per_kg: i32) -> ??? {
+    fn get_fees(&self, cents_per_kg: i32) -> i32 {
         // Something goes here... (beware of grams to kg conversion)
+        cents_per_kg*self.weight_in_grams/1000
     }
 }
 
@@ -58,7 +59,7 @@ mod tests {
         let sender_country = String::from("Spain");
         let recipient_country = String::from("Spain");
 
-        let cents_per_kg = ???;
+        let cents_per_kg = 3000;
         
         let package = Package::new(sender_country, recipient_country, 1500);
         
diff --git a/exercises/tests/tests1.rs b/exercises/tests/tests1.rs
index 50586a1..005731f 100644
--- a/exercises/tests/tests1.rs
+++ b/exercises/tests/tests1.rs
@@ -6,12 +6,10 @@
 // This test has a problem with it -- make the test compile! Make the test
 // pass! Make the test fail! Execute `rustlings hint tests1` for hints :)
 
-// I AM NOT DONE
-
 #[cfg(test)]
 mod tests {
     #[test]
     fn you_can_assert() {
-        assert!();
+        assert!(true);
     }
 }
diff --git a/exercises/tests/tests2.rs b/exercises/tests/tests2.rs
index 0d981ad..6bfcd09 100644
--- a/exercises/tests/tests2.rs
+++ b/exercises/tests/tests2.rs
@@ -2,12 +2,10 @@
 // This test has a problem with it -- make the test compile! Make the test
 // pass! Make the test fail! Execute `rustlings hint tests2` for hints :)
 
-// I AM NOT DONE
-
 #[cfg(test)]
 mod tests {
     #[test]
     fn you_can_assert_eq() {
-        assert_eq!();
+        assert_eq!(1, 1);
     }
 }
diff --git a/exercises/tests/tests3.rs b/exercises/tests/tests3.rs
index 693b8aa..c45b48b 100644
--- a/exercises/tests/tests3.rs
+++ b/exercises/tests/tests3.rs
@@ -4,8 +4,6 @@
 // we expect to get when we call `is_even(5)`.
 // Execute `rustlings hint tests3` for hints :)
 
-// I AM NOT DONE
-
 pub fn is_even(num: i32) -> bool {
     num % 2 == 0
 }
@@ -16,6 +14,11 @@ mod tests {
 
     #[test]
     fn is_true_when_even() {
-        assert!();
+        assert!(is_even(2));
+    }
+    
+    #[test]
+    fn is_false_when_odd() {
+        assert!(!is_even(5));
     }
 }
diff --git a/exercises/threads/threads1.rs b/exercises/threads/threads1.rs
index 1785e8c..b1081b0 100644
--- a/exercises/threads/threads1.rs
+++ b/exercises/threads/threads1.rs
@@ -5,9 +5,7 @@
 // of "waiting..." and the program ends without timing out when running,
 // you've got it :)
 
-// I AM NOT DONE
-
-use std::sync::Arc;
+use std::sync::{Arc, Mutex};
 use std::thread;
 use std::time::Duration;
 
@@ -16,15 +14,15 @@ struct JobStatus {
 }
 
 fn main() {
-    let status = Arc::new(JobStatus { jobs_completed: 0 });
+    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
     let status_shared = status.clone();
     thread::spawn(move || {
         for _ in 0..10 {
             thread::sleep(Duration::from_millis(250));
-            status_shared.jobs_completed += 1;
+            status_shared.lock().unwrap().jobs_completed += 1;
         }
     });
-    while status.jobs_completed < 10 {
+    while status.lock().unwrap().jobs_completed < 10 {
         println!("waiting... ");
         thread::sleep(Duration::from_millis(500));
     }
diff --git a/exercises/traits/traits1.rs b/exercises/traits/traits1.rs
index 8253ef8..36d3c4e 100644
--- a/exercises/traits/traits1.rs
+++ b/exercises/traits/traits1.rs
@@ -8,14 +8,18 @@
 // which appends "Bar" to any object
 // implementing this trait.
 
-// I AM NOT DONE
 trait AppendBar {
     fn append_bar(self) -> Self;
 }
 
 impl AppendBar for String {
     //Add your code here
-
+    fn append_bar(self) -> Self {
+        let mut bar: Self = self.clone();
+        bar.push_str("Bar");
+        
+        bar
+    }
 }
 
 fn main() {
diff --git a/exercises/traits/traits2.rs b/exercises/traits/traits2.rs
index 7f5014d..7086633 100644
--- a/exercises/traits/traits2.rs
+++ b/exercises/traits/traits2.rs
@@ -10,16 +10,19 @@
 // No boiler plate code this time,
 // you can do this!
 
-// I AM NOT DONE
-
 trait AppendBar {
     fn append_bar(self) -> Self;
 }
 
 //TODO: Add your code here
-
-
-
+impl AppendBar for Vec<String> {
+    fn append_bar(self) -> Self {
+        let mut bar: Self = self.clone();
+        bar.push("Bar".to_string());
+        
+        bar
+    }
+}
 
 #[cfg(test)]
 mod tests {
diff --git a/exercises/variables/variables1.rs b/exercises/variables/variables1.rs
index 4a3af73..db85e7e 100644
--- a/exercises/variables/variables1.rs
+++ b/exercises/variables/variables1.rs
@@ -6,9 +6,7 @@
 // even after you already figured it out. If you got everything working and
 // feel ready for the next exercise, remove the `I AM NOT DONE` comment below.
 
-// I AM NOT DONE
-
 fn main() {
-    x = 5;
+    let x = 5;
     println!("x has the value {}", x);
 }
diff --git a/exercises/variables/variables2.rs b/exercises/variables/variables2.rs
index 7774a8f..5cdc753 100644
--- a/exercises/variables/variables2.rs
+++ b/exercises/variables/variables2.rs
@@ -1,10 +1,8 @@
 // variables2.rs
 // Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)
 
-// I AM NOT DONE
-
 fn main() {
-    let x;
+    let x = 10;
     if x == 10 {
         println!("Ten!");
     } else {
diff --git a/exercises/variables/variables3.rs b/exercises/variables/variables3.rs
index 30ec48f..7d84709 100644
--- a/exercises/variables/variables3.rs
+++ b/exercises/variables/variables3.rs
@@ -1,10 +1,8 @@
 // variables3.rs
 // Make me compile! Execute the command `rustlings hint variables3` if you want a hint :)
 
-// I AM NOT DONE
-
 fn main() {
-    let x = 3;
+    let mut x = 3;
     println!("Number {}", x);
     x = 5; // don't change this line
     println!("Number {}", x);
diff --git a/exercises/variables/variables4.rs b/exercises/variables/variables4.rs
index 77f1e9a..86645b5 100644
--- a/exercises/variables/variables4.rs
+++ b/exercises/variables/variables4.rs
@@ -1,9 +1,7 @@
 // variables4.rs
 // Make me compile! Execute the command `rustlings hint variables4` if you want a hint :)
 
-// I AM NOT DONE
-
 fn main() {
-    let x: i32;
+    let x: i32 = 1;
     println!("Number {}", x);
 }
diff --git a/exercises/variables/variables5.rs b/exercises/variables/variables5.rs
index 5b2c2fa..b87d501 100644
--- a/exercises/variables/variables5.rs
+++ b/exercises/variables/variables5.rs
@@ -1,11 +1,9 @@
 // variables5.rs
 // Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)
 
-// I AM NOT DONE
-
 fn main() {
     let number = "3"; // don't change this line
     println!("Number {}", number);
-    number = 3;
+    let number = 3;
     println!("Number {}", number);
 }
diff --git a/exercises/variables/variables6.rs b/exercises/variables/variables6.rs
index 9866691..2ad47aa 100644
--- a/exercises/variables/variables6.rs
+++ b/exercises/variables/variables6.rs
@@ -1,9 +1,7 @@
 // variables6.rs
 // Make me compile! Execute the command `rustlings hint variables6` if you want a hint :)
 
-// I AM NOT DONE
-
-const NUMBER = 3;
+const NUMBER: i32 = 3;
 fn main() {
     println!("Number {}", NUMBER);
 }
