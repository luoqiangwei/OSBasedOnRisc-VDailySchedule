# Lab 0的一些记录

<!-- TOC -->

- [Lab 0的一些记录](#lab-0的一些记录)
  - [前情提要](#前情提要)
  - [实验概要](#实验概要)
  - [实验过程](#实验过程)
    - [创建Rust项目](#创建rust项目)

<!-- /TOC -->

## 前情提要
我已经不想继续花时间在无聊的Rust理论和Risc-V上了，所以我想直接开始Lab 0的学习，就像玩新游戏一样~

## 实验概要
* 使用 Rust 包管理器 cargo 创建一个 Rust 项目
* 移除 Rust 程序对操作系统的依赖，构建一个独立化可执行的程序
* 我们将程序的目标平台设置为 RISC-V，这样我们的代码将可以在 RISC-V 指令集的裸机（Bare Metal）上执行 Rust 代码
* 生成内核镜像、调整代码的内存布局并在 QEMU 模拟器中启动
* 封装如输出、关机等一些 SBI 的接口，方便后续开发

## 实验过程
### 创建Rust项目
1. 创建一个名为`rust-toolchain`的文件，并且写入工具链版本：
   ```shell
   nightly-2020-06-27
   ```
   
2. 在项目中运行`cargo new`命令创建一个新的Rust项目'os'
   ```shell
   cargo new os
   ```
   cargo 默认为我们添加了 `--bin` 选项，说明我们将要创建一个可执行文件而非一个库。
   
3. 进入'os'目录，执行`cargo run`，构建和运行项目。

### 移除标准库依赖

Rust默认的标准库是依赖于OS的std，但是OS实验是不可能一开始就提供std库的，所以需要使用`#![no_std]`进行禁用。

之后执行`cargo run`会出现错误，原因是默认创建的程序有一些宏和函数依赖于Rust基于特定OS的std标准库。

根据错误提示，将`src/main.rs`改成了这个样子。

```rust
#![no_std]

use core::panic::PanicInfo;

fn main() {
  // 删除println!("Hello, world!");
}

// 需要自己实现panic_handler，它在程序发生panic时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

`Cargo.toml`改为以下这样

```toml
[package]
name = "os"
version = "0.1.0"
authors = ["luoqiangwei <luoqiangwei@live.cn>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

# eh_personality是处理异常相关的一个语义项，当发生panic是调用.此处放弃使用panic
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

### 移除运行时环境依赖

```rust
//!  - `#![no_std]`
//! 禁用Rust标准库
#![no_std]

//!  - `#![no_main]`
//! 不使用`main`函数等全部Rust-level入口点作为程序入口
#![no_main]

use core::panic::PanicInfo;

/// 当 panic 发生时会调用该函数
/// 我们暂时将它的实现为一个死循环
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// 覆盖 crt0 中的 _start 函数
/// 我们暂时将它的实现为一个死循环
/// - `#[no_mangle]`
/// 函数禁用编译期间的名称重整（Name Mangling），即确保编译器生成一个名为 _start 的函数
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
```

此时运行`cargo build`还是构建错误，原因在于还在使用C的OS级运行时环境。

### 编译为裸机目标

输入`rustc --version --verbose`可以查看当前的环境信息。

Rust使用一个目标三元组（Target Triple）的字符串`<arch><sub>-<vendor>-<sys>-<abi>`。

目标三元组 riscv64imac-unknown-none-elf 描述了一个 RISC-V 64 位指令集的系统，并且环境底层没有操作系统，这是由三元组中的none描述的。

```shell
rustup target add riscv64imac-unknown-none-elf
```

使用这条指令进行添加。

在构建的时候进行指定编译目标环境。

```shell
cargo build --target riscv64imac-unknown-none-elf
```

为了方便，可以写入.cargo/config文件中

```properties
# 编译的目标平台
[build]
target = "riscv64imac-unknown-none-elf"
```

这样就可以直接使用`cargo build`进行构建了。

这样就可以编译成功了，Output:

```shell
Finished dev [unoptimized + debuginfo] target(s) in 0.82s
```

### 生成内核镜像

1. 安装binutils工具集

   Rust 社区提供了一个 cargo-binutils 项目，可以帮助我们方便地调用 Rust 内置的 LLVM binutils。我们用以下命令安装它

   ```shell
   cargo install cargo-binutils
   rustup component add llvm-tools-preview
   ```

2. 使用`rust-objdump --version`查看是否安装成功

   ```
   LLVM (http://llvm.org/):
     LLVM version 10.0.1-rust-1.46.0-nightly
     Optimized build.
     Default target: x86_64-unknown-linux-gnu
     Host CPU: haswell
   
     Registered Targets:
       aarch64    - AArch64 (little endian)
       aarch64_32 - AArch64 (little endian ILP32)
       aarch64_be - AArch64 (big endian)
       arm        - ARM
       arm64      - ARM64 (little endian)
       arm64_32   - ARM64 (little endian ILP32)
       armeb      - ARM (big endian)
       avr        - Atmel AVR Microcontroller
       hexagon    - Hexagon
       mips       - MIPS (32-bit big endian)
       mips64     - MIPS (64-bit big endian)
       mips64el   - MIPS (64-bit little endian)
       mipsel     - MIPS (32-bit little endian)
       msp430     - MSP430 [experimental]
       nvptx      - NVIDIA PTX 32-bit
       nvptx64    - NVIDIA PTX 64-bit
       ppc32      - PowerPC 32
       ppc64      - PowerPC 64
       ppc64le    - PowerPC 64 LE
       riscv32    - 32-bit RISC-V
       riscv64    - 64-bit RISC-V
       sparc      - Sparc
       sparcel    - Sparc LE
       sparcv9    - Sparc V9
       systemz    - SystemZ
       thumb      - Thumb
       thumbeb    - Thumb (big endian)
       wasm32     - WebAssembly 32-bit
       wasm64     - WebAssembly 64-bit
       x86        - 32-bit X86: Pentium-Pro and above
       x86-64     - 64-bit X86: EM64T and AMD64
   ```

3. 使用`file target/riscv64imac-unknown-none-elf/debug/os`查看编译产物信息

   ```
   target/riscv64imac-unknown-none-elf/debug/os: ELF 64-bit LSB executable, UCB RISC-V, version 1 (SYSV), statically linked, with debug_info, not stripped
   ```

   它是一个64位的elf格式的可执行文件，架构是RISC-V，链接方式是静态链接，no stripped指的是里面符号表的信息未被剔除。

4. 使用`rust-objdump target/riscv64imac-unknown-none-elf/debug/os -x --arch-name=riscv64`查看更具体的信息

   ```
   target/riscv64imac-unknown-none-elf/debug/os:   file format ELF64-riscv
   
   architecture: riscv64
   start address: 0x0000000000011120
   
   Program Header:
       PHDR off    0x0000000000000040 vaddr 0x0000000000010040 paddr 0x0000000000010040 align 2**3      
            filesz 0x00000000000000e0 memsz 0x00000000000000e0 flags r--
       LOAD off    0x0000000000000000 vaddr 0x0000000000010000 paddr 0x0000000000010000 align 2**12     
            filesz 0x0000000000000120 memsz 0x0000000000000120 flags r--
       LOAD off    0x0000000000000120 vaddr 0x0000000000011120 paddr 0x0000000000011120 align 2**12     
            filesz 0x0000000000000004 memsz 0x0000000000000004 flags r-x
      STACK off    0x0000000000000000 vaddr 0x0000000000000000 paddr 0x0000000000000000 align 2**64     
            filesz 0x0000000000000000 memsz 0x0000000000000000 flags rw-
   
   Dynamic Section:
   Sections:
   Idx Name            Size     VMA              Type
     0                 00000000 0000000000000000
     1 .text           00000004 0000000000011120 TEXT
     2 .debug_str      0000043e 0000000000000000
     3 .debug_abbrev   00000113 0000000000000000
     4 .debug_info     0000053c 0000000000000000
     5 .debug_aranges  00000040 0000000000000000
     6 .debug_ranges   00000030 0000000000000000
     7 .debug_pubnames 000000a4 0000000000000000
     8 .debug_pubtypes 00000308 0000000000000000
     9 .debug_frame    00000050 0000000000000000
    10 .debug_line     0000005b 0000000000000000
    11 .comment        00000013 0000000000000000
    12 .symtab         00000108 0000000000000000 
    13 .shstrtab       000000a5 0000000000000000
    14 .strtab         0000002d 0000000000000000
   
   SYMBOL TABLE:
   0000000000000000 l    df *ABS*  00000000 3gqd1qcioyc9uzqc
   0000000000011120         .text  00000000
   0000000000011120         .text  00000000
   0000000000011120         .text  00000000
   0000000000011124         .text  00000000 
   0000000000000000         .debug_info    00000000
   0000000000000000         .debug_ranges  00000000
   0000000000000000         .debug_frame   00000000
   0000000000000000         .debug_line    00000000 .Lline_table_start0
   0000000000011120 g     F .text  00000004 _start
   ```

   * start address是程序入口
   * Sections：从这里我们可以看到程序各段的各种信息。后面以 debug 开头的段是调试信息
   * SYMBOL TABLE：符号表，从中我们可以看到程序中所有符号的地址。例如 `_start` 函数就位于入口地址上
   * Program Header：程序加载时所需的段信息
     * 其中的 off 是它在文件中的位置，vaddr 和 paddr 是要加载到的虚拟地址和物理地址，align 规定了地址的对齐，filesz 和 memsz 分别表示它在文件和内存中的大小，flags 描述了相关权限（r 表示可读，w 表示可写，x 表示可执行）

5. 之后利用`rust-objdump`的`-d`选项进行反编译

   `rust-objdump target/riscv64imac-unknown-none-elf/debug/os -d --arch-name=riscv64`

   ```
   target/riscv64imac-unknown-none-elf/debug/os:   file format ELF64-riscv
   
   
   Disassembly of section .text:
   
   0000000000011120 _start:
      11120: 09 a0                         j       2
      11122: 01 a0                         j       0
   ```

   _start函数中只有一个循环。

6. 生成镜像

   之前生成的elf文件具有以下特点：

   * 含有冗余的调试信息，使得程序体积较大
   * 需要对 Program Header 部分进行手动解析才能知道各段的信息，而这需要我们了解 Program Header 的二进制格式，并以字节为单位进行解析

   使用`rust-objcopy`从elf格式可执行文件中生成内核镜像

   ```shell
   rust-objcopy target/riscv64imac-unknown-none-elf/debug/os --strip-all -O binary target/riscv64imac-unknown-none-elf/debug/kernel.bin
   ```

    `--strip-all` 表明丢弃所有符号表及调试信息，`-O binary` 表示输出为二进制文件。

在使用QEMU模拟器运行起来之前，还需要**调整内存布局**和**重写入口函数**。

### 调整内存布局

在上一步骤中，程序入口点在0x0011120的位置。

对于普通用户程序来说，数据是放在低地址空间上的。但是对于 OS 内核，它一般都在高地址空间上。并且在 RISC-V 中，内存的物理地址也是从 0x80000000 开始的。因此接下来需要调整程序的内存布局，改变它的链接地址。

> 一般来说，一个程序按照功能不同会分为下面这些段：
>
> - .text 段：代码段，存放汇编代码
> - .rodata 段：只读数据段，顾名思义里面存放只读数据，通常是程序中的常量
> - .data 段：存放被初始化的可读写数据，通常保存程序中的全局变量
> - .bss 段：存放被初始化为 0 的可读写数据，与 .data 段的不同之处在于我们知道它要被初始化为 0，因此在可执行文件中只需记录这个段的大小以及所在位置即可，而不用记录里面的数据，也不会实际占用二进制文件的空间
> - Stack：栈，用来存储程序运行过程中的局部变量，以及负责函数调用时的各种机制。它从高地址向低地址增长
> - Heap：堆，用来支持程序**运行过程中**内存的**动态分配**，比如说你要读进来一个字符串，在你写程序的时候你也不知道它的长度究竟为多少，于是你只能在运行过程中，知道了字符串的长度之后，再在堆中给这个字符串分配内存
>
> 内存布局，也就是指这些段各自所放的位置。一种典型的内存布局如下：
>
> ![img](README.assets/typical-layout.png)

编写链接脚本（Linker Script）来指定程序的内存布局。

编写`src/linker.ld`文件

```makefile
/* 有关 Linker Script 可以参考：https://sourceware.org/binutils/docs/ld/Scripts.html */

/* 目标架构 */
OUTPUT_ARCH(riscv)

/* 执行入口 */
ENTRY(_start)

/* 数据存放起始地址 */
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    /* . 表示当前地址（location counter） */
    . = BASE_ADDRESS;

    /* start 符号表示全部的开始位置 */
    kernel_start = .;

    text_start = .;

    /* .text 字段 */
    .text : {
        /* 把 entry 函数放在最前面 */
        *(.text.entry)
        /* 要链接的文件的 .text 字段集中放在这里 */
        *(.text .text.*)
    }

    rodata_start = .;

    /* .rodata 字段 */
    .rodata : {
        /* 要链接的文件的 .rodata 字段集中放在这里 */
        *(.rodata .rodata.*)
    }

    data_start = .;

    /* .data 字段 */
    .data : {
        /* 要链接的文件的 .data 字段集中放在这里 */
        *(.data .data.*)
    }

    bss_start = .;

    /* .bss 字段 */
    .bss : {
        /* 要链接的文件的 .bss 字段集中放在这里 */
        *(.sbss .bss .bss.*)
    }

    /* 结束地址 */
    kernel_end = .;
}
```

> OpenSBI 将自身放在 0x80000000，完成初始化后会跳转到 0x80200000，因此 `_start` 必须位于这个地址。.text 为代码段标识，其第一个放置的就是 `_start`（即 `.text.entry`）。

 .text.entry是后面自己定义的段。

使用链接脚本，在`.cargo/config`文件中添加下面配置：

```properties
# 使用我们的 linker script 来进行链接
[target.riscv64imac-unknown-none-elf]
rustflags = [
    "-C", "link-arg=-Tsrc/linker.ld",
]
```

它的作用是在链接时传入一个参数 `-T` 来指定使用哪个链接脚本。

在重新编译后端的信息如下：

```
target/riscv64imac-unknown-none-elf/debug/os:   file format ELF64-riscv

architecture: riscv64
start address: 0x0000000080200000

Program Header:
    LOAD off    0x0000000000001000 vaddr 0x0000000080200000 paddr 0x0000000080200000 align 2**12     
         filesz 0x0000000000000004 memsz 0x0000000000000004 flags r-x
   STACK off    0x0000000000000000 vaddr 0x0000000000000000 paddr 0x0000000000000000 align 2**64     
         filesz 0x0000000000000000 memsz 0x0000000000000000 flags rw-

Dynamic Section:
Sections:
Idx Name            Size     VMA              Type
  0                 00000000 0000000000000000
  1 .text           00000004 0000000080200000 TEXT
  2 .debug_str      0000043e 0000000000000000
  3 .debug_abbrev   00000113 0000000000000000
  4 .debug_info     0000053c 0000000000000000
  5 .debug_aranges  00000040 0000000000000000
  6 .debug_ranges   00000030 0000000000000000 
  7 .debug_pubnames 000000a4 0000000000000000
  8 .debug_pubtypes 00000308 0000000000000000
  9 .debug_frame    00000050 0000000000000000
 10 .debug_line     0000005b 0000000000000000
 11 .comment        00000013 0000000000000000
 12 .symtab         000001b0 0000000000000000
 13 .shstrtab       000000a5 0000000000000000
 14 .strtab         0000007f 0000000000000000 

SYMBOL TABLE:
0000000000000000 l    df *ABS*  00000000 3gqd1qcioyc9uzqc
0000000080200000         .text  00000000
0000000080200000         .text  00000000
0000000080200000         .text  00000000
0000000080200004         .text  00000000
0000000000000000         .debug_info    00000000
0000000000000000         .debug_ranges  00000000
0000000000000000         .debug_frame   00000000
0000000000000000         .debug_line    00000000 .Lline_table_start0
0000000080200000 g     F .text  00000004 _start
0000000080200000         *ABS*  00000000 BASE_ADDRESS
0000000080200000         .text  00000000 kernel_start
0000000080200000         .text  00000000 text_start
0000000080200004         .text  00000000 rodata_start
0000000080200004         .text  00000000 data_start
0000000080200004         .text  00000000 bss_start
0000000080200004         .text  00000000 kernel_end

Disassembly of section .text:

0000000080200000 text_start:
80200000: 09 a0                         j       2
80200002: 01 a0                         j       0
```

程序已经被放入了0x080200000这个位置。

### 重写程序入口点_start

> **第一条指令**
>
> 在 CPU 加电或 Reset 后，它首先会进行自检（POST, Power-On Self-Test），通过自检后会跳转到**启动代码（Bootloader）**的入口。在 bootloader 中，我们进行外设探测，并对内核的运行环境进行初步设置。随后，bootloader 会将内核代码从硬盘加载到内存中，并跳转到内核入口，正式进入内核。也就是说，CPU 所执行的第一条指令其实是指 bootloader 的第一条指令。

qemu已经实现了OpenSBI固件（Firmware）

> **Firmware 固件**
>
> 在计算中，固件是一种特定的计算机软件，它为设备的特定硬件提供低级控制进一步加载其他软件的功能。固件可以为设备更复杂的软件（如操作系统）提供标准化的操作环境，或者，对于不太复杂的设备，充当设备的完整操作系统，执行所有控制、监视和数据操作功能。在基于 x86 的计算机系统中, BIOS 或 UEFI 是一种固件；在基于 RISC-V 的计算机系统中，OpenSBI 是一种固件。

OpenSBI 固件运行在特权级别很高的计算机硬件环境中，即 RISC-V 64 的 M Mode（CPU 加电后也就运行在 M Mode），我们将要实现的 OS 内核运行在 S Mode，而我们要支持的用户程序运行在 U Mode。在开发过程中我们重点关注 S Mode。

> **RISC-V 64 的特权级**
>
> RISC-V 共有 3 种特权级，分别是 U Mode（User / Application 模式）、S Mode（Supervisor 模式）和 M Mode（Machine 模式）。
>
> 从 U 到 S 再到 M，权限不断提高，这意味着你可以使用更多的特权指令，访需求权限更高的寄存器等等。我们可以使用一些指令来修改 CPU 的**当前特权级**。而当当前特权级不足以执行特权指令或访问一些寄存器时，CPU 会通过某种方式告诉我们。

OpenSBI 所做的一件事情就是把 CPU 从 M Mode 切换到 S Mode，接着跳转到一个固定地址 0x80200000，开始执行内核代码。

> **RISC-V 的 M Mode**
>
> Machine 模式是 RISC-V 中可以执行的最高权限模式。在机器态下运行的代码对内存、I/O 和一些对于启动和配置系统来说必要的底层功能有着完全的使用权。
>
> **RISC-V 的 S Mode**
>
> Supervisor 模式是支持现代类 Unix 操作系统的权限模式，支持现代类 Unix 操作系统所需要的基于页面的虚拟内存机制是其核心。

修改main.rs文件

```rust
/// 覆盖 crt0 中的 _start 函数
/// 我们暂时将它的实现为一个死循环
/// - `#[no_mangle]`
/// 函数禁用编译期间的名称重整（Name Mangling），即确保编译器生成一个名为 _start 的函数
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
--------------------------------------------------------------------
/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    loop {}
}
```

添加`src/entry.asm`

```assembly
# 操作系统启动时所需的指令以及字段
#
# 我们在 linker.ld 中将程序入口设置为了 _start，因此在这里我们将填充这个标签
# 它将会执行一些必要操作，然后跳转至我们用 rust 编写的入口函数
#
# 关于 RISC-V 下的汇编语言，可以参考 https://github.com/riscv/riscv-asm-manual/blob/master/riscv-asm.md

# 们将 _start 函数放在了 .text 段的开头。
    .section .text.entry
    .globl _start
# 目前 _start 的功能：将预留的栈空间写入 $sp，然后跳转至 rust_main
_start:
    # 栈是从高地址往低地址增长，所以高地址是初始的栈顶
    la sp, boot_stack_top
    # 正式进入内核。
    call rust_main

    # 回忆：bss 段是 ELF 文件中只记录长度，而全部初始化为 0 的一段内存空间
    # 这里声明字段 .bss.stack 作为操作系统启动时的栈
    .section .bss.stack
    .global boot_stack
boot_stack:
    # 4096×4 Bytes = 16K 启动栈大小  启动时内核的栈
    .space 4096 * 16
    .global boot_stack_top
boot_stack_top:
    # 栈结尾
```

### 使用QEMU运行

