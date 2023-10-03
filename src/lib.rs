use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use dirs::home_dir;


pub fn find_latest_firefox_profile() -> io::Result<PathBuf> {
    let profiles_dir = home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find home directory"))?
        .join("AppData/Roaming/Mozilla/Firefox/Profiles");

    let (latest_profile_path, _) = fs::read_dir(&profiles_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_dir() {
                let modified = entry.metadata().ok()?.modified().ok()?;
                Some((entry.path(), modified))
            } else {
                None
            }
        })
        .max_by_key(|&(_, modified)| modified)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No profiles found"))?;

    Ok(latest_profile_path)
}

pub fn restore_firefox_cookies() -> io::Result<()> {
    println!("Enter encryption password: ");
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim();

    let mut encrypted_file = File::open("firefox_cookies.bak")?;
    let mut encrypted_data = Vec::new();
    encrypted_file.read_to_end(&mut encrypted_data)?;

    let decrypted_data = xor_encrypt(&encrypted_data, &password);

    let profile_path = find_latest_firefox_profile()?;
    let cookies_path = profile_path.join("cookies.sqlite");
    let mut output_file = File::create(&cookies_path)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("無法創建或打開檔案: {}", e)))?;
    output_file.write_all(&decrypted_data)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("無法寫入檔案: {}", e)))?;

    println!("Decrypted cookies restored to {:?}", cookies_path);
    Ok(())
}

pub fn save_firefox_cookies() -> io::Result<()> {
    let profile_path = find_latest_firefox_profile()?;
    let cookies_path = profile_path.join("cookies.sqlite");

    println!("Enter encryption password: ");
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim();

    let mut cookies_file = File::open(&cookies_path)?;
    let mut cookies_data = Vec::new();
    cookies_file.read_to_end(&mut cookies_data)?;

    let encrypted_data = xor_encrypt(&cookies_data, &password);

    let mut output_file = File::create("firefox_cookies.bak")?;
    output_file.write_all(&encrypted_data)?;

    Ok(())
}

pub fn xor_encrypt(data: &[u8], key: &str) -> Vec<u8> {
    let key_bytes = key.as_bytes();
    let mut encrypted_data = Vec::new();
    for (i, byte) in data.iter().enumerate() {
        let key_byte = key_bytes[i % key_bytes.len()];
        encrypted_data.push(byte ^ key_byte);
    }
    encrypted_data
}

pub fn execute_choice(choice: &str) -> io::Result<()> {
    match choice {
        "1" => {
            match save_firefox_cookies() {
                Ok(_) => println!("Cookies已保存"),
                Err(err) => println!("保存失敗: {:?}", err),
            }
        },
        "2" => {
            match restore_firefox_cookies() {
                Ok(_) => println!("Cookies已還原"),
                Err(err) => println!("還原失敗: {:?}", err),
            }
        },
        _ => println!("無效的選擇，請再試一次"),
    }

    Ok(())
}
