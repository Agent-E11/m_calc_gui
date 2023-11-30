use std::collections::HashMap;

use eframe::{
    run_native,
    NativeOptions,
    Frame,
    App,
};
use eframe::egui::{
    CentralPanel,
    ScrollArea,
    TextEdit,
    Context,
    Layout,
    Align,
    Key,
};

use m_calc::{
    display_expr,
    calculate,
    tokenize,
};
use m_calc::{
    CalcErr,
    Token,
};

struct Calculator {
    context: HashMap<String, Vec<Token>>,
    input_text: String,
    output_text: String,
    history: Vec<Vec<Token>>,
}
impl Calculator {
    fn new() -> Calculator {
        Calculator {
            context: HashMap::new(),
            input_text: String::new(),
            output_text: String::new(),
            history: vec![],
        }
    }
}
impl App for Calculator {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {

                let button = ui.button("Calculate");
                let input = ui.add_sized((ui.available_width(), 0.0), TextEdit::singleline(&mut self.input_text));
                if button.clicked() || ui.input(|i| i.key_pressed(Key::Enter)) {
                    // Check if the button was clicked _or_ the `Enter` was pressed
                    println!("Input text: `{}`", self.input_text);

                    match tokenize((self.input_text.clone()).as_str()) {
                        Ok(tokens) => {
                            self.history.push(tokens.clone());

                            match calculate(&tokens, &mut self.context) {
                                Ok(res) => {
                                    self.output_text = res.display(); // Output result
                                },
                                Err(CalcErr(err_str)) => self.output_text = err_str, // Output error
                            }
                        },
                        Err(CalcErr(err_str)) => {
                            self.output_text = err_str;
                        },
                    }

                    input.request_focus(); // Re-focus after (`Enter` un-focuses)
                }
            });

            // Output section
            ui.add_space(5.0);
            ui.group(|ui| {
                ui.label(&self.output_text);
                ui.set_width(ui.available_width());
            });

            // `History` and `Assignments` columns
            ui.add_space(5.0);
            ui.columns(2, |cols| {
                cols[0].group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.heading("History");
                        if ui.button("Clear").clicked() {
                            self.history.clear();
                        }
                    });
                    ScrollArea::vertical().id_source("l_scroll").show(ui, |ui| {
                        for expr in self.history.clone().iter().enumerate().rev() {
                            ui.horizontal(|ui| {
                                if ui.small_button("X").clicked() {
                                    self.history.remove(expr.0);
                                }
                                ui.monospace(display_expr(expr.1));
                            });
                        }
                    });
                    ui.set_min_size(ui.available_size());
                });
                cols[1].group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.heading("Assignments");
                        if ui.button("Clear").clicked() {
                            self.context.clear();
                        }
                    });
                    ScrollArea::vertical().id_source("r_scroll").show(ui, |ui| {
                        let mut entries: Vec<(String, String)> = Vec::new();
                        for entry in &self.context {
                            entries.push((entry.0.clone(), format!("{} = {}", entry.0, display_expr(entry.1))));
                        }
                        entries.sort();
                        for entry in entries {
                            ui.horizontal(|ui| {
                                if ui.button("X").clicked() {
                                    self.context.remove(&entry.0);
                                };
                                ui.monospace(entry.1);
                            });
                        }
                    });
                    ui.set_min_size(ui.available_size());
                });
            });
        });
    }
}
fn main() -> eframe::Result<()> {
    let app = Calculator::new();
    let win_option = NativeOptions::default();

    run_native(
        "GUI Calculator - m_calc",
        win_option,
        Box::new(|_cc| Box::new(app)),
    )
}
