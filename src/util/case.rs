pub fn to_title_case(s: &str) -> String {
    let mut prev_char = ' ';
    let mut is_first_char = true;
    let mut result = String::new();
    for char in s.chars() {
        if char.is_alphanumeric() {
            if prev_char.is_lowercase() && char.is_uppercase() {
                prev_char = ' ';
            }
            if prev_char.is_alphanumeric() {
                result.push(char);
            } else {
                if !is_first_char { result.push(' '); }
                result.push(char.to_ascii_uppercase());
            }
            is_first_char = false;
        }
        prev_char = char;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::to_title_case;

    fn test(source: &str, expected: &str) {
        assert_eq!(expected, to_title_case(source));
    }

    #[test]
    fn simple() {
        test("fooBar", "Foo Bar");
        test("FooBar", "Foo Bar");
        test("foo-bar", "Foo Bar");
        test("Foo bar", "Foo Bar");
        test("Foo Bar", "Foo Bar");
        test("Foo-Bar", "Foo Bar");
        test("FOO-BAR", "FOO BAR");
        test("foo_bar", "Foo Bar");
    }

    #[test]
    fn complex() {
        test("foo    bar", "Foo Bar");
        test("-_.foo-_.bar-_.", "Foo Bar");
        test("factorio-mod-example", "Factorio Mod Example");
        test("LTN-Language-Pack", "LTN Language Pack");
        test("Noxys StackSizeMultiplier", "Noxys Stack Size Multiplier");
    }

    #[test]
    fn all() {
        test("arachnophobia", "Arachnophobia");
        test("Atomic_Overhaul", "Atomic Overhaul");
        test("AutoDeconstruct", "Auto Deconstruct");
        test("automatic-discharge-defense", "Automatic Discharge Defense");
        test("biters_drop_money", "Biters Drop Money");
        test("biter-trails", "Biter Trails");
        test("brush-tools", "Brush Tools");
        test("commands", "Commands");
        test("cutscene-creator", "Cutscene Creator");
        test("dim_lamps", "Dim Lamps");
        test("diplomacy", "Diplomacy");
        test("eco-friendly-electric-machines", "Eco Friendly Electric Machines");
        test("enemy_race_manager", "Enemy Race Manager");
        test("erm_marspeople", "Erm Marspeople");
        test("erm_redarmy", "Erm Redarmy");
        test("erm_terran", "Erm Terran");
        test("erm_toss", "Erm Toss");
        test("erm_zerg", "Erm Zerg");
        test("ExtendedAngels", "Extended Angels");
        test("extended-factorio", "Extended Factorio");
        test("Factorio.AdvancedAirPurification", "Factorio Advanced Air Purification");
        test("factorio-autobuild", "Factorio Autobuild");
        test("factorio-AutoPauseForAFKplayers", "Factorio Auto Pause For AFKplayers");
        test("factorio-beltlayer", "Factorio Beltlayer");
        test("factorio-cooked-fish", "Factorio Cooked Fish");
        test("factorio-ender-pearl", "Factorio Ender Pearl");
        test("factorio-free-market", "Factorio Free Market");
        test("factorio.InserterCranes", "Factorio Inserter Cranes");
        test("factorio-kill_nest_get_money", "Factorio Kill Nest Get Money");
        test("Factorio.LongStorageTanks", "Factorio Long Storage Tanks");
        test("Factorio.LongWarehouses", "Factorio Long Warehouses");
        test("factorio-lua-compiler", "Factorio Lua Compiler");
        test("FactorioMilestones", "Factorio Milestones");
        test("factorio-minable_tiles", "Factorio Minable Tiles");
        test("factorio-miniloader", "Factorio Miniloader");
        test("factorio-mod-example", "Factorio Mod Example");
        test("Factorio-Modules-T4", "Factorio Modules T4");
        test("factorio-money-UI", "Factorio Money UI");
        test("Factorio-Non-Colliding-Rails", "Factorio Non Colliding Rails");
        test("factorio-ODAD", "Factorio ODAD");
        test("factorio-passive_player_income", "Factorio Passive Player Income");
        test("factorio-passive_team_income", "Factorio Passive Team Income");
        test("factorio-pipelayer", "Factorio Pipelayer");
        test("factorio-Players_info", "Factorio Players Info");
        test("factorio-PrivateElectricity", "Factorio Private Electricity");
        test("factorio-railloader", "Factorio Railloader");
        test("factorio-restrict_building", "Factorio Restrict Building");
        test("factorio-shifted-worlds", "Factorio Shifted Worlds");
        test("factorio-show_my_damage", "Factorio Show My Damage");
        test("factorio-skip-hours", "Factorio Skip Hours");
        test("Factorio.SmallTank", "Factorio Small Tank");
        test("Factorio-Sniper-Rifle", "Factorio Sniper Rifle");
        test("Factorio.SpaceScienceDelivery", "Factorio Space Science Delivery");
        test("Factorio.SpaceShuttle", "Factorio Space Shuttle");
        test("Factorio-Start-With-Nanobots", "Factorio Start With Nanobots");
        test("factorio-surface_floors", "Factorio Surface Floors");
        test("factorio-switchable_mods", "Factorio Switchable Mods");
        test("factorio-tank-pvp", "Factorio Tank Pvp");
        test("factorio-techs_for_science", "Factorio Techs For Science");
        test("factorio-todo-list", "Factorio Todo List");
        test("factorio-trainsaver", "Factorio Trainsaver");
        test("factorio-useful_book", "Factorio Useful Book");
        test("factorio-WhereIsMyBody", "Factorio Where Is My Body");
        test("Factorissimo2", "Factorissimo2");
        test("FactorySearch", "Factory Search");
        test("firework_rockets", "Firework Rockets");
        test("fish-farm", "Fish Farm");
        test("flow-control", "Flow Control");
        test("FreightForwarding", "Freight Forwarding");
        test("glowing_trees", "Glowing Trees");
        test("hiladdars-robots", "Hiladdars Robots");
        test("ick-automatic-train-repair", "Ick Automatic Train Repair");
        test("ickputzdirwech-vanilla-tweaks", "Ickputzdirwech Vanilla Tweaks");
        test("Industrial-Revolution-Language-Pack", "Industrial Revolution Language Pack");
        test("inserter-visualizer", "Inserter Visualizer");
        test("IntermodalContainers", "Intermodal Containers");
        test("LTN-Language-Pack", "LTN Language Pack");
        test("M-Dirigible", "M Dirigible");
        test("misery", "Misery");
        test("m-lawful-evil", "M Lawful Evil");
        test("m-microcontroller", "M Microcontroller");
        test("m-multiplayertrading", "M Multiplayertrading");
        test("ModuleInserterSimplified", "Module Inserter Simplified");
        test("More_Ammo", "More Ammo");
        test("more-fish", "More Fish");
        test("More_Repair_Packs", "More Repair Packs");
        test("Noxys_Achievement_Helper", "Noxys Achievement Helper");
        test("Noxys_Deep_Core_Mining_Tweak", "Noxys Deep Core Mining Tweak");
        test("Noxys_Extra_Settings_Info", "Noxys Extra Settings Info");
        test("Noxys_Fading", "Noxys Fading");
        test("Noxys_Multidirectional_Trains", "Noxys Multidirectional Trains");
        test("Noxys_Robot_Battery_Tweak", "Noxys Robot Battery Tweak");
        test("Noxys_StackSizeMultiplier", "Noxys Stack Size Multiplier");
        test("Noxys_Swimming", "Noxys Swimming");
        test("Noxys_Trees", "Noxys Trees");
        test("Noxys_Waterfill", "Noxys Waterfill");
        test("OmniLocales", "Omni Locales");
        test("OmniSea", "Omni Sea");
        test("PowerOverload", "Power Overload");
        test("prismatic-belts", "Prismatic Belts");
        test("RemoteConfiguration", "Remote Configuration");
        test("reskins-angels", "Reskins Angels");
        test("reskins-bobs", "Reskins Bobs");
        test("reskins-compatibility", "Reskins Compatibility");
        test("reskins-library", "Reskins Library");
        test("rpg_items", "Rpg Items");
        test("SeaBlockCustomPack", "Sea Block Custom Pack");
        test("secondary-chat", "Secondary Chat");
        test("sentient_spiders", "Sentient Spiders");
        test("Shortcuts-ick", "Shortcuts Ick");
        test("show-health-and-shield", "Show Health And Shield");
        test("slashgamble", "Slashgamble");
        test("spell-pack", "Spell Pack");
        test("SpidertronEngineer", "Spidertron Engineer");
        test("SpidertronEnhancements", "Spidertron Enhancements");
        test("SpidertronPatrols", "Spidertron Patrols");
        test("SpidertronWeaponSwitcher", "Spidertron Weapon Switcher");
        test("stationary_chat", "Stationary Chat");
        test("status_bars", "Status Bars");
        test("teams-zo", "Teams Zo");
        test("train-trails", "Train Trails");
        test("vanilla-loaders-hd", "Vanilla Loaders Hd");
        test("vanilla-loaders-hd-krastorio", "Vanilla Loaders Hd Krastorio");
        test("VehicleSnap", "Vehicle Snap");
        test("Warptorio2-Language-Pack", "Warptorio2 Language Pack");
        test("zk-lib", "Zk Lib");
    }
}
