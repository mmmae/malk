#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::Cursor;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use eframe::egui;
use egui::DragValue;
use egui::Slider;

fn main() -> Result<(), eframe::Error> {
    let mut savegame_path = String::from("No save game loaded");

    let mut savegame_data: Vec<u8> = Vec::new();

    let mut status = String::from("Ready");

    let mut timestamp_year = 0;      // 0x1 - 0x2
    let mut timestamp_month = 0;     // 0x4
    let mut timestamp_day = 0;       // 0x5
    let mut timestamp_hour = 0;      // 0x6
    let mut timestamp_minute = 0;    // 0x7
    let mut timestamp_second = 0;    // 0x8

    let mut count_gags = 0;          // 0x259
    let mut count_coins = 0;         // 0x1129 - 0x112b

    let mut last_level_played = 0;   // 0x1115
    let mut last_mission = 0;        // 0x1119
    let mut last_level_unlocked = 0; // 0x111d

    let mut cards_l1 = 0;            // 0x1c13
    let mut cards_l2 = 0;            // 0x1c14
    let mut cards_l3 = 0;            // 0x1c15
    let mut cards_l4 = 0;            // 0x1c16
    let mut cards_l5 = 0;            // 0x1c17
    let mut cards_l6 = 0;            // 0x1c18
    let mut cards_l7 = 0;            // 0x1c19
    
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(480.0, 640.0)),
        max_window_size: Some(egui::vec2(480.0, 640.0)),
        resizable: false,
        ..Default::default()
    };

    eframe::run_simple_native("malk", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Now with vitamin R!");
            
            ui.horizontal(|ui| {
                if ui.button("Import...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        savegame_path = path.display().to_string();

                        // read file contents into bytes vector, unwrapped
                        savegame_data = fs::read(&savegame_path).unwrap();

                        // basic validation check with size and byte offset
                        if savegame_data.len() > 8192 || savegame_data[0] != 186 {
                            // invalid, shadow and reassign new empty vector
                            let mut savegame_data: Vec<u8> = Vec::new();
                            status = String::from("Import failed: not a save file");
                        } else {
                            // assign select bytes to variables
                            let mut rdr = Cursor::new(vec![savegame_data[1], savegame_data[2]]);
                            timestamp_year = rdr.read_u16::<BigEndian>().unwrap();

                            timestamp_month = savegame_data[4];
                            timestamp_day = savegame_data[5];
                            timestamp_hour = savegame_data[6];
                            timestamp_minute = savegame_data[7];
                            timestamp_second = savegame_data[8];

                            count_gags = savegame_data[601];

                            last_level_played = savegame_data[4373] + 1;
                            last_mission = savegame_data[4377] + 1;
                            last_level_unlocked = savegame_data[4381] + 1;

                            let mut rdr = Cursor::new(vec![savegame_data[4393], savegame_data[4394], savegame_data[4395]]);
                            count_coins = rdr.read_u24::<LittleEndian>().unwrap();

                            cards_l1 = savegame_data[7187];
                            cards_l2 = savegame_data[7188];
                            cards_l3 = savegame_data[7189];
                            cards_l4 = savegame_data[7190];
                            cards_l5 = savegame_data[7191];
                            cards_l6 = savegame_data[7192];
                            cards_l7 = savegame_data[7193];

                            status = String::from("Imported save file");
                        }
                    }
                }
                if ui.button("Export...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        savegame_path = path.display().to_string();

                        savegame_data[601] = count_gags;

                        savegame_data[10] = last_level_played; // display value (load game)
                        savegame_data[11] = last_mission; // display value (load game)
                        savegame_data[4373] = last_level_played - 1;
                        savegame_data[4377] = last_mission - 1;
                        savegame_data[4381] = last_level_unlocked - 1;

                        let mut wtr = Vec::new();
                        wtr.write_u24::<LittleEndian>(count_coins).unwrap();

                        savegame_data[4393] = wtr[0];
                        savegame_data[4394] = wtr[1];
                        savegame_data[4395] = wtr[2];

                        savegame_data[7187] = cards_l1;
                        savegame_data[7188] = cards_l2;
                        savegame_data[7189] = cards_l3;
                        savegame_data[7190] = cards_l4;
                        savegame_data[7191] = cards_l5;
                        savegame_data[7192] = cards_l6;
                        savegame_data[7193] = cards_l7;

                        fs::write(&savegame_path, &savegame_data).unwrap();
                        
                        status = String::from("Exported save file");
                    }
                }

                ui.label(&status).on_hover_text(&savegame_path);
            });

            ui.add_space(10.0);

            // display timestamp of imported save file
            ui.vertical_centered(|ui| {
                ui.label("Timestamp").on_hover_text("The time and date at the moment of save creation.\nFormat: YYYY-MM-DD HH:MM:SS");
                ui.code(format!("{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}", 
                    &timestamp_year,
                    &timestamp_month,
                    &timestamp_day,
                    &timestamp_hour,
                    &timestamp_minute,
                    &timestamp_second,
                ));
            });

            ui.add_space(20.0);

            ui.columns(5, |ui| {
                ui[0].label("Gags").on_hover_text("The total number of gags discovered across all levels.\nMax value: 84");
                ui[1].add(DragValue::new(&mut count_gags).clamp_range(0..=84));
                ui[2].label("");
                ui[3].label("Coins").on_hover_text("The number of coins currently held by the player.\nMax value: 9999999");
                ui[4].add(DragValue::new(&mut count_coins).clamp_range(0..=9999999));
            });

            ui.add_space(20.0);

            ui.columns(3, |ui| {
                ui[0].label("Last level played").on_hover_text("The last level loaded by the player. Should not exceed the last level unlocked.\nValue range: 1-7");
                ui[1].label("Last mission selected").on_hover_text("The last mission selected on the last level played.\nValue range: 1-7 (1-8 for L1, as the first mission is the tutorial.)");
                ui[2].label("Last level unlocked").on_hover_text("The last level unlocked by the player.\nValue range: 1-7");
            });
            ui.columns(3, |ui| {
                ui[0].add(DragValue::new(&mut last_level_played).clamp_range(1..=7));
                ui[1].add(DragValue::new(&mut last_mission).clamp_range(1..=8));
                ui[2].add(DragValue::new(&mut last_level_unlocked).clamp_range(1..=7));
            });
            
            ui.add_space(20.0);
            ui.style_mut().spacing.slider_width = 380.0;

            ui.label("Collector cards");
            ui.label("Use the sliders below to adjust the collected cards per level. Digits to the side of the slider represent which cards are locked (0) or unlocked (1). Numbers at the top represent their order on the scrap book menu.");
            ui.hyperlink_to("View image guide", "https://github.com/mmmae/malk");
            ui.add_space(10.0);
            ui.label("1234567");

            ui.horizontal(|ui| {
                ui.code(format!("{:0>7b}", &cards_l1).chars().rev().collect::<String>());
                ui.add(Slider::new(&mut cards_l1, 0..=127).text("L1").show_value(false));
            });

            ui.horizontal(|ui| {
                ui.code(format!("{:0>7b}", &cards_l2).chars().rev().collect::<String>());
                ui.add(Slider::new(&mut cards_l2, 0..=127).text("L2").show_value(false));
            });

            ui.horizontal(|ui| {
                ui.code(format!("{:0>7b}", &cards_l3).chars().rev().collect::<String>());
                ui.add(Slider::new(&mut cards_l3, 0..=127).text("L3").show_value(false));
            });

            ui.horizontal(|ui| {
                ui.code(format!("{:0>7b}", &cards_l4).chars().rev().collect::<String>());
                ui.add(Slider::new(&mut cards_l4, 0..=127).text("L4").show_value(false));
            });

            ui.horizontal(|ui| {
                ui.code(format!("{:0>7b}", &cards_l5).chars().rev().collect::<String>());
                ui.add(Slider::new(&mut cards_l5, 0..=127).text("L5").show_value(false));
            });

            ui.horizontal(|ui| {
                ui.code(format!("{:0>7b}", &cards_l6).chars().rev().collect::<String>());
                ui.add(Slider::new(&mut cards_l6, 0..=127).text("L6").show_value(false));
            });

            ui.horizontal(|ui| {
                ui.code(format!("{:0>7b}", &cards_l7).chars().rev().collect::<String>());
                ui.add(Slider::new(&mut cards_l7, 0..=127).text("L7").show_value(false));
            });

            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.hyperlink_to("mmmae/malk", "https://github.com/mmmae/malk");
            });
        });
    })
}
