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

4. 



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



