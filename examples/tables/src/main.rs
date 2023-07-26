use eframe::egui;

// Not exactly an example, but this was the most convenient in-repo place for a test case

fn main() -> eframe::Result<()> {
    let frame_options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(400.0, 400.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        "Tables example",
        frame_options,
        Box::new(|cc| Box::<TableExample>::new(TableExample::new(cc))),
    )
}

#[derive(Default)]
struct TableExample {}

impl TableExample {
    fn new(cc: &eframe::CreationContext<'_>) -> TableExample {
        cc.egui_ctx.set_visuals(egui::Visuals::light());
        TableExample::default()
    }
}

impl eframe::App for TableExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Three by three, initial col width 100");
            ui.push_id("3x3", |ui| {
                egui_extras::TableBuilder::new(ui)
                    .columns(egui_extras::Column::auto_with_initial_suggestion(100.0), 3)
                    .striped(true)
                    .body(three_by_three)
            });
            ui.heading("Four by four, initial col width 100");
            ui.push_id("4x4-100", |ui| {
                egui_extras::TableBuilder::new(ui)
                    .columns(egui_extras::Column::auto_with_initial_suggestion(100.0), 4)
                    .striped(true)
                    .body(four_by_four)
            });
            ui.heading("Four by four, initial col width 200");
            ui.push_id("4x4-200", |ui| {
                egui_extras::TableBuilder::new(ui)
                    .columns(egui_extras::Column::auto_with_initial_suggestion(200.0), 4)
                    .striped(true)
                    .body(four_by_four)
            });
        });
    }
}

fn three_by_three(mut body: egui_extras::TableBody) {
    body.row(15.0, |mut row| {
        row.col(|ui| {
            ui.label("bit");
        });
        row.col(|ui| {
            ui.label("byte");
        });
        row.col(|ui| {
            ui.label("barbecue");
        });
    });
    body.row(15.0, |mut row| {
        row.col(|ui| {
            ui.label("byte");
        });
        row.col(|ui| {
            ui.label("barbecue");
        });
        row.col(|ui| {
            ui.label("bit");
        });
    });
    body.row(15.0, |mut row| {
        row.col(|ui| {
            ui.label("barbecue");
        });
        row.col(|ui| {
            ui.label("bit");
        });
        row.col(|ui| {
            ui.label("byte");
        });
    });
}

fn four_by_four(mut body: egui_extras::TableBody) {
    body.row(15.0, |mut row| {
        row.col(|ui| {
            ui.label("bit");
        });
        row.col(|ui| {
            ui.label("byte");
        });
        row.col(|ui| {
            ui.label("barbecue");
        });
        row.col(|ui| {
            ui.label("sesquipedalian");
        });
    });
    body.row(15.0, |mut row| {
        row.col(|ui| {
            ui.label("byte");
        });
        row.col(|ui| {
            ui.label("barbecue");
        });
        row.col(|ui| {
            ui.label("sesquipedalian");
        });
        row.col(|ui| {
            ui.label("bit");
        });
    });
    body.row(15.0, |mut row| {
        row.col(|ui| {
            ui.label("barbecue");
        });
        row.col(|ui| {
            ui.label("sesquipedalian");
        });
        row.col(|ui| {
            ui.label("bit");
        });
        row.col(|ui| {
            ui.label("byte");
        });
    });
    body.row(15.0, |mut row| {
        row.col(|ui| {
            ui.label("sesquipedalian");
        });
        row.col(|ui| {
            ui.label("bit");
        });
        row.col(|ui| {
            ui.label("byte");
        });
        row.col(|ui| {
            ui.label("barbecue");
        });
    });
}
