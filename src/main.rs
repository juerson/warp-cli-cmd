use std::process::Command;
use std::io::{self, Write};
use regex::Regex; // 引入正则表达式库

fn main() {
	print!("按Enter键，正式进入设置WARP的优选端点IP！");
	wait_for_enter();
    // 第一步：执行命令 "C:\Program Files\Cloudflare\Cloudflare WARP\warp-cli.exe clear-custom-endpoint"
    execute_command("C:\\Program Files\\Cloudflare\\Cloudflare WARP\\warp-cli.exe", &["clear-custom-endpoint"]);
    // 第二步：用户输入一个合法的 ip:port 格式的地址
    let endpoint = read_endpoint_from_user();
    
    // 第三步：执行命令 "C:\Program Files\Cloudflare\Cloudflare WARP\warp-cli.exe set-custom-endpoint {endpoint}"
    execute_command("C:\\Program Files\\Cloudflare\\Cloudflare WARP\\warp-cli.exe", &["set-custom-endpoint", &endpoint]);

    // 添加等待输入以保持命令行窗口打开
	print!("按Enter键退出程序...");
    wait_for_enter();
}

fn execute_command(command: &str, args: &[&str]) {
	println!("执行命令: {} {}", command, args.join(" "));
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("命令执行失败");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("执行状态: {}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("执行状态: {}", stderr);
    }
}

fn read_endpoint_from_user() -> String {
    let mut endpoint = String::new();
	let ip_port_regex = Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d+$").unwrap();

    loop {
        print!("请输入IP:Port格式的地址(e.g.188.114.97.10:890)：");
        io::stdout().flush().expect("Failed to flush stdout");

        endpoint.clear();
        io::stdin().read_line(&mut endpoint).expect("读取输入内容失败");

        let endpoint = endpoint.trim();
        
        // 使用正则表达式检测是否符合 IP:Port 格式
        if ip_port_regex.is_match(endpoint) {
            return endpoint.to_string();
        } else {
            println!("无效的格式。请输入IP:Port格式的地址.");
        }
    }
}

fn wait_for_enter() {
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
}

