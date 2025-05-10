use inquire::{Select, Text};
use std::fs;
use std::path::Path;
use regex::Regex;

fn generate_cmakelists(
    project_name: &str,
    lang: &str,
    project_type: &str,
    cxx_standard: &str,
) -> String {
    let mut lines = vec![
        "cmake_minimum_required(VERSION 3.20)".to_string(),
        format!("project({} LANGUAGES {})", project_name, lang.to_uppercase()),
        "".to_string(),
    ];

    // 添加C++标准设置
    if lang == "cxx" {
        lines.push(format!("set(CMAKE_CXX_STANDARD {})", cxx_standard));
        lines.push("set(CMAKE_CXX_STANDARD_REQUIRED ON)".to_string());
        lines.push("".to_string());
    }

    // 处理不同项目类型
    match project_type {
        "exe" => {
            lines.push(format!(
                "add_executable({} src/main.{})",
               project_name,
               if lang == "c" { "c" } else { "cpp" }, // 使用.cpp扩展名但CMake语言为CXX
            ));
        }
        "static_lib" => {
            lines.push(format!(
                "add_library({} STATIC src/main.{})",
               project_name,
               if lang == "c" { "c" } else { "cpp" }, // 使用.cpp扩展名但CMake语言为CXX
            ));
            lines.push(format!("target_include_directories({} PRIVATE include)", project_name));
            lines.push(format!(
                "target_compile_definitions({} PUBLIC {}_EXPORTS)", 
                project_name,
                project_name.to_uppercase()
            ));
        }
        _ => panic!("Unsupported project type"),
    }

    lines.join("\n")
}

fn generate_source_file(lang: &str) -> String {
    if lang == "c" {
        "#include <stdio.h>\n\nint main() {\n    printf(\"Hello, World!\\n\");\n    return 0;\n}".to_string()
    } else {
        "#include <iostream>\n\nint main() {\n    std::cout << \"Hello, World!\" << std::endl;\n    return 0;\n}".to_string()
    }
}

// 清理项目名，移除不适合作为文件夹名的字符
fn sanitize_folder_name(name: &str) -> String {
    // 使用正则表达式替换所有非字母数字、下划线和连字符的字符
    let re = Regex::new(r"[^a-zA-Z0-9_-]").unwrap();
    let sanitized = re.replace_all(name, "").to_string();
    
    // 确保不为空，如果为空则使用默认名称
    if sanitized.is_empty() {
        return "cmake_project".to_string();
    }
    
    sanitized
}

fn main() -> Result<(), inquire::InquireError> {
    // 1. 输入项目名称
    let project_name = Text::new("请输入项目名称：").prompt()?;
    
    // 2. 选择语言类型
    let lang_choices = vec!["C", "C++"];
    let lang = Select::new("请选择编程语言：", lang_choices).prompt()?;
    
    // 3. 选择项目类型
    let type_choices = vec!["Executable", "Static Library"];
    let project_type = Select::new("请选择项目类型：", type_choices).prompt()?;
    
    // 4. 选择C++标准版本（仅当选择C++时）
    let cxx_standard = if lang == "C++" {
        let standard_choices = vec!["11", "14", "17", "20"];
        Select::new("请选择C++标准版本：", standard_choices).prompt()?
    } else {
        &"00".to_string()
    };
    
    // 清理项目名，创建有效的文件夹名
    let folder_name = sanitize_folder_name(&project_name);
    
    // 创建项目文件夹和目录结构
    let output_path = Path::new(".").join(&folder_name);
    fs::create_dir_all(&output_path)?;
    fs::create_dir_all(output_path.join("src"))?;
    fs::create_dir_all(output_path.join("include"))?;
    
    // 写入CMakeLists.txt
    fs::write(
        output_path.join("CMakeLists.txt"),
        generate_cmakelists(
            &project_name,
            if lang == "C" { "c" } else { "cxx" },
            match project_type {
                "Executable" => "exe",
                "Static Library" => "static_lib",
                _ => "exe",
            },
            &cxx_standard,
        ),
    )?;
    
    // 写入示例源文件
    let source_ext = if lang == "C" { "c" } else { "cpp" }; // 源文件扩展名保持.cpp，CMake语言使用CXX
    fs::write(
        output_path.join("src").join(format!("main.{}", source_ext)),
        generate_source_file(if lang == "C" { "c" } else { "cpp" }),
    )?;
    
    println!("✅ 项目已成功创建！");
    println!("📁 项目结构：");
    println!("   - {}/", folder_name);
    println!("     ├── CMakeLists.txt");
    println!("     ├── include/");
    println!("     └── src/");
    println!("         └── main.{}", source_ext);
    
    Ok(())
}
