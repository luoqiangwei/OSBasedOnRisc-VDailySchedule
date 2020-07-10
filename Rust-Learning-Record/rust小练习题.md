# Rust小练习题

## exercises/variables

1. ```rust
       x = 5;
       --------------------------
       let x = 5;
   ```

2. ```rust
   let x;
   -----------------------------
   let x = 10;
   ```

3. ```rust
   let x = 3;
   --------------------
   let mut x = 3;
   ```

   原因，默认let是定义常量，无法重新赋值。

4. ```rust
   let x: i32;
   -----------------------
   let x: i32 = 0;
   ```

   未初始化的变量无法使用。

5. ```rust
   number = 3;
   -------------------------
   let number0 = 3;
   ```

   常量和变量在确定类型后，中途都无法改变。



## exercises/if

1. 在bigger方法中添加下面这个语句即可

   ```rust
   return if a > b {a} else {b};
   ```

   

## exercises/functions

1. ```rust
   fn call_me() {}
   ```

   总之，就是在main外面定义一个调用用的私有方法。

2. ```rust
   fn call_me(num)
   -------------------------------
   fn call_me(num: u32)
   ```

   方法传入的参数需要指定类型

3. ```rust
   call_me();
   --------------------------------
   call_me(4);
   ```

   需要传参的方法需要传入参数

4. ```rust
   fn sale_price(price: i32) -> 
   --------------------------------
   fn sale_price(price: i32) -> i32
   ```

   方法需要定义返回值

5. ```rust
   num * num;
   ------------------
   num * num
   ```

   将分号去掉，变成表达式，会自动返回值。



##   exercises/primitive_types

1. ```rust
   let
   -------------
   let is_evening = false;
   ```

   完成代码即可。

2. ```rust
   let your_character = '1';
   ==========================
   let your_character = 'C';
   ==========================
   let your_character = ' ';
   ```

3. ```rust
   let a = [123, 32, 6, 45];
   ```

4. ```rust
   let nice_slice = &a[1..4];
   ```

   注意！切片中的数字表示在数组指定下标处**前面**进行切割，所以a[4]前面切割，就把4和5分离了。

5. ```rust
   let (name, age) = cat;
   ```

   其实这里我是猜的，因为看起来是把cat元组的值，分别放在name和age中，正好我用过的python有类似的语法，所以就这么完成了~

6. ```rust
   println!("The second number is {}", numbers.1);
   ```

   其实我一开始使用nambers[1]，但是错误提示告诉我元组要用numbers.1进行索引。所以对于元组的操作，我已经基本了解了。



## exercises/structs

https://doc.rust-lang.org/rust-by-example/custom_types.html

根据这里进行学习和完成，这里无法使用其他语言的经验了。

1. ```rust
   // structs1.rs
   // Address all the TODOs to make the tests pass!
   struct ColorClassicStruct<'a> {
       // TODO: Something goes here
       name: &'a str,
       hex: &'a str
   }
   
   struct ColorTupleStruct<'a>(/* TODO: Something goes here */&'a str, &'a str);
   
   #[derive(Debug)]
   struct UnitStruct;
   
   #[cfg(test)]
   mod tests {
       use super::*;
   
       #[test]
       fn classic_c_structs() {
           // TODO: Instantiate a classic c struct!
           // let green =
           let green = ColorClassicStruct {
               name: "green", 
               hex: "#00FF00"
           };
   
           assert_eq!(green.name, "green");
           assert_eq!(green.hex, "#00FF00");
       }
   
       #[test]
       fn tuple_structs() {
           // TODO: Instantiate a tuple struct!
           // let green =
           let green = ColorTupleStruct("green", "#00FF00");
   
           assert_eq!(green.0, "green");
           assert_eq!(green.1, "#00FF00");
       }
   
       #[test]
       fn unit_structs() {
           // TODO: Instantiate a unit struct!
           // let unit_struct =
           let unit_struct = UnitStruct{};
           let message = format!("{:?}s are fun!", unit_struct);
   
           assert_eq!(message, "UnitStructs are fun!");
       }
   }
   
   ```

