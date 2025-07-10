mod modules;
use eframe::{App, NativeOptions, run_native};
use egui::{CentralPanel, SidePanel, Window, menu};
use modules::ProductMte;
use rust_decimal::prelude::*;

#[derive(Default)]
struct MteToolBox {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
}

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
                if ui.button("Utilisateurs").clicked() {
                    todo!()
                }
            });
        //
        //CENTRAL PANEL
        //--- MUST BE THE LAST ONE ! ---
        //here wilL be displayed all the widgets tools thats selected in the side menu in a window
        CentralPanel::default().show(ctx, |ui| {
            // VERTICAL CENTERED
            ui.vertical_centered(|ui| {
                ui.heading("Centered Header");
            });
            //
            ui.label(format!("{product_mte}"));
            Window::new("Stock")
                .resizable(true)
                .collapsible(true)
                .show(ctx, |ui| ui.spinner());
            //
            ui.label("Drag-and-drop files onto the window!");

            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            // Show dropped files (if any):
            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Dropped files:");

                    for file in &self.dropped_files {
                        let mut info = if let Some(path) = &file.path {
                            path.display().to_string()
                        } else if !file.name.is_empty() {
                            file.name.clone()
                        } else {
                            "???".to_owned()
                        };

                        let mut additional_info = vec![];
                        if !file.mime.is_empty() {
                            additional_info.push(format!("type: {}", file.mime));
                        }
                        if let Some(bytes) = &file.bytes {
                            additional_info.push(format!("{} bytes", bytes.len()));
                        }
                        if !additional_info.is_empty() {
                            info += &format!(" ({})", additional_info.join(", "));
                        }

                        ui.label(info);
                    }
                });
            }
        });

        preview_files_being_dropped(ctx);

        // Collect dropped files:
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.dropped_files.clone_from(&i.raw.dropped_files);
            }
        });
    }
}

/// Preview hovering files:
fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::{Align2, Color32, Id, LayerId, Order, TextStyle};
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
        //
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
