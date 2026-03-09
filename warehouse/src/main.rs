mod inventory;
mod orders;

use inventory::{FLOOR_SPACE, MANAGER, talk_to_manager};
// use inventory::products::ProductCategory;
// use inventory::products::Item;
use inventory::products::{self, ProductCategory};

/*
    3 ways to declare a module.
    1) in-line declaration
    2) file with the name of the module
    3) folder with the name of the module, with a mod.rs file within it

    The Rust compiler will throw an error/warning when it finds two or modules with the same name.

    A submodule is a module that lives with another module (sub/child module).
    1) inline
    2) inventory/products.rs
    3) inventory/products/mod.rs

    This creates a hierarchial data structure of files and folders (modules). The top most node is the crate root.

    There are two ways to reach a module.
    - An absolute path is the full complete path to name starting from the crate root.
    - A relative path is the path to a name starting from the current location or module.

    The use keyword brings a name into the current scope. It creates a "shortcut" to a name in a nested module. There can be only one existence of a name within a single file.
*/
fn main() {
    println!("\nWarehouse Project");
    println!(
        "- This project is intended to understand Rust file/folder structuring (modules & crates)."
    );

    println!("\nThe manager of our inventory is {}.", MANAGER);
    println!("The available floor space is {} sqft.", FLOOR_SPACE);
    talk_to_manager();

    println!("\nThe manager of our orders is {}.", orders::MANAGER);
    println!("The available floor space is {} sqft.", orders::FLOOR_SPACE);
    orders::talk_to_manager();

    let favorite_category = ProductCategory::Ladder;
    println!("\nMy favorite category of items is {favorite_category:?}.");

    let tall_ladder = products::Item {
        name: String::from("Premium Folding Ladder"),
        category: favorite_category,
        quantity: 10,
    };
    println!("\ntall_ladder: {tall_ladder:#?}")
}
