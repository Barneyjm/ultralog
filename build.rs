fn main() {
    // Only run on Windows
    #[cfg(windows)]
    {
        // Embed icon and version info into Windows executable
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico")
            .set("ProductName", "UltraLog")
            .set("FileDescription", "High-performance ECU log viewer")
            .set("LegalCopyright", "Copyright (c) 2024 Cole Gentry");

        // Only compile if icon exists
        if std::path::Path::new("assets/icon.ico").exists() {
            res.compile().expect("Failed to compile Windows resources");
        }
    }
}
