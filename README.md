# CMake项目生成工具 (ccmk)

一个简单的命令行工具，用于快速生成CMake项目结构。

## 功能

- 交互式创建CMake项目
- 支持C和C++项目
- 支持可执行文件和静态库项目
- 自动创建项目目录结构
- 自动生成CMakeLists.txt和示例源代码
- 自动处理项目名中的特殊字符

## 安装

1. 确保已安装Rust工具链
2. 克隆本项目
3. 运行以下命令安装：

```bash
cargo install --path .
```

## 使用方法

1. 运行命令：

```bash
ccmk
```

2. 按照提示输入：
   - 项目名称（会自动处理特殊字符）
   - 编程语言（C或C++）
   - 项目类型（可执行文件或静态库）
   - C++标准版本（如果选择C++）

3. 工具会自动创建项目目录和文件

## 生成的项目结构

```
项目名/
├── CMakeLists.txt
├── include/
└── src/
    └── main.c或main.cpp
```

## 示例

```bash
$ ccmk
请输入项目名称：My Project
请选择编程语言：C++
请选择项目类型：Static Library
请选择C++标准版本：17
✅ 项目已成功创建！
📁 项目结构：
   - MyProject/
     ├── CMakeLists.txt
     ├── include/
     └── src/
         └── main.cpp
```

## 注意事项

- 项目名中的特殊字符（如空格、标点符号）会被自动移除
- 如果清理后的项目名为空，会使用默认名称"cmake_project"
- 生成的CMakeLists.txt使用CMake 3.14作为最低版本要求