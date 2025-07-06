mod modules;
use eframe::{App, NativeOptions, run_native};
use egui::{CentralPanel, SidePanel, Window, menu};
use modules::ProductMte;
use rust_decimal::prelude::*;

#[derive(Default)]
struct MteToolBox;

impl MteToolBox {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl App for MteToolBox {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //
        //TEST TO SHOWCASE WITH BUTTON
        let product_mte = ProductMte {
            id: 250659,
            manufacturer: "String".to_string(),
            name: "String".to_string(),
            price: Decimal::new(123, 2),
            quantity: 26,
            in_stock: true,
        };
        //MENU BAR FOR ADVANCED OPTIONS
        egui::TopBottomPanel::top("top_panel0")
            .resizable(false)
            .show(ctx, |ui| {
                menu::bar(ui, |ui| {
                    /**/
                    ui.menu_button("File", |ui| {
                        if ui.button("Save").clicked() {
                            todo!()
                        }
                        if ui.button("Quit").clicked() {
                            todo!()
                        }
                    });
                    /**/
                    ui.menu_button("Edit", |ui| {
                        if ui.button("Cut").clicked() {
                            todo!()
                        }
                        if ui.button("Copy").clicked() {
                            todo!()
                        }
                        if ui.button("Paste").clicked() {
                            todo!()
                        }
                    });
                    /**/
                    ui.menu_button("Database", |ui| {
                        if ui.button("Connect").clicked() {
                            todo!()
                        }
                        if ui.button("Manage").clicked() {
                            todo!()
                        }
                        if ui.button("History").clicked() {
                            todo!()
                        }
                    });
                    /**/
                });
            });
        //
        //
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Bottom Panel");
            });
        });
        //
        //SIDE MENU FOR ACCESS DIFFERENT TOOLS WIDGETS
        SidePanel::left("side_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Side Menu");
                });
                /*
                ADD ALL THE BUTTONS FOR ACCESS ALL THE TOOLS WIDGETS HERE
                */
                if ui.button("Transmetteur Vidéo").clicked() {
                    todo!()
                }
                if ui.button("Cartes SIM").clicked() {
                    todo!()
                }
                if ui.button("Stock").clicked() {
                    todo!()
                }
                if ui.button("Magasin").clicked() {
                    todo!()
                }
                if ui.button("SAV").clicked() {
                    todo!()
                }
            });
        //
        //CENTRAL PANEL
        //--- MUST BE THE LAST ONE ! ---
        //here wilL be displayed all widgets tools thats selected in the side menu in a window
        CentralPanel::default().show(ctx, |ui| {
            // VERTICAL CENTERED
            ui.vertical_centered(|ui| {
                ui.heading("Centered Header");
            });
            //
            Window::new("Transmetteur Vidéo").show(ctx, |ui| {
                ui.label(format!("{product_mte}"));
                if ui.button("export JSON").clicked() {
                    println!("{product_mte}")
                }
            });
            //
        });
        //
    }
}

fn main() {
    //APP LAUCHING IS HERE
    let win_option = NativeOptions::default(); //DEFINE OPTIONS FOR THE APP
    run_native(
        "MteToolBox",
        win_option,
        Box::new(|cc| Ok(Box::new(MteToolBox::new(cc)))),
    )
    .unwrap(); //NEED TO MANAGE THIS ERROR BETTER
}
