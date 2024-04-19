use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use regex::Regex;
use std::{
    io::{self, Write},
    process::Command,
};

fn main() {
    println!("设置WARP优选IP的小工具(只支持2024以及后面的版本)");
    let command_menu = r#"
  【1】删除旧的Endpoint(重置)
  【2】设置新的Endpoint
  【3】查看WARP的连接状态
  【4】清除当前屏幕的内容
  【5】退出程序，或者按"Ctrl + C"组合键退出程序
"#;
    print_colored_text(Color::Blue, command_menu);
    loop {
        println!("\n——————————————————————————————————————————————————————————————————————————");
        print!("请选择前面菜单的数字：");
        io::stdout().flush().expect("无法刷新缓冲区");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("无法读取输入");

        match input.trim() {
            "1" => {
                execute_command("warp-cli", &["tunnel", "endpoint", "reset"]);
            }
            "2" => {
                let endpoint = read_endpoint_from_user();
                execute_command("warp-cli", &["tunnel", "endpoint", "set", &endpoint]);
            }
            "3" => {
                execute_command("warp-cli", &["tunnel", "stats"]);
            }
            "4" => {
                // 清除了终端上的所有内容
                print!("\x1B[2J\x1B[1;1H"); // ANSI escape sequence 清屏
                io::stdout().flush().expect("无法刷新缓冲区");
                print_colored_text(Color::Blue, command_menu);
            }
            "5" => {
                break;
            }
            _ => {
                //
            }
        }
    }
}

fn execute_command(command: &str, args: &[&str]) {
    print!("\nWARP-CLI命令，正在执行: ");
    io::stdout().flush().expect("Failed to flush stdout");
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Green),
        Print(format!("{} {}", command, args.join(" "))),
        ResetColor,
    )
    .expect("Failed to execute command");
    io::stdout().flush().expect("Failed to flush stdout");

    let output = Command::new(command)
        .args(args)
        .output()
        .expect("命令执行失败");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if args.contains(&"stats") {
            println!("\nWARP-CLI命令，执行状态:");
            io::stdout().flush().expect("Failed to flush stdout");
            print_colored_text(Color::DarkRed, &stdout);
            io::stdout().flush().expect("Failed to flush stdout");
        } else {
            print!("\nWARP-CLI命令，执行状态: ");
            io::stdout().flush().expect("Failed to flush stdout");
            print_colored_text(Color::DarkGrey, &stdout);
            io::stdout().flush().expect("Failed to flush stdout");
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("WARP-CLI命令，执行失败: {}", stderr);
    }
}

fn read_endpoint_from_user() -> String {
    let mut endpoint = String::new();
    let ip_port_regex = Regex::new(r"\b^(?:[0-9]{1,3}\.){3}[0-9]{1,3}:[0-9]{1,5}$\b").unwrap();

    loop {
        print!("\n请输入Endpoint的地址(P.S. 162.159.192.1:2408)：");
        io::stdout().flush().expect("Failed to flush stdout");

        endpoint.clear();
        io::stdin()
            .read_line(&mut endpoint)
            .expect("读取输入内容失败");

        let endpoint = endpoint.trim();
        // 使用正则表达式检测是否符合 IP:PORT 格式
        if ip_port_regex.is_match(endpoint) {
            return endpoint.to_string();
        } else {
            //
        }
    }
}

fn print_colored_text(color: Color, text: &str) {
    // 设置前景色并打印文本
    execute!(
        io::stdout(),
        SetForegroundColor(color),
        Print(text),
        ResetColor,
    )
    .expect("Failed to execute command");
}
