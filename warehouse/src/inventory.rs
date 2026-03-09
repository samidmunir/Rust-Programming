pub mod products;

pub const MANAGER: &str = "Sami M.";
pub const FLOOR_SPACE: i32 = 10000;

pub fn talk_to_manager() {
    println!("\ninventory.talk_to_manager() called...");
    println!("- Hey, {MANAGER}, how is the inventory looking like today?");
}
