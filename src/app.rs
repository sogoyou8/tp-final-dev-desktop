use eframe::egui;
use eframe::egui::{Color32, Stroke, Margin, Frame, RichText, Vec2, FontId};
use crate::model::{Note, AppState, Backend};
use crate::dao::{Dao, JsonDao, SqliteDao};
use crate::api::lancer_fetch_notes;
use std::sync::mpsc;
use uuid::Uuid;
use chrono::Utc;

pub struct NotesApp {
    state: AppState,
    dao: Box<dyn Dao>,
    
    note_selectionnee: Option<Uuid>,
    edit_titre: String,
    edit_contenu: String,
    edit_tags: String,
    
    rx_fetch: Option<mpsc::Receiver<Result<Vec<Note>, String>>>,
    erreur_api: Option<String>,
    statut_message: String,
    
    show_stats: bool,
}

impl NotesApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut visuals = egui::Visuals::dark();
        
        let neon_blue = Color32::from_rgb(0, 242, 255);
        let deep_space = Color32::from_rgb(5, 7, 10);
        let nebula_purple = Color32::from_rgb(112, 0, 255);

        visuals.widgets.active.bg_fill = neon_blue;
        visuals.widgets.hovered.bg_stroke = Stroke::new(2.0, neon_blue);
        visuals.window_corner_radius = 16.0.into();
        visuals.widgets.noninteractive.bg_fill = deep_space;
        visuals.panel_fill = deep_space;
        visuals.extreme_bg_color = Color32::from_rgb(15, 18, 25);
        visuals.selection.bg_fill = nebula_purple.linear_multiply(0.3);
        
        cc.egui_ctx.set_visuals(visuals);

        let dao = Box::new(JsonDao::new("notes.json"));
        let mut app = Self {
            state: AppState::default(),
            dao,
            note_selectionnee: None,
            edit_titre: String::new(),
            edit_contenu: String::new(),
            edit_tags: String::new(),
            rx_fetch: None,
            erreur_api: None,
            statut_message: "SYSTÈME PRÊT".to_string(),
            show_stats: false,
        };
        
        app.recharger_notes();
        app
    }

    fn recharger_notes(&mut self) {
        match self.dao.lire_tout() {
            Ok(notes) => {
                self.state.notes = notes;
                self.statut_message = format!("BASE SYNCHRO : {} NOTES", self.state.notes.len());
            }
            Err(e) => self.erreur_api = Some(format!("ERREUR SYNCHRO : {}", e)),
        }
    }

    fn changer_backend(&mut self, nouveau_backend: Backend) {
        if self.state.backend_actuel == nouveau_backend { return; }
        match nouveau_backend {
            Backend::Json => self.dao = Box::new(JsonDao::new("notes.json")),
            Backend::Sqlite => {
                if let Ok(d) = SqliteDao::new("notes.db") { self.dao = Box::new(d); }
            }
        }
        self.state.backend_actuel = nouveau_backend;
        self.recharger_notes();
    }

    fn sauvegarder_edition(&mut self) {
        if self.edit_titre.is_empty() { return; }
        let tags: Vec<String> = self.edit_tags.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        if let Some(id) = self.note_selectionnee {
            if let Some(note) = self.state.notes.iter_mut().find(|n| n.id == id) {
                note.titre = self.edit_titre.clone();
                note.contenu = self.edit_contenu.clone();
                note.tags = tags;
                note.date_modification = Utc::now();
                let _ = self.dao.mettre_a_jour(note);
            }
        } else {
            let nouvelle_note = Note::new(self.edit_titre.clone(), self.edit_contenu.clone(), tags);
            let _ = self.dao.sauvegarder(&nouvelle_note);
            self.state.notes.push(nouvelle_note);
        }
        self.statut_message = "ENREGISTREMENT TERMINÉ".to_string();
    }

    fn supprimer_note(&mut self, id: Uuid) {
        if let Ok(_) = self.dao.supprimer(id) {
            self.state.notes.retain(|n| n.id != id);
            if self.note_selectionnee == Some(id) {
                self.note_selectionnee = None;
                self.edit_titre.clear();
                self.edit_contenu.clear();
                self.edit_tags.clear();
            }
            self.statut_message = "NOTE SUPPRIMÉE".to_string();
        }
    }

    fn importer_notes(&mut self) {
        let (tx, rx) = mpsc::channel();
        self.rx_fetch = Some(rx);
        self.statut_message = "CONNEXION SERVEUR EXTERNE...".to_string();
        lancer_fetch_notes(tx);
    }

    fn exporter_tout(&mut self) {
        let now = Utc::now().format("%Y%m%d_%H%M%S");
        let nom_fichier = format!("export_{}.json", now);
        if let Ok(json) = serde_json::to_string_pretty(&self.state.notes) {
            if let Ok(_) = std::fs::write(&nom_fichier, json) {
                self.statut_message = format!("SAUVEGARDE CRÉÉE : {}", nom_fichier);
            }
        }
    }
}