2. ```rust
   let your_order = Order {
       name: String::from("Hacker in Rust"),
       count: 1,
       ..order_template
   };
   ```

   以order_template为模板，只改变name和count创建结构体。

## exercises/strings

1. ```rust
   "blue"
   --------------------------
   String::from("blue")
   ```

   将&str类型转换为String类型

2. 参考来源https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str

   问题记录：how to convert a String to a &str in Rust?

   解决方法：利用切片将String转换成&str

   ```rust
   if is_a_color_word(&word[..])
   ```



## exercises/enums

1. ```rust
   enum Message {
       // TODO: define a few types of messages as used below
       Quit, Echo, Move, ChangeColor
   }
   ```

   和其他语言一样的方式定义枚举类型

2. ```rust
   enum Message {
       // TODO: define the different variants used below
       Move {x: i32, y: i32},
       Echo(String),
       ChangeColor(i32, i32, i32),
       Quit
   }
   ```

   这个用法，大部分都猜对了~

3. ```rust
   enum Message {
       // TODO: implement the message variant types based on their usage below
       Move {x: u8, y: u8},
       Echo(String),
       ChangeColor(u8, u8, u8),
       Quit
   }
   
   impl State {
       fn process(&mut self, message: Message) {
           // TODO: create a match expression to process the different message variants
           match message {
               Message::ChangeColor (x, y, z) => {
                   self.change_color((x, y, z));
               },
               Message::Echo (x) => {
                   self.echo(x);
               },
               Message::Move{x, y} => {
                   self.move_position(Point{x: x, y: y});
               },
               Message::Quit => {
                   self.quit();
               }
           }
       }
   }
   ```

   这里花了挺多时间学习match的语法，比如impl可以用self指代自己这个对线，类似java的this，但是更像python的写法。然后匹配后，如果要取出值，元组用()，结构体用{}，普通的则什么都不需要加。

## exercises/tests

1. ```rust
   assert!(12 == 12);
   ```

   调试用的语句，里面的值应该是一个布尔值。

2. ```rust
   assert_eq!(12, 12);
   ```

   判断两遍是否相等

3. ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
   
       #[test]
       fn is_true_when_even() {
           assert!(is_even(6));
       }
   }
   ```

   测试方法返回的bool值

## exercises/modules

1. ```rust
   fn make_sausage()
   ----------------------------
   pub fn make_sausage()
   ```

   将私有方法变成公有的

2. ```rust
       pub use self::fruits::PEAR as fruit;
       pub use self::veggies::CUCUMBER as veggie;
   ```

   给成员加上pub，扩大权限即可。我在这稍微花了些时间弄清楚pub应该放在什么位置。

## exercises/macros

1. ```rust
   my_macro();
   --------------
   my_macro!();
   ```

   将方法调用改成宏调用

2. 宏要在调用之前定义，把下面这段移到main方法上面。

   ```rust
   macro_rules! my_macro {
       () => {
           println!("Check out my macro!");
       };
   }
   ```

3. ```rust
   #[macro_use]
   mod macros {
       macro_rules! my_macro {
           () => {
               println!("Check out my macro!");
           };
       }
   }
   ```

   定义了宏的模块，需要加上#[macro_use]才能使用

4. ```rust
   macro_rules! my_macro {
       () => {
           println!("Check out my macro!");
       };
       ($val:expr) => {
           println!("Look at this other macro: {}", $val);
       };
   }
   
   ```

   两个宏实现后面没加分号



## exercises/move_semantics

1. Vec也要声明成变量才能进行修改。

2. ```rust
   let mut vec1 = fill_vec(vec0);
   ------------------
   let mut vec1 = vec0.clone();
   ```

   将移动改成克隆。

3. ```rust
   fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
       vec.push(22);
       vec.push(44);
       vec.push(66);
   
       vec
   }
   
   ```

   在方法参数列表，就设置vec为变量。

4. ```rust
   fn main() {
       let mut vec1 = fill_vec();
   
       println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
   
       vec1.push(88);
   
       println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
   }
   
   // `fill_vec()` no longer take `vec: Vec<i32>` as argument
   fn fill_vec() -> Vec<i32> {
       let mut vec = Vec::new();
   
       vec.push(22);
       vec.push(44);
       vec.push(66);
   
       vec
   }
   
   ```

   总之就是重构这个方法，这样就不用再定义一个没用的vue0创建了。

## exercises/error_handling

1. 其实此处是讲解一个关于函数编写的问题。

2. 此处参照https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator

   当方法加入了?操作符时，如果出现错误，将会自动返回出错的信息。

   ```rust
   let qty = item_quantity.parse::<i32>()?;
   ```

3. 此题其实是让main具有返回值，这样可以抛出异常，或者正常结束时返回0。

   ```rust
   fn main() -> Result<i32, ParseIntError> {
       let mut tokens = 100;
       let pretend_user_input = "8";
   
       let cost = total_cost(pretend_user_input)?;
   
       if cost > tokens {
           println!("You can't afford that many!");
       } else {
           tokens -= cost;
           println!("You now have {} tokens.", tokens);
       }
   
       Ok(0)
   }
   ```

4. ```rust
   fn read_and_validate(b: &mut dyn io::BufRead) -> Result<PositiveNonzeroInteger, Box<dyn error::Error>> {
       let mut line = String::new();
       b.read_line(&mut line)?;
       let num: i64 = line.trim().parse()?;
       let answer = PositiveNonzeroInteger::new(num)?;
       Ok(answer)
   }
   ```

   里面可能产生多种类型的异常，都在可能产生错误的方法后面加上问号，返回的错误类型不止一种，所以填入调用这个函数的函数返回的异常类型，即`Box<dyn error::Error>`，需要注意的是，answer需要用Ok返回，这里Ok的k是小写，不是大写！我在这里被坑了很久。

   errorsn.rs

5. result1.rs

   ```rust
   impl PositiveNonzeroInteger {
       fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
           if (value < 0) {
               Err(CreationError::Negative)
           } else if(value == 0) {
               Err(CreationError::Zero)
           } else {
               Ok(PositiveNonzeroInteger(value as u64))
           }
       }
   }
   ```

   根据不同的情况，返回值或异常。

## exercises/clippy

1. ```rust
   if (y - x).abs() > 0.0
   ```

   按照clippy的提示，将代码改成了这样，因为浮点数不精确，不能直接判断是否相等。

2. ```rust
   if let Some(x) = option {
   	res += x;
   }
   ```

   按照clippy的提示，将代码改成了这样，相当于将Option类型变成普通的数值类型。

## exercises/option

1. ```rust
   fn main() {
       print_number(Some(13));
       print_number(Some(99));
   
       let mut numbers: [Option<u16>; 5] = [Some(0), Some(0), Some(0), Some(0), Some(0)];
       for iter in 0..5 {
           let number_to_add: u16 = {
               ((iter * 5) + 2) / (4 * 16)
           };
   
           numbers[iter as usize] = Some(number_to_add);
       }
   }
   ```

   这里主要修改main中的数据类型。

   1. print_number需要传入一个Option<u16>类型的数据，所以需要用Some进行包装。
   2. 需要初始化数组numbers。
   3. 需要将iter改为usize类型。
   4. 需要封装number_to_add

2. ```rust
   fn main() {
       let optional_value = Some(String::from("rustlings"));
       // Make this an if let statement whose value is "Some" type
       if let Some(ref value) = optional_value {
           println!("the value of optional value is: {}", value);
       } else {
           println!("The optional value doesn't contain anything!");
       }
   
       let mut optional_values_vec: Vec<Option<i8>> = Vec::new();
       for x in 1..10 {
           optional_values_vec.push(Some(x));
       }
   
       // make this a while let statement - remember that vector.pop also adds another layer of Option<T>
       // You can stack `Option<T>`'s into while let and if let
       if let Some(ref value) = optional_values_vec.pop() {
           println!("current value: {}", value.unwrap());
       }
   }
   ```

     这里主要是if let的用法。

   1. 如果是从Option类型中取出有效值并判断，使用if let Some(ref value)，有效值会被放入value中。
   2. Vue pop的就是Option类型，再加上它原本就是存储Option类型，所以取出Value后，还需要unwarap。



## exercises/standard_library_types

1. ```rust
   fn main() {
       let numbers: Vec<_> = (0..100u32).collect();
       let shared_numbers = Arc::new(numbers); // TODO
       let mut joinhandles = Vec::new();
   
       for offset in 0..8 {
           let child_numbers = shared_numbers.clone();
           joinhandles.push(thread::spawn(move || {
               let mut i = offset;
               let mut sum = 0;
               while i < child_numbers.len() {
                   sum += child_numbers[i];
                   i += 5;
               }
               println!("Sum of offset {} is {}", offset, sum);
           }));
       }
       for handle in joinhandles.into_iter() {
           handle.join().unwrap();
       }
   }
   
   ```

2. ```rust
   pub fn capitalize_first(input: &str) -> String {
       let mut c = input.chars();
       let mut result = String::new();
       while let Some(first) = c.next() {
           if (result.len() == 0) {
               result.push(first.to_ascii_uppercase());
           } else {
               result.push(first);
           }
       }
       return result;
       // match c.next() {
       //     None => String::new(),
       //     Some(first) => first.collect::<String>() + c.as_str(),
       // }
   }
   
   #[cfg(test)]
   mod tests {
       use super::*;
   
       // Step 1.
       // Tests that verify your `capitalize_first` function implementation
       #[test]
       fn test_success() {
           assert_eq!(capitalize_first("hello"), "Hello");
       }
   
       #[test]
       fn test_empty() {
           assert_eq!(capitalize_first(""), "");
       }
   
       // Step 2.
       #[test]
       fn test_iterate_string_vec() {
           let words = vec!["hello", "world"];
           let capitalized_words: Vec<String> = words.iter().map(|x| capitalize_first(x)).collect(); // TODO
           assert_eq!(capitalized_words, ["Hello", "World"]);
       }
   
       #[test]
       fn test_iterate_into_string() {
           let words = vec!["hello", " ", "world"];
           let mut capitalized_words = String::new(); // TODO
           for word in words.iter() {
               capitalized_words.push_str(&capitalize_first(word));
           }
           assert_eq!(capitalized_words, "Hello World");
       }
   }
   
   ```

   只要按照步骤进行就可以了。

3. 挑战一部分，很简单，根据要求，针对不同的除法情况进行不同的处理即可。

   ```rust
   // This function should calculate `a` divided by `b` if `a` is
   // evenly divisible by b.
   // Otherwise, it should return a suitable error.
   pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
       if b == 0 {
           Err(DivisionError::DivideByZero)
       } else if a % b != 0 {
           Err(DivisionError::NotDivisible(NotDivisibleError{dividend: a, divisor: b}))
       } else {
           Ok(a / b)
       }
   }
   ```

   第二部分，难点在于分清楚类型之间的转换。

   ```rust
       // Iterator exercises using your `divide` function
       #[test]
       fn result_with_list() {
           let numbers = vec![27, 297, 38502, 81];
           let division_results = numbers.into_iter().map(|n| divide(n, 27));
           let x : Result<Vec<i32>, DivisionError> = Ok(division_results.map(|n| n.unwrap()).collect()); //... Fill in here!
           assert_eq!(format!("{:?}", x), "Ok([1, 11, 1426, 3])");
       }
   
       #[test]
       fn list_of_results() {
           let numbers = vec![27, 297, 38502, 81];
           let division_results = numbers.into_iter().map(|n| divide(n, 27));
           let x : Vec<Result<i32, DivisionError>> = division_results.collect(); //... Fill in here!
           assert_eq!(format!("{:?}", x), "[Ok(1), Ok(11), Ok(1426), Ok(3)]");
       }
   ```

4. ```rust
   pub fn factorial(num: u64) -> u64 {
       // Complete this function to return factorial of num
       // Do not use:
       // - return
       // For extra fun don't use:
       // - imperative style loops (for, while)
       // - additional variables
       // For the most fun don't use:
       // - recursion
       // Execute `rustlings hint iterators4` for hints.
       if (num == 1) {
           1
       } else {
           num * factorial(num - 1)
       }
   }
   ```

   对于我来说，这个测试是这里面最简单的……



## exercises/traits

1. ```rust
   impl AppendBar for String {
       //Add your code here
       fn append_bar(self) -> Self {
           self + "Bar"
       }
   }
   
   ```

   就是实现一个简单的traits，让所有String具备append_bar方法，效果是为原来的字符串添加Bar

2. ```rust
   //TODO: Add your code here
   impl AppendBar for Vec<String> {
       fn append_bar(self) -> Self {
           let mut tmp = self;
           tmp.push(String::from("Bar"));
           return tmp;
       }
   }
   ```

   此处首先需要让self变为可修改，然后在添加Bar后返回。



## exercises/generic

1. ```rust
   let mut shopping_list: Vec<&str> = Vec::new();
   ```

   把？换成`&str`表示存储字面量

2. ```rust
   // struct Wrapper<u32> {
   //     value: u32
   // }
   
   struct Wrapper<T> {
       value: T
   }
   
   // impl<u32> Wrapper<u32> {
   //     pub fn new(value: u32) -> Self {
   //         Wrapper { value }
   //     }
   // }
   
   impl<T> Wrapper<T> {
       pub fn new(value: T) -> Self {
           Wrapper { value }
       }
   }
   ```

   将Wrapper的类型改为泛型即可。别忘了删除**store_str_in_wrapper**中的`assert!(false);` 

3. ```rust
   impl ReportCard {
       pub fn print(&self) -> String {
           if self.grade < 2.5 {
               return format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, "A+");
           } else if self.grade < 3.0 {
               return format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, "B+");
           } else if self.grade < 3.5 {
               return format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, "C+");
           } else if self.grade < 4.0 {
               return format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, "D+");
           } else if self.grade < 4.5 {
               return format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, "E+");
           }
           return format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, String::from("F-"));
       }
   }
   ```

   总之，把分数转换成字母评分就可以了~

   后来发现好像理解错题目意思了，除了把第二个测试的分数改成字母以外，最后修改如下。

   ```rust
   pub struct ReportCard<T: std::fmt::Display> {
       pub grade: T,
       pub student_name: String,
       pub student_age: u8,
   }
   
   impl<T: std::fmt::Display> ReportCard<T> {
       pub fn print(&self) -> String {
           format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, &self.grade)
       }
   }
   ```

   只用T的时候报了错误，并给出了提示`'T' doesn't implement 'std::fmt::Display'`，所以就限定了类型。

