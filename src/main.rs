use firefox_cookies_export::execute_choice;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    loop {
        println!("Firefox Cookies儲存匯出");
        println!("1. 保存目前cookies");
        println!("2. 還原cookies");
        println!("3. 退出");
        print!("請選擇一個操作: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        
        if choice.trim() == "3" {
            break;
        }

        execute_choice(choice.trim())?;
    }
    Ok(())
}
