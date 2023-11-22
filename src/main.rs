use std::collections::HashMap;

use eframe::{
    run_native,
    epi::{App, Frame},
    egui::{CentralPanel, CtxRef},
    NativeOptions
};

use m_calc::{calc::Token, calculate, tokenize, display_expr};

struct Calculator {
    context: HashMap<String, Vec<Token>>,
    input_text: String,
    history: Vec<Vec<Token>>,
}
impl Calculator {
    fn new() -> Calculator {
        Calculator {
            context: HashMap::new(),
            input_text: String::new(),
            history: vec![vec![]],
        }
    }
}
impl App for Calculator {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.input_text);
                if ui.button("This is a button").clicked() {
                    println!("Input text: `{}`", self.input_text);
                    let tokens = tokenize((self.input_text.clone() + " ").as_str()).unwrap();

                    self.history.push(tokens.clone());

                    let res = calculate(&tokens, &mut self.context).unwrap();
                    println!("Tokens: `{:?}` (`{}`)", tokens, display_expr(&tokens));
                    println!("Result: `{}` (`{}`)", res, display_expr(&vec![res.clone()]));
                }
            });

            ui.columns(2, |cols| {
                cols[0].heading("History");
                for expr in self.history.iter().rev() {
                    cols[0].monospace(display_expr(expr));
                }
                cols[1].heading("Assignments");
                for entry in &self.context {
                    cols[1].monospace(format!("{} = {}", entry.0, display_expr(entry.1)));
                }
            });
        });
    }

    fn name(&self) -> &str {
        "GUI Calculator - m_calc"
    }
}

fn main() {
    let app = Calculator::new();
    let win_option = NativeOptions::default();

    run_native(Box::new(app), win_option);
}
