Description

firefox_cookies_export is a Rust-based command line utility tool designed to export and import cookies from Firefox browser. The exported cookies are saved in an encrypted file, and can be restored back to the browser whenever needed. This is particularly useful for backup or transfer of cookies between different systems or profiles.
Usage

    Export Cookies:
        Run the program and select option 1 to save the current cookies.
        Enter the encryption password when prompted.

    Import Cookies:
        Select option 2 to restore cookies.
        Enter the encryption password when prompted.

    Exit:
        Select option 3 to exit the program.

Commands

    Run the program:

    bash

    cargo run

Dependencies

    Rust: Ensure you have the latest version of Rust installed.
    dirs crate: Used for fetching the home directory path.

Installation

    Clone the repository:

    bash

git clone https://github.com/your-username/firefox_cookies_export

Navigate to the project directory:

bash

cd firefox_cookies_export

Build the project:

bash

    cargo build --release

Future Features

    Complete Bookmark and History Backup:
        Future iterations of this tool will include the ability to export and import bookmarks and browser history alongside cookies.

    Profile Selection:
        Users will be able to select the Firefox profile they wish to export cookies from or import cookies to.

    Enhanced Encryption Method:
        Improved encryption techniques will be applied to ensure the security of the exported data.

Contributing

If you would like to contribute to the development of this tool, feel free to fork the repository, make your changes, and open a pull request. Any contributions are highly appreciated!
License

This project is licensed under the MIT License. See the LICENSE file for details.
