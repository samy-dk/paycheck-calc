#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;


fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Paycheck Calculator ",
        options,
        Box::new(|_cc| Box::new(Paycheck::new())),
    );
}


struct Paycheck {
    wage: f32,
    ot_wage: f32,
    tax: f32,
    paycheck: f32,
    deductions: f32,
    r_hours: f32,
    ot_hours: f32,
}

impl Paycheck {
    fn new() -> Self {
        Self {
            wage : 0.0,
            ot_wage: 0.0,
            tax : 0.0,
            paycheck : 0.0,
            deductions : 0.0,
            r_hours: 0.0,
            ot_hours: 0.0,
        }
    }


    fn calc_paycheck(&mut self) {
        self.paycheck = self.r_hours * self.wage;
        self.paycheck += self.ot_hours * self.ot_wage;
        self.paycheck *= 1.0 - self.tax; 
        self.paycheck -= self.deductions;
    }
}

impl eframe::App for Paycheck {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(2.0);


        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Paycheck Calculator");
            ui.add(egui::Slider::new(&mut self.wage, 0.0..=100.0).text("Wage"));
            self.ot_wage = self.wage * 1.5;
            ui.add(egui::Slider::new(&mut self.r_hours, 0.0..=80.0).text("Regular Hours"));
            ui.add(egui::Slider::new(&mut self.ot_hours, 0.0..=40.0).text("Overtime Hours"));
            ui.add(egui::Slider::new(&mut self.tax, 0.0..=1.0).text("Tax Rate"));
            ui.add(egui::Slider::new(&mut self.deductions, 0.0..=500.0).text("Deductions"));
            ui.horizontal(|ui| {
                if ui.button("Calculate Paycheck").clicked() {
                    self.calc_paycheck();
                }
                ui.label("Paycheck:");
                ui.label(format!("${:.2}", self.paycheck));
            });
        });
    }
}
