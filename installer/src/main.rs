use fs_extra::dir::CopyOptions;

fn main() { 
    let directories =
        directories::ProjectDirs::from("rocks.sambowden", "", "zonkey-browser").unwrap();

    std::fs::remove_dir_all(directories.data_dir()).ok();
    std::fs::create_dir_all(directories.data_dir()).ok();

    fs_extra::dir::copy(
        "zonkey-data",
        directories.data_dir(),
        &CopyOptions::new().content_only(true),
    )
    .unwrap();

    println!("Installed the required zonkey data files successfully");
}
