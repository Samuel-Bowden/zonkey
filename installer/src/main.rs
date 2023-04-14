use fs_extra::dir::CopyOptions;

fn main() {
    let directories =
        directories::ProjectDirs::from("rocks.sambowden", "", "zonkey-browser").unwrap();

    std::fs::create_dir_all(directories.data_dir()).unwrap();

    fs_extra::dir::copy(
        "default-profile",
        directories.data_dir(),
        &CopyOptions::new().content_only(true),
    )
    .unwrap();

    println!("Installed profile successfully");
}
