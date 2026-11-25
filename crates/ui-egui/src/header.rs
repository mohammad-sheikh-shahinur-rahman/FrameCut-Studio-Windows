//! The header bar (Premiere 26 layout, 38 pt): Home and Import / Edit / Export on the left, the
//! document title centred, the workspace name and quick actions on the right. On macOS the menus
//! live in the native menu bar (set up by the app); elsewhere a compact in-window menu bar follows
//! the mode tabs.

use egui::{Color32, Rect, Sense, Stroke, pos2, vec2};

use crate::FilmcraftApp;
use crate::icons::{self, Icon};
use crate::state::Mode;
use crate::theme::Tokens;

/// Height of the header bar.
pub const HEIGHT: f32 = 62.0;

/// Dynamic height depending on whether in-window menu bar is displayed.
pub fn height(app: &FilmcraftApp) -> f32 {
    if app.ui.show_menu_bar { 62.0 } else { 38.0 }
}

/// Draws the header bar in `rect` and handles its clicks: Home, the mode tabs, the workspace name
/// and its menu (the workspaces, then Reset to Saved Layout, Save Changes to this Workspace, Save as
/// New Workspace… and Edit Workspaces…), Community and the quick actions.
pub fn show(app: &mut FilmcraftApp, ui: &mut egui::Ui, rect: Rect) {
    let t = app.tokens;
    let p = ui.painter().clone();

    // When show_menu_bar is true, top 24px is the dedicated Premiere Pro Menu Bar strip.
    // The remaining height is the Workspace and Mode bar.
    let (menu_strip, ws_rect) = if app.ui.show_menu_bar {
        let m = Rect::from_min_size(rect.min, vec2(rect.width(), 24.0));
        let w = Rect::from_min_max(pos2(rect.min.x, rect.min.y + 24.0), rect.max);
        (Some(m), w)
    } else {
        (None, rect)
    };

    // 1. Draw Menu Bar strip if present
    if let Some(m_rect) = menu_strip {
        p.rect_filled(m_rect, 0.0, Color32::from_rgb(18, 19, 22));
        p.line_segment([m_rect.left_bottom(), m_rect.right_bottom()], Stroke::new(1.0, Color32::from_rgb(32, 34, 38)));
        let mut mu = ui.new_child(egui::UiBuilder::new().max_rect(m_rect.shrink2(vec2(10.0, 0.0))).layout(egui::Layout::left_to_right(egui::Align::Center)));
        mu.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
        mu.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::NONE;
        mu.style_mut().visuals.override_text_color = Some(t.text_dim);
        crate::menus::menu_bar(app, &mut mu);
    }

    // 2. Draw Workspace / Navigation strip
    p.rect_filled(ws_rect, 0.0, t.header_bg);
    p.line_segment([ws_rect.left_bottom(), ws_rect.right_bottom()], Stroke::new(1.0, Color32::BLACK));

    let drag = ui.interact(ws_rect, egui::Id::unique("header-drag"), Sense::click_and_drag());
    if drag.drag_started_by(egui::PointerButton::Primary) {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }
    if drag.double_clicked() {
        let max = ui.ctx().input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(!max));
    }

    let mut x = ws_rect.min.x + if app.integrated_titlebar { 104.0 } else { 14.0 };

    // Home button
    let home = Rect::from_center_size(pos2(x + 10.0, ws_rect.center().y), vec2(26.0, 26.0));
    let hresp = ui.interact(home, egui::Id::unique("hdr-home"), Sense::click()).on_hover_text(tl!("Home"));
    app.auto.add("header.home", home, "Home");
    if hresp.hovered() {
        p.rect_filled(home, 4.0, t.hover);
    }
    icons::paint(&p, home.shrink(5.0), Icon::Home, if hresp.hovered() { t.text } else { t.text_dim });
    if hresp.clicked() {
        app.ui.mode = Mode::Import;
    }
    x = home.max.x + 18.0;

    // Studio Mode tabs: Import, Edit, Export (matches Adobe Premiere Pro modern layout)
    struct TabDef {
        id: &'static str,
        label: String,
        is_active: bool,
    }

    let tabs = [
        TabDef { id: "import", label: tl!("Import").to_string(), is_active: app.ui.mode == Mode::Import },
        TabDef { id: "edit", label: tl!("Edit").to_string(), is_active: app.ui.mode == Mode::Edit },
        TabDef { id: "export", label: tl!("Export").to_string(), is_active: app.ui.mode == Mode::Export },
    ];

    for tab in tabs {
        let galley = p.layout_no_wrap(tab.label.clone(), Tokens::semibold(13.5), if tab.is_active { t.tab_text_active } else { t.tab_text });
        let r = Rect::from_min_size(pos2(x - 5.0, ws_rect.min.y + 6.0), vec2(galley.size().x + 10.0, ws_rect.height() - 12.0));
        let resp = ui.interact(r, egui::Id::unique(("mode_tab", tab.id)), Sense::click());
        app.auto.add(&format!("header.mode.{}", tab.id), r, &tab.label);
        let col = if tab.is_active || resp.hovered() { Color32::WHITE } else { t.tab_text };
        let gw = galley.size().x;
        p.galley_with_override_text_color(pos2(x, ws_rect.center().y - galley.size().y / 2.0), galley, col);
        if tab.is_active {
            p.line_segment([pos2(x, ws_rect.max.y - 3.0), pos2(x + gw, ws_rect.max.y - 3.0)], Stroke::new(2.5, Color32::from_rgb(0, 140, 255)));
        }
        if resp.clicked() {
            match tab.id {
                "import" => app.ui.mode = Mode::Import,
                "edit" => app.ui.mode = Mode::Edit,
                "export" => app.ui.mode = Mode::Export,
                _ => {}
            }
        }
        x += gw + 22.0;
    }
    let left_end = x;

    // Document title, centered
    let title = if app.session.is_dirty() { tlf!("{name} - Edited", name = app.session.project.name) } else { app.session.project.name.clone() };

    // Right cluster: icons and workspace name
    let mut rx = ws_rect.max.x - 14.0;
    let btn = |ui: &mut egui::Ui, icon: Icon, id: &str, tip: &str, app: &mut FilmcraftApp, rx: &mut f32| -> egui::Response {
        let r = Rect::from_center_size(pos2(*rx - 14.0, ws_rect.center().y), vec2(28.0, 28.0));
        *rx -= 36.0;
        let resp = ui.interact(r, egui::Id::unique(("hdr", id)), Sense::click()).on_hover_text(tip);
        app.auto.add(&format!("header.{id}"), r, tip);
        if resp.hovered() {
            ui.painter().rect_filled(r, 4.0, t.hover);
        }
        icons::paint(ui.painter(), r.shrink(6.0), icon, if resp.hovered() { t.text } else { t.text_dim });
        resp
    };

    if btn(ui, Icon::Fullscreen, "fullscreen", tl!("Full screen"), app, &mut rx).clicked() {
        let fs = ui.ctx().input(|i| i.viewport().fullscreen.unwrap_or(false));
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Fullscreen(!fs));
    }
    if btn(ui, Icon::Speaker, "volume", tl!("Volume"), app, &mut rx).clicked() {
        app.ui.status = tl!("Master volume: use the Audio Track Mixer").into();
    }
    if btn(ui, Icon::Search, "search", tl!("Search"), app, &mut rx).clicked() {
        app.show_panel(crate::dock::PanelKind::Effects);
    }
    if btn(ui, Icon::Bell, "notifications", tl!("Progress"), app, &mut rx).clicked() {
        app.ui.mode = Mode::Export;
    }

    // Community button (header.discord for test compatibility)
    if btn(ui, Icon::Globe, "discord", tl!("FrameCut Studio Community"), app, &mut rx).clicked() {
        crate::links::open(app, ui.ctx(), crate::links::DISCORD);
    }

    // Quick Export button
    let qx = rx;
    if btn(ui, Icon::Export, "quickExport", tl!("Quick Export"), app, &mut rx).clicked() {
        app.ui.export.quick_open = !app.ui.export.quick_open;
        ui.ctx().data_mut(|d| d.insert_temp(egui::Id::unique("quick-export-toggled"), true));
    }
    crate::panels::export_mode::quick_export(app, ui.ctx(), pos2(qx - 340.0, ws_rect.max.y + 4.0));

    // Appearance Mode
    let (icon, mode) = match app.session.prefs.appearance.appearance_mode.as_str() {
        "auto" => (Icon::Monitor, tl!("Sync with system")),
        "light" => (Icon::Sun, tl!("Light")),
        _ => (Icon::Moon, tl!("Dark")),
    };
    let tip = format!("{}: {mode}", tl!("Appearance Mode"));
    let appearance = btn(ui, icon, "appearance", &tip, app, &mut rx);
    appearance.context_menu(|ui| crate::panels::settings::appearance_context_menu(app, ui));
    if appearance.clicked() {
        crate::panels::settings::cycle_appearance(app, ui.ctx());
    }

    let ws_resp = btn(ui, Icon::Workspaces, "workspaces", tl!("Workspaces"), app, &mut rx);

    // Workspace name badge (caps)
    let ws = crate::i18n::to_uppercase(crate::i18n::t(&app.ui.workspace));
    let wg = p.layout_no_wrap(ws.clone(), Tokens::ui(11.0), t.text_dim);
    let wr = Rect::from_min_size(pos2(rx - wg.size().x + 8.0, ws_rect.center().y - 10.0), vec2(wg.size().x + 8.0, 20.0));
    let wresp = ui.interact(wr, egui::Id::unique("hdr-ws-name"), Sense::click());
    app.auto.add("header.workspaceName", wr, &ws);
    p.galley_with_override_text_color(pos2(wr.min.x + 4.0, ws_rect.center().y - wg.size().y / 2.0), wg, if wresp.hovered() { t.text } else { t.text_dim });

    // Document title, cleanly centered in available space with ample room
    {
        let left = left_end + 16.0;
        let right = wr.min.x - 16.0;
        if right > left + 40.0 {
            let mut job = egui::text::LayoutJob::simple_singleline(title, Tokens::semibold(13.5), t.tab_text_active);
            job.wrap.max_width = right - left;
            job.wrap.max_rows = 1;
            let galley = p.layout_job(job);
            let half = galley.size().x / 2.0;
            let center = ws_rect.center().x;
            let draw_x = if center - half >= left && center + half <= right { center - half } else { left };
            p.with_clip_rect(Rect::from_min_max(pos2(left, ws_rect.min.y), pos2(right, ws_rect.max.y))).galley(
                pos2(draw_x, ws_rect.center().y - galley.size().y / 2.0),
                galley,
                t.tab_text_active,
            );
        }
    }

    let popup_id = egui::Id::unique("workspaces-popup");
    if ws_resp.clicked() || wresp.clicked() {
        ui.ctx().data_mut(|d| d.insert_temp(popup_id, true));
    }
    let open = ui.ctx().data(|d| d.get_temp::<bool>(popup_id).unwrap_or(false));
    if open {
        let anchor = pos2(wr.min.x, ws_rect.max.y + 4.0);
        let area = egui::Area::new(popup_id.with("area")).order(egui::Order::Foreground).fixed_pos(anchor).show(ui.ctx(), |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                    ui.set_min_width(220.0);
                    for w in crate::dock::names(&app.workspaces) {
                        let sel = app.ui.workspace == w;
                        if ui.selectable_label(sel, crate::i18n::t(&w)).clicked() {
                            app.set_workspace(&w);
                            ui.ctx().data_mut(|d| d.insert_temp(popup_id, false));
                        }
                    }
                    ui.separator();
                    let actions = [
                        ("reset", tl!("Reset to Saved Layout")),
                        ("saveChanges", tl!("Save Changes to this Workspace")),
                        ("saveAs", tl!("Save as New Workspace…")),
                        ("edit", tl!("Edit Workspaces…")),
                    ];
                    for (action, label) in actions {
                        let resp = ui.button(label);
                        app.auto.add(&format!("header.workspaces.{action}"), resp.rect, label);
                        if resp.clicked() {
                            ui.ctx().data_mut(|d| d.insert_temp(popup_id, false));
                            let ctx = ui.ctx().clone();
                            if let Err(e) = crate::menus::invoke(app, &ctx, &format!("window.workspace.{action}"), serde_json::json!({})) {
                                app.ui.status = e;
                            }
                        }
                    }
                });
            });
        });
        if area.response.clicked_elsewhere() && !(ws_resp.clicked() || wresp.clicked()) {
            ui.ctx().data_mut(|d| d.insert_temp(popup_id, false));
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn workspace_name_uppercasing_is_turkish_aware() {
        crate::i18n::set_current(crate::i18n::Language::Tr);
        assert_eq!(crate::i18n::to_uppercase("Birleştirme"), "BİRLEŞTİRME");
        assert_eq!(crate::i18n::to_uppercase("Altyazılar ve Grafikler"), "ALTYAZILAR VE GRAFİKLER");
        crate::i18n::set_current(crate::i18n::Language::En);
        assert_eq!(crate::i18n::to_uppercase("Birleştirme"), "BIRLEŞTIRME");
    }
}
