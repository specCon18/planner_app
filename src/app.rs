use egui::Color32;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
                ui.separator();
                ui.vertical_centered(|ui| {
                    ui.heading("Dump List");
                });
                ui.separator();
                let mut checked: bool = false;        
                ui.vertical(|ui| {
                    ui.checkbox(&mut checked, "Task");
                    ui.checkbox(&mut checked, "Task");
                    ui.checkbox(&mut checked, "Task");
                });
                ui.separator();
                ui.vertical_centered(|ui| {
                    ui.heading("Daily Kanban");
                });
                ui.separator();
                ui.vertical(|ui| {
                    // Create a horizontal layout for Kanban columns
                    ui.horizontal(|ui| {
                    // "Staged" column
                    ui.vertical(|ui| {
                        ui.vertical(|ui|{
                            ui.colored_label(Color32::from_rgb(0x0E,0xA5,0xE9), "Staged");
                        });
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut checked, "Do thing with the stuff 1");
                            ui.colored_label(Color32::from_rgb(0x22,0xC5,0x5E), "🕑")
                        });
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut checked, "Do thing with the stuff 2");
                            ui.colored_label(Color32::from_rgb(0x22,0xC5,0x5E), "🕑")                            
                        });
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut checked, "Do thing with the stuff 3");
                            ui.colored_label(Color32::from_rgb(0x22,0xC5,0x5E), "🕑")                            
                        });
                    });
        
                    // "In Progress" column
                    ui.vertical(|ui| {
                        ui.colored_label(Color32::from_rgb(0xEA,0xB3,0x08), "In Progress");
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut checked, "Do thing with the stuff 4");
                            ui.colored_label(Color32::from_rgb(0xF9,0x73,0x16), "🕘")                            
                        });
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut checked, "Do thing with the stuff 5");
                            ui.colored_label(Color32::from_rgb(0xEF,0x44,0x44), "🕚")                            
                        });
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut checked, "Do thing with the stuff 6");
                            ui.colored_label(Color32::from_rgb(0xEA,0xB3,0x08), "🕕")                            
                        });
                    });
        
                    // "Done" column
                    ui.vertical(|ui| {
                        ui.colored_label(Color32::from_rgb(0x22,0xC5,0x5E), "Done");
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut checked, "Do thing with the stuff 7");
                            ui.colored_label(Color32::from_rgb(0xEF,0x44,0x44), "🕚")                            
                        });
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut checked, "Do thing with the stuff 8");
                            ui.colored_label(Color32::from_rgb(0xEF,0x44,0x44), "🕚")                            
                        });
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut checked, "Do thing with the stuff 9");
                            ui.colored_label(Color32::from_rgb(0xEF,0x44,0x44), "🕚")                            
                        });
                    });
                });
                });
                ui.separator();
                ui.vertical_centered(|ui| {
                    ui.heading("Monthly Calendar");
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.horizontal(|ui|{
                        ui.separator();
                        ui.label("Month");
                        ui.separator();
                    });
                    ui.horizontal(|ui| {
                        //Monday
                        ui.vertical(|ui| {
                            ui.horizontal(|ui|{
                                ui.colored_label(Color32::from_rgb(0xEC,0x48,0x99),"MON");
                            });
                            ui.horizontal(|ui|{
                                ui.label("01");
                            });
                            ui.horizontal(|ui|{
                                ui.label("08");
                            });
                            ui.horizontal(|ui|{
                                ui.label("15");
                            });
                            ui.horizontal(|ui|{
                                ui.label("22");
                            });
                            ui.horizontal(|ui|{
                                ui.label("29");
                            });
                        });
                        //Tuesday
                        ui.vertical(|ui| {
                            ui.horizontal(|ui|{
                                ui.colored_label(Color32::from_rgb(0xD9,0x46,0xEF),"TUE");
                            });
                            ui.horizontal(|ui|{
                                ui.label("02");
                            });
                            ui.horizontal(|ui|{
                                ui.label("09");
                            });
                            ui.horizontal(|ui|{
                                ui.label("16");
                            });
                            ui.horizontal(|ui|{
                                ui.label("23");
                            });
                            ui.horizontal(|ui|{
                                ui.label("30");
                            });
                        });
                        //Wednesday
                        ui.vertical(|ui| {
                            ui.horizontal(|ui|{
                                ui.colored_label(Color32::from_rgb(0xA8,0x55,0xF7),"WED");
                            });
                            ui.horizontal(|ui|{
                                ui.label("03");
                            });
                            ui.horizontal(|ui|{
                                ui.label("10");
                            });
                            ui.horizontal(|ui|{
                                ui.label("17");
                            });
                            ui.horizontal(|ui|{
                                ui.label("24");
                            });
                            ui.horizontal(|ui|{
                                ui.label("31");
                            });
                        });
                        //Thursday
                        ui.vertical(|ui| {
                            ui.horizontal(|ui|{
                                ui.colored_label(Color32::from_rgb(0x8B,0x5C,0xF6),"THU");
                            });
                            ui.horizontal(|ui|{
                                ui.label("04");
                            });
                            ui.horizontal(|ui|{
                                ui.label("11");
                            });
                            ui.horizontal(|ui|{
                                ui.label("18");
                            });
                            ui.horizontal(|ui|{
                                ui.label("25");
                            });
                            ui.horizontal(|_ui|{
                            });
                        });
                        //Friday
                        ui.vertical(|ui| {
                            ui.horizontal(|ui|{
                                ui.colored_label(Color32::from_rgb(0x63,0x66,0xF1),"FRI");
                            });
                            ui.horizontal(|ui|{
                                ui.label("05");
                            });
                            ui.horizontal(|ui|{
                                ui.label("12");
                            });
                            ui.horizontal(|ui|{
                                ui.label("19");
                            });
                            ui.horizontal(|ui|{
                                ui.label("26");
                            });
                            ui.horizontal(|_ui|{
                            });
                        });
                        //Saterday
                        ui.vertical(|ui| {
                            ui.horizontal(|ui|{
                                ui.colored_label(Color32::from_rgb(0x3B,0x82,0xF6),"SAT");
                            });
                            ui.horizontal(|ui|{
                                ui.label("06");
                            });
                            ui.horizontal(|ui|{
                                ui.label("13");
                            });
                            ui.horizontal(|ui|{
                                ui.label("20");
                            });
                            ui.horizontal(|ui|{
                                ui.label("27");
                            });
                            ui.horizontal(|_ui|{
                            });
                        });
                        //Sunday
                        ui.vertical(|ui| {
                            ui.horizontal(|ui|{
                                ui.colored_label(Color32::from_rgb(0x0E,0xA5,0xE9),"SUN");
                            });
                            ui.horizontal(|ui|{
                                ui.label("07");
                            });
                            ui.horizontal(|ui|{
                                ui.label("14");
                            });
                            ui.horizontal(|ui|{
                                ui.label("21");
                            });
                            ui.horizontal(|ui|{
                                ui.label("28");
                            });
                            ui.horizontal(|_ui|{
                            });
                        });
                    });
                });
            });       
    }
}