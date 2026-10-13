//! Build script embedding the Windows UAC manifest into the final executable.

fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_manifest_file("app.manifest");
        if let Err(e) = res.compile() {
            eprintln!("Failed to compile Windows resource manifest: {}", e);
        }
    }
}