## exercises/threads

终于到了线程部分！

```rust
fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            status_shared.lock().unwrap().jobs_completed += 1;
        }
    });
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
```

使用锁（Mutex，RwLock）才能在其他线程进行变量修改。

Arc<T>是指共享T的所有权。当调用clone之后，会产生一个新的指针指向堆空间同一个结构，当最后一个Arc指针被销毁时，这部分堆内存才会被回收。



##   exercises/conversions

终于到最后一部分了~

1. ```rust
   total / values.len()
   -----------------------------
   total / values.len() as f64
   ```

   类型转换……

2. ```rust
   // Steps:
   // 1. If the length of the provided string is 0, then return the default of Person
   // 2. Split the given string on the commas present in it
   // 3. Extract the first element from the split operation and use it as the name
   // 4. Extract the other element from the split operation and parse it into a `usize` as the age
   // If while parsing the age, something goes wrong, then return the default of Person
   // Otherwise, then return an instantiated Person onject with the results
   impl From<&str> for Person {
       fn from(s: &str) -> Person {
           if s.len() == 0 {
               return Person::default();
           }
           let t : Vec<&str> = s.split(",").collect();
           if let Ok(age1) = t[1].parse::<usize>() {
               Person{name: t[0].to_string(), age: age1}
           } else {
               Person::default()
           }
           
       }
   }
   ```

    需要注意parse转换出错的情况。

