use clap::{ 
    Parser, 
    Subcommand,
    ValueEnum // 新增
};

use dialoguer::{
    theme::ColorfulTheme, 
    Select
};


#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Name {
    N1,
    N2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Address {
    A1,
    A2
}


#[derive(Parser,Debug)]
#[command(version, about)]
struct Cli {
    #[arg(default_value = "fe")]
    name: String,
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand, Debug)]
enum Commands {
    Create {
        #[arg(
            short = 'n',
            long="name",
            help = "用户信息",
            default_value = "fe"
        )]
        name: String,
        #[arg(
            short = 'a',
            long="address",
            help = "地址信息",
            requires = "name",
            default_value = "北京"
        )]
        address: String,
    },
    Replace,
    Update,
    Delete
}

fn main() {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::Replace) {
        Commands::Create { name, address } => {
            let items = &["React", "Vue"]; // 添加其他选项
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("请选择要创建的项目类型")
                .items(items)
                .default(0)
                .interact()
                .unwrap();

            match selection {
                0 => println!("选择了 React"),
                1 => println!("选择了 Vue"),
                _ => println!("未知选项"),
            }

             // 打印用户提供的姓名和地址
            println!("用户信息: {}", name);
            println!("地址信息: {}", address);
        },
        Commands::Replace => println!("执行 Replace 命令"),
        Commands::Update => println!("执行 Update 命令"),
        Commands::Delete => println!("执行 Delete 命令"),
    }
}
