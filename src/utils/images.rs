// Expose slint_icons to your UI as the `collection_icons` property
// ui.set_collection_icons(slint_icons.into());

// --- Search Function in your backend ---
fn find_matching_icons(search_string: &str, paths: &[String]) -> Vec<usize> {
    // Backend side
    let all_icon_paths = vec![
        "../icons/folder.svg".to_string(),
        "../icons/file.svg".to_string(),
        // ... other paths
    ];

    // You'd load these into Slint's Image objects to expose to the UI
    let slint_icons: Vec<slint::Image> = all_icon_paths
        .iter()
        .map(|path| slint::Image::load_from_path(path.as_ref()).unwrap()) // Load images
        .collect();

    paths
        .iter()
        .enumerate()
        .filter(|(_index, path)| path.contains(search_string)) // Or starts_with, equals, etc.
        .map(|(index, _path)| index)
        .collect() // Returns a list of indices of matches
}

// When the user types in a search input in Slint and triggers a search event:
// ui.on_search_triggered(|search_text| {
//     let matching_indices = find_matching_icons(&search_text, &all_icon_paths);
//     // Now use these indices to highlight/filter elements in the UI
//     // You might update another list property in Slint, e.g., `matching_indices_slint: Vec<i32>`
//     ui.set_matching_indices_slint(matching_indices.iter().map(|&i| i as i32).collect::<Vec<_>>().into());
// });