3. ```rust
   // Your task is to complete this implementation
   // in order for the line `let p = Person::try_from("Mark,20")` to compile
   // and return an Ok result of inner type Person.
   // Please note that you'll need to parse the age component into a `usize`
   // with something like `"4".parse::<usize>()`. The outcome of this needs to
   // be handled appropriately.
   //
   // Steps:
   // 1. If the length of the provided string is 0, then return an error
   // 2. Split the given string on the commas present in it
   // 3. Extract the first element from the split operation and use it as the name
   // 4. Extract the other element from the split operation and parse it into a `usize` as the age
   // If while parsing the age, something goes wrong, then return an error
   // Otherwise, then return a Result of a Person object
   impl TryFrom<&str> for Person {
       type Error = String;
       fn try_from(s: &str) -> Result<Self, Self::Error> {
           if s.len() == 0 {
               return Err(String::from("Err happen!"));
           }
           let t : Vec<&str> = s.split(",").collect();
           if let Ok(age1) = t[1].parse::<usize>() {
               Ok(Person{name: t[0].to_string(), age: age1})
           } else {
               Err(String::from("Err happen!"))
           }
       }
   }
   ```

   和之前那个一样，只不过这里是返回错误，不是返回默认的Person。

4. 参考此处信息*https://doc.rust-lang.org/std/convert/trait.AsRef.html*，其他类似信息*https://doc.rust-lang.org/std/convert/trait.AsMut.html*。

   ```rust
   // Obtain the number of bytes (not characters) in the given argument
   // Add the AsRef trait appropriately as a trait bound
   fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
       arg.as_ref().as_bytes().len()
   }
   
   // Obtain the number of characters (not bytes) in the given argument
   // Add the AsRef trait appropriately as a trait bound
   fn char_counter<T: AsRef<str>>(arg: T) -> usize {
       arg.as_ref().chars().count()
   }
   
   ```

   其实也算是“教学关”。

