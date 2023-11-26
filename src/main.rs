// TODO: Add `Clear History` and `Clear Assignments` buttons
use std::collections::HashMap;

use eframe::{
    run_native,
    App,
    Frame,
    NativeOptions,
    egui::{
        CentralPanel,
        Context,
        Key,
        Layout,
        Align,
        Direction,
        ScrollArea, TextEdit, vec2,
    },
};

use m_calc::{
    calculate,
    tokenize,
    display_expr,
    Token,
    CalcErr,
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
            history: vec![vec![]],
        }
    }
}
impl App for Calculator {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
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

            ui.add_space(5.0);
            ui.group(|ui| { // Output
                ui.label(&self.output_text);
                ui.set_width(ui.available_width());
            });

            ui.add_space(5.0);
            ui.columns(2, |cols| {
                cols[0].group(|ui| {
                    ui.heading("History");
                    ScrollArea::vertical().id_source("l_scroll").show(ui, |ui| {
                        for expr in self.history.iter().rev() {
                            ui.monospace(display_expr(expr));
                        }
                    });
                    ui.set_height(ui.available_height());
                });
                cols[0].set_min_height(cols[0].available_height());
                cols[1].group(|ui| {
                    ui.heading("Assignments");
                    ScrollArea::vertical().id_source("r_scroll").show(ui, |ui| {
                        let mut entries: Vec<String> = Vec::new();
                        for entry in &self.context {
                            entries.push(format!("{} = {}", entry.0, display_expr(entry.1)));
                        }
                        entries.sort();
                        for entry in entries {
                            ui.monospace(entry);
                        }
                    });
                    ui.set_height(ui.available_height());
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
