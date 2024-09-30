use std::{fs, thread::sleep, time::Duration};

use rand::Rng;

fn main() {
    run_recursively()
}

fn run_recursively() {
    let picked_wallpaper_file_path = pick_random_wallpaper_file();

    set_wallpaper(&picked_wallpaper_file_path).expect("error setting new wallpaper");
}

fn pick_random_wallpaper_file() -> std::ffi::OsString {
    let home_path = match home::home_dir() {
        Some(path) => {
            if path.as_os_str().is_empty() {
                panic!("could not find home path");
            }

            path
        }
        _ => panic!("could not find home path"),
    };

    let wallpaper_dir_path = home_path.as_path().join("wallpapers");

    let wallpaper_files = fs::read_dir(&wallpaper_dir_path).unwrap_or_else(|error_msg| {
        panic!(
            "could not read wallpapers directory, directory {:?} is missing; error message: {}",
            &wallpaper_dir_path.as_os_str(),
            error_msg
        )
    });

    let mut wallpaper_list: Vec<fs::DirEntry> = vec![];

    for path_result in wallpaper_files {
        let path = path_result.expect("path in wallpaper dir doesn't exist");

        wallpaper_list.push(path);
    }

    let rand_number = rand::thread_rng().gen_range(0..wallpaper_list.len() - 1);

    let picked_wallpaper_path = wallpaper_list[rand_number].path();

    picked_wallpaper_path.as_os_str().to_owned()
}

fn set_wallpaper(new_wallpaper_path: &std::ffi::OsString) -> Result<(), std::io::Error> {
    let result = std::process::Command::new("swaybg")
        .arg("-m")
        .arg("fill")
        .arg("-i")
        .arg(new_wallpaper_path.to_str().unwrap())
        .spawn();

    match result {
        Ok(_child) => Ok(()),
        Err(error) => Err(error),
    }
}
