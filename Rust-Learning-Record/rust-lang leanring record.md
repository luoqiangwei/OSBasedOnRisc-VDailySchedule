# rust-lang leanring record

<!-- TOC -->

- [rust-lang leanring record](#rust-lang-leanring-record)
    - [Ruat tools](#ruat-tools)
        - [rustc](#rustc)
    - [Basic syntax](#basic-syntax)

<!-- /TOC -->

## Ruat tools
### rustc
这个可以将rust的源文件，编译成二进制的可执行文件。
用法：

```shell
rustc <source file>
```

## Basic syntax

### 函数入口

main函数作为执行入口，以下是一个示例。

```rust
// This is the main function
fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
}
```



### 格式化输出

* format!

用于输出格式化文本到String。

* print!

类似`format!`，并且文本会被打印到控制台(io::stdout)。

* println!

类似`print!`，并且会在会在打印后自动添加换行控制符。

* eprint!

类似`format!`，并且文本会被输出到标准错误流(io::stderr)。

* eprintln!

类似`eprint!`，并且会在会在打印后自动添加换行控制符。



`std::fmt`包含很多控制文本显示的方式，以下是两种重要的方式。

* `fmt::Debug`: 使用标记（占位符），格式化文本以便调试。 `{:?}`

* `fmt::Display`： 使用标记（占位符），以更加有好的方式进行文本格式化。`{}`

`fmt::Display`实现的ToString方式，会自动将类型转换成`String`。



### 注释

#### 普通注释

* ```rust
  // 这种注释是单行注释，在这一行的内容都会被编译器忽视
  ```

* ```rust
  /*
   这种注释是多行注释，在这个区域内的内容，都会被编译器忽视
  */
  ```

#### 文档注释

* ```rust
  /// 以下的项将会用于产生库文档条目
  ```

* ```rust
  //! 为闭包产生库文档条目
  ```



### 数据类型

#### 数据类型

* 整型：`i8`, `i16`, `i32`, `i64`, `i128` and `isize` (pointer size)
* 无符号整型：`u8`, `u16`, `u32`, `u64`, `u128` and `usize` (pointer size)
* 浮点类型：`f32`, `f64`
* `char` ：Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
* `bool` 取值 true 或者 false
* 单元（unit）类型`()`，它唯一的值就是空元组: ()

#### 组合类型

* `arrays`，数组类型，类似 [1, 2, 3]
* `tuples`，元组类型，类似 (1, true)

**如何为变量声明类型**

```rust
let logical: bool = true;
let a_float: f64 = 1.0;

// Or a default will be used.
let default_float   = 3.0; // `f64`
let default_integer = 7;   // `i32`
```

类似上面这样。

**元组拆分**

```rust
let cat = ("Furry McFurson", 3.5);
let (name, age) = cat;
```

**元组索引**

```rust
let numbers = (1, 2, 3);
println!("The second number is {}", numbers.1);
```



### 常量和变量

只用`let`声明的都是常量。

使用`let mut`声明的都是变量。



### 三元运算符的变化

```rust
return if a > b {a} else {b};
```

类似这样



### 数组切片

```rust
let a = [1, 2, 3, 4, 5];
let nice_slice = &a[1..4];
```

类似这样，分割出了`[2, 3, 4]`

注意！切片中的数字表示在数组指定下标处**前面**进行切割，所以a[4]前面切割，就把4和5分离了。



### 字符串

#### &str

字符串字面量`&str`是会在rust编译时就知道其值的字符串类型。

在定义元组、结构体是，需要注意`&str`的生命周期！

```rust
struct ColorClassicStruct<'a> {
    // TODO: Something goes here
    name: &'a str,
    hex: &'a str
}

struct ColorTupleStruct<'a>(/* TODO: Something goes here */&'a str, &'a str);

```

一半会在结构体或元组后用类似`<'a>`的标记进行生命周期定义。

#### String

这是标准库中的一种字符串，可以使用`String::from(<some data>)`进行转换。



### 结构体

结构体更新语法，即只更新部分数据，但是大部分数据和另一个结构体相同时。

```rust
let your_order = Order {
    name: String::from("Hacker in Rust"),
    count: 1,
    ..order_template
};
```

例如以上这样，以order_template为模板，只改变name和count创建结构体。

