fn main() -> std::io::Result<()> {
    let directories =
        directories::ProjectDirs::from("rocks.sambowden", "", "zonkey-browser").unwrap();

    std::fs::create_dir_all(directories.data_dir())?;

    std::fs::copy(
        "../internal_scripts/home.zonk",
        directories.data_dir().join("home.zonk"),
    )?;
    std::fs::copy(
        "../internal_scripts/settings.zonk",
        directories.data_dir().join("settings.zonk"),
    )?;
    std::fs::copy(
        "../internal_scripts/invalid.zonk",
        directories.data_dir().join("invalid.zonk"),
    )?;

    println!("Installed required files successfully");

    Ok(())
}