5. ```rust
   // Steps:
   // 1. If the length of the provided string is 0, then return an error
   // 2. Split the given string on the commas present in it
   // 3. Extract the first element from the split operation and use it as the name
   // 4. Extract the other element from the split operation and parse it into a `usize` as the age
   // If while parsing the age, something goes wrong, then return an error
   // Otherwise, then return a Result of a Person object
   impl FromStr for Person {
       type Err = String;
       fn from_str(s: &str) -> Result<Person, Self::Err> {
           if s.len() == 0 {
               Err(String::from("Format error!"))
           } else {
               let t : Vec<&str> = s.split(",").collect();
               if let Ok(age1) = t[1].parse::<usize>() {
                   Ok(Person{name: t[0].to_string(), age: age1})
               } else {
                   Err(String::from("Format error!"))
               }
           }
       }
   }
   ```

   和上一个之前的前两个大同小异~



## 测验

### test1

先定义一个方法**calculate_apple_price**，然后根据题目要求写就完成了~

```rust
fn calculate_apple_price(count: u32) -> u32 {
    if (count > 40) {
        count
    } else {
        count * 2
    }
}
```

### test2

```rust
string_slice("blue");
string("red".to_string());
string(String::from("hi"));
string("rust is fun!".to_owned());
string("nice weather".into());
string(format!("Interpolation {}", "Station"));
string_slice(&String::from("abc")[0..1]);
string_slice("  hello there ".trim());
string("Happy Monday!".to_string().replace("Mon", "Tues"));
string("mY sHiFt KeY iS sTiCkY".to_lowercase());
```

没想到一次通过了！~ 这的测试主要是用来考察对&str和String的判断。

### test3

自己补充和完成一个测试。

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_twice_of_positive_numbers() {
        assert_eq!(times_two(4), 8);
    }

    #[test]
    fn returns_twice_of_negative_numbers() {
        // TODO write an assert for `times_two(-4)`
        assert_eq!(times_two(-4), -8)
    }
}

```

### test4

总之，就是自己编写一个宏。

```rust
macro_rules! my_macro {
    ($s:expr) => {
        format!("Hello {}", $s);
    };
}
```



## 完成！

![image-20200706211024835](rust%E5%B0%8F%E7%BB%83%E4%B9%A0%E9%A2%98.assets/image-20200706211024835.png)