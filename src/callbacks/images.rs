use slint::{ComponentHandle, Image, VecModel};
use std::{error::Error, path::PathBuf, rc::Rc};

use crate::{AppConfig, AppWindow};

pub fn process_get_images(app: &AppWindow) -> Result<(), Box<dyn Error>> {
    // --- Your array/Vec of image paths ---
    let image_paths: Vec<PathBuf> = vec![
        PathBuf::from("ui/icons/close.svg"),
        PathBuf::from("ui/icons/more.svg"),
        PathBuf::from("ui/icons/close.svg"),
        PathBuf::from("ui/icons/more.svg"),
        PathBuf::from("ui/icons/close.svg"),
        PathBuf::from("ui/icons/more.svg"),
        PathBuf::from("ui/icons/close.svg"),
        PathBuf::from("ui/icons/more.svg"),
        PathBuf::from("ui/icons/close.svg"),
        PathBuf::from("ui/icons/more.svg"),
        PathBuf::from("ui/icons/close.svg"),
        PathBuf::from("ui/icons/more.svg"),
        PathBuf::from("ui/icons/close.svg"),
        PathBuf::from("ui/icons/more.svg"),
        PathBuf::from("ui/icons/close.svg"),
        PathBuf::from("ui/icons/more.svg"),
        PathBuf::from("ui/icons/close.svg"),
        PathBuf::from("ui/icons/more.svg"),
        PathBuf::from("ui/icons/close.svg"),
        PathBuf::from("ui/icons/more.svg"),
        PathBuf::from("ui/icons/close.svg"),
        PathBuf::from("ui/icons/more.svg"),
        PathBuf::from("ui/icons/close.svg"),
        PathBuf::from("ui/icons/more.svg"),
    ];

    // --- Load images from paths and collect into a Vec<slint::Image> ---
    let mut loaded_icons: Vec<Image> = Vec::new();

    for path in image_paths {
        match Image::load_from_path(&path) {
            Ok(image) => {
                // Image loaded successfully, add it to our vector
                loaded_icons.push(image);
            }
            Err(e) => {
                // Failed to load the image. Print an error and skip this path.
                // You could push a default "broken image" placeholder here if you have one.
                eprintln!("Error loading image {}: {}", path.display(), e);
            }
        }
    }

    let items_model = Rc::new(VecModel::from(loaded_icons));

    let config = app.global::<AppConfig>();
    config.set_icons(items_model.clone().into());

    Ok(()) // Return Ok if everything ran without panicking
}
