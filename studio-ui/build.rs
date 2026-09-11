fn main() {
    // let material_path = env::var_os("SLINT_MATERIAL_PATH")
    //     .map(PathBuf::from)
    //     .expect("SLINT_MATERIAL_PATH must be set")
    //     .join("material.slint");

    // let config = slint_build::CompilerConfiguration::new()
    //     .with_library_paths([("material".into(), material_path)].into_iter().collect());

    slint_build::compile("src/ui/app-window.slint").unwrap();
}
