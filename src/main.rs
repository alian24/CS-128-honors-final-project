use eframe::egui;
use vigenere_poke::VigenereCipher;

// Holds all GUI states across frames
struct VigenereApp {
    message: String,
    pokemon_key: String,
    output: String,
    key_valid: Option<bool>, // None = empty, Some(true/false) = validation result
}

// Initial Blank App 
impl Default for VigenereApp {
    fn default() -> Self {
        Self {
            message: String::new(),
            pokemon_key: String::new(),
            output: String::new(),
            key_valid: None,
        }
    }
}

impl eframe::App for VigenereApp {
    // Calls every frame to build the UI
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Heading
            ui.heading("PokéVigenère Cipher");
            ui.separator();

            // Message input
            ui.label("Message:");
            ui.add(egui::TextEdit::singleline(&mut self.message).desired_width(f32::INFINITY));

            ui.add_space(8.0);

            // Pokemon key input with live validation
            ui.label("Pokémon Key:");
            let key_response = ui.add(egui::TextEdit::singleline(&mut self.pokemon_key).desired_width(f32::INFINITY));
            if key_response.changed() {
                self.key_valid = if self.pokemon_key.is_empty() {
                    None
                } else {
                    Some(VigenereCipher::is_valid_pokemon(&self.pokemon_key))
                };
            }

            match self.key_valid {
                Some(true)  => { ui.colored_label(egui::Color32::GREEN, "Valid Pokémon"); }
                Some(false) => { ui.colored_label(egui::Color32::RED,   "Not a valid Pokémon"); }
                None        => { ui.label("Enter a Pokémon name as the key"); }
            }

            ui.add_space(8.0);

            // Encrypt / Decrypt buttons (disabled until a valid Pokemon key is entered)
            ui.horizontal(|ui| {
                ui.add_enabled_ui(self.key_valid == Some(true), |ui| {
                    if ui.button("Encrypt").clicked() {
                        let cipher = VigenereCipher::new(&self.pokemon_key).unwrap();
                        self.output = cipher.vigenere_encrypt(&self.message);
                    }
                    if ui.button("Decrypt").clicked() {
                        let cipher = VigenereCipher::new(&self.pokemon_key).unwrap();
                        self.output = cipher.vigenere_decipher(&self.message);
                    }
                });
            });

            // Output display (read-only but text is copyable)
            ui.add_space(8.0);
            ui.separator();
            ui.label("Output:");
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.add(egui::Label::new(&self.output).selectable(true));
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    // Set UI Window Size
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 400.0])
            .with_min_inner_size([250.0, 250.0])
            .with_resizable(true),
        ..Default::default()
    };
    // Launches App with Blank States
    eframe::run_native(
        "PokéVigenère",
        options,
        Box::new(|_cc| Ok(Box::new(VigenereApp::default()))),
    )
}