impl eframe::App for NotesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::N)) { self.note_selectionnee = None; self.edit_titre.clear(); self.edit_contenu.clear(); self.edit_tags.clear(); }
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::S)) { self.sauvegarder_edition(); }

        if let Some(rx) = &self.rx_fetch {
            if let Ok(resultat) = rx.try_recv() {
                if let Ok(notes) = resultat {
                    for n in notes { let _ = self.dao.sauvegarder(&n); self.state.notes.push(n); }
                    self.statut_message = "IMPORTATION TERMINÉE".to_string();
                }
                self.rx_fetch = None;
            }
        }

        let neon_blue = Color32::from_rgb(0, 242, 255);
        let nebula_purple = Color32::from_rgb(112, 0, 255);
        let error_red = Color32::from_rgb(255, 60, 60);
        let dark_void = Color32::from_rgb(10, 12, 18);

        // --- TOP PANEL ---
        egui::TopBottomPanel::top("top_panel").frame(Frame::NONE.fill(dark_void).inner_margin(Margin::same(15))).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("⚡ COSMIC_NOTES").strong().color(neon_blue).size(22.0).italics());
                ui.add_space(40.0);
                
                ui.add(egui::TextEdit::singleline(&mut self.state.recherche)
                    .hint_text("RECHERCHER DANS L'UNIVERS...")
                    .margin(Margin::symmetric(20, 10))
                    .font(FontId::proportional(15.0)));
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add(egui::Button::new("📊 STATS").corner_radius(4.0)).clicked() { self.show_stats = !self.show_stats; }
                    ui.add_space(10.0);
                    if ui.add(egui::Button::new("📤 EXPORTER").corner_radius(4.0)).clicked() { self.exporter_tout(); }
                    ui.add_space(10.0);
                    if ui.add(egui::Button::new("📥 IMPORTER").corner_radius(4.0)).clicked() { self.importer_notes(); }
                    
                    ui.separator();
                    ui.label(RichText::new("MOTEUR:").small().color(nebula_purple));
                    let current_backend = self.state.backend_actuel;
                    if ui.selectable_label(current_backend == Backend::Json, "JSON").clicked() { self.changer_backend(Backend::Json); }
                    if ui.selectable_label(current_backend == Backend::Sqlite, "SQLITE").clicked() { self.changer_backend(Backend::Sqlite); }
                });
            });
        });

        // --- STATUS BAR ---
        egui::TopBottomPanel::bottom("status_bar").frame(Frame::NONE.fill(Color32::BLACK).inner_margin(Margin::symmetric(15, 8))).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("STATUT : EN LIGNE").small().color(neon_blue));
                ui.add_space(20.0);
                ui.label(RichText::new(&self.statut_message).small().color(Color32::GRAY));
                
                let mut clear_error = false;
                if let Some(err) = &self.erreur_api {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.colored_label(error_red, format!("⚠ ERREUR : {}", err));
                        if ui.button("EFFACER").clicked() { clear_error = true; }
                    });
                }
                if clear_error { self.erreur_api = None; }
            });
        });

        // --- SIDE PANEL ---
        egui::SidePanel::left("side_panel").frame(Frame::NONE.fill(dark_void.linear_multiply(1.2)).inner_margin(Margin::same(20))).resizable(true).show(ctx, |ui| {
            ui.vertical(|ui| {
                if ui.add(egui::Button::new(RichText::new("+ NOUVELLE NOTE").strong().color(Color32::BLACK))
                    .fill(neon_blue)
                    .min_size(Vec2::new(ui.available_width(), 50.0))
                    .corner_radius(8.0)).clicked() {
                    self.note_selectionnee = None; self.edit_titre.clear(); self.edit_contenu.clear(); self.edit_tags.clear();
                }
                
                ui.add_space(25.0);
                ui.label(RichText::new("FILTRES NÉBULEUSE").small().strong().color(nebula_purple));
                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    if ui.selectable_label(self.state.tag_filtre.is_none(), "TOUT").clicked() { self.state.tag_filtre = None; }
                    for tag in self.state.tags_uniques() {
                        if ui.selectable_label(self.state.tag_filtre == Some(tag.clone()), tag.to_uppercase()).clicked() {
                            self.state.tag_filtre = Some(tag);
                        }
                    }
                });
                
                ui.add_space(20.0);
                ui.separator();
                ui.add_space(20.0);

                let mut note_a_supprimer = None;
                egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                    for note in self.state.notes_filtrees() {
                        let is_selected = self.note_selectionnee == Some(note.id);
                        
                        let card_frame = if is_selected {
                            Frame::NONE.fill(nebula_purple.linear_multiply(0.2)).stroke(Stroke::new(2.0, nebula_purple)).corner_radius(10.0).inner_margin(15)
                        } else {
                            Frame::NONE.fill(Color32::from_rgb(20, 22, 30)).corner_radius(10.0).inner_margin(15)
                        };

                        let response = card_frame.show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.vertical(|ui| {
                                ui.label(RichText::new(&note.titre).strong().size(15.0).color(if is_selected { neon_blue } else { Color32::WHITE }));
                                ui.add_space(6.0);
                                ui.label(RichText::new(note.date_modification.format("%d.%m.%Y | %H:%M").to_string()).small().color(Color32::DARK_GRAY));
                            });
                        }).response;

                        let response = ui.interact(response.rect, response.id, egui::Sense::click());
                        if response.clicked() {
                            self.note_selectionnee = Some(note.id);
                            self.edit_titre = note.titre.clone();
                            self.edit_contenu = note.contenu.clone();
                            self.edit_tags = note.tags.join(", ");
                        }

                        response.context_menu(|ui| {
                            if ui.button("💥 SUPPRIMER DÉFINITIVEMENT").clicked() { note_a_supprimer = Some(note.id); ui.close_menu(); }
                        });
                        ui.add_space(12.0);
                    }
                });

                if let Some(id) = note_a_supprimer { self.supprimer_note(id); }
            });
        });

        // --- CENTRAL PANEL (EDITOR) ---
        egui::CentralPanel::default().frame(Frame::NONE.fill(Color32::from_rgb(5, 7, 10)).inner_margin(Margin::same(40))).show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.edit_titre)
                    .hint_text("TITRE DE LA NOTE...")
                    .font(FontId::proportional(36.0))
                    .text_color(neon_blue)
                    .frame(false));
                
                ui.add_space(8.0);
                ui.add(egui::TextEdit::singleline(&mut self.edit_tags)
                    .hint_text("tags: cosmos, nebula, important")
                    .font(FontId::proportional(16.0))
                    .text_color(nebula_purple)
                    .frame(false));
                
                ui.add_space(30.0);
                
                // HUD Editor Border
                Frame::NONE.fill(Color32::from_rgb(15, 18, 25)).corner_radius(12.0).inner_margin(20).show(ui, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_sized([ui.available_width(), ui.available_height() - 100.0], egui::TextEdit::multiline(&mut self.edit_contenu)
                            .hint_text("ÉCRIVEZ VOTRE CONTENU ICI...")
                            .frame(false)
                            .font(FontId::monospace(16.0))
                            .text_color(Color32::from_rgb(220, 230, 240)));
                    });
                });

                ui.add_space(20.0);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                    let btn_text = if self.note_selectionnee.is_some() { "✨ METTRE À JOUR LA NOTE" } else { "💾 VALIDER ET ENREGISTRER" };
                    if ui.add(egui::Button::new(RichText::new(btn_text).strong().size(18.0).color(Color32::BLACK))
                        .fill(neon_blue)
                        .min_size(Vec2::new(300.0, 50.0))
                        .corner_radius(10.0)).clicked() {
                        self.sauvegarder_edition();
                    }

                    ui.add_space(15.0);

                    if let Some(id) = self.note_selectionnee {
                        if ui.add(egui::Button::new(RichText::new("🗑 SUPPRIMER").strong().color(Color32::WHITE))
                            .fill(error_red.linear_multiply(0.6))
                            .min_size(Vec2::new(150.0, 50.0))
                            .corner_radius(10.0)).clicked() {
                            self.supprimer_note(id);
                        }
                    }
                });
                
                ui.add_space(10.0);
            });
        });

        if self.show_stats {
            egui::Window::new("📊 STATISTIQUES DU NOYAU").open(&mut self.show_stats).show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(15.0);
                    ui.label(RichText::new(format!("{}", self.state.notes.len())).size(60.0).color(neon_blue));
                    ui.label(RichText::new("NOTES ENREGISTRÉES").strong());
                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(15.0);
                    ui.label(RichText::new(format!("TAGS UNIQUES: {}", self.state.tags_uniques().len())).color(nebula_purple));
                    ui.add_space(15.0);
                });
            });
        }
    }
}
