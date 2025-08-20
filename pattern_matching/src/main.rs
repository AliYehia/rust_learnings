fn main() {
    enum ItemType {
        Weapon(String, i32),
        Potion(String, i32),
        Armor(String, i32),
    }

    let game_objects = vec![
        ItemType::Weapon("Sword".to_string(), 20),
        ItemType::Weapon("Mace".to_string(), 35),
        ItemType::Potion("Health Potion".to_string(), 20),
        ItemType::Potion("Mana Potion".to_string(), 25),
        ItemType::Armor("Shield".to_string(), 30),
        ItemType::Armor("Boots".to_string(), 20),
        ItemType::Armor("Gloves".to_string(), 15),
    ];

    for object in game_objects {
        match object {
            ItemType::Weapon(weapon_name, damage ) => println!("Weapon: {} has damage: {}", weapon_name, damage),
            ItemType::Potion(potion_name, strength ) => println!("Potion: {} has damage: {}", potion_name, strength),
            ItemType::Armor(armor_name, defence ) => println!("Armor: {} has defence: {}", armor_name, defence),
            _ => println!("Object not coded in the game yet!"),
        }
    }

}
