//! Import mode (header "Import", also Home): Premiere Pro style Welcome & Ingest Dashboard
//! with left navigation rail, rich quick start template cards, and recent projects.

use egui::{Align2, Color32, Rect, Sense, Stroke, pos2, vec2};
use serde_json::json;

use crate::FilmcraftApp;
use crate::theme::Tokens;

pub fn show(app: &mut FilmcraftApp, ui: &mut egui::Ui, rect: Rect) {
    let t = app.tokens;
    let p = ui.painter().clone();
    p.rect_filled(rect, 0.0, t.panel_bg);

    let ctx = ui.ctx().clone();
    let sidebar_w = 210.0_f32;

    // -------------------------------------------------------------
    // Left Navigation Rail (Premiere Pro Style)
    // -------------------------------------------------------------
    let sb_rect = Rect::from_min_max(rect.min, pos2(rect.min.x + sidebar_w, rect.max.y));
    p.rect_filled(sb_rect, 0.0, Color32::from_rgb(18, 19, 22));
    p.line_segment([sb_rect.right_top(), sb_rect.right_bottom()], Stroke::new(1.0, Color32::from_rgb(34, 36, 42)));

    // "+ New Project" button
    let np_rect = Rect::from_min_size(sb_rect.min + vec2(16.0, 20.0), vec2(sidebar_w - 32.0, 36.0));
    let np_resp = ui.interact(np_rect, egui::Id::unique("sb-new-project"), Sense::click());
    let np_bg = if np_resp.hovered() { Color32::from_rgb(20, 135, 255) } else { Color32::from_rgb(0, 110, 240) };
    p.rect_filled(np_rect, 6.0, np_bg);
    crate::icons::paint(&p, Rect::from_center_size(pos2(np_rect.min.x + 22.0, np_rect.center().y), vec2(14.0, 14.0)), crate::icons::Icon::Plus, Color32::WHITE);
    p.text(pos2(np_rect.min.x + 38.0, np_rect.center().y), Align2::LEFT_CENTER, tl!("New Project"), Tokens::semibold(13.5), Color32::WHITE);
    if np_resp.clicked() {
        let _ = crate::menus::invoke(app, &ctx, "file.newProject", json!({}));
        app.ui.mode = crate::state::Mode::Edit;
    }

    // "Open Project…" button
    let op_rect = Rect::from_min_size(sb_rect.min + vec2(16.0, 64.0), vec2(sidebar_w - 32.0, 32.0));
    let op_resp = ui.interact(op_rect, egui::Id::unique("sb-open-project"), Sense::click());
    let op_bg = if op_resp.hovered() { Color32::from_rgb(38, 42, 50) } else { Color32::from_rgb(26, 28, 34) };
    p.rect_filled(op_rect, 6.0, op_bg);
    p.rect_stroke(op_rect, 6.0, Stroke::new(1.0, Color32::from_rgb(50, 54, 64)), egui::StrokeKind::Inside);
    crate::icons::paint(&p, Rect::from_center_size(pos2(op_rect.min.x + 22.0, op_rect.center().y), vec2(14.0, 14.0)), crate::icons::Icon::Folder, t.text_dim);
    p.text(pos2(op_rect.min.x + 38.0, op_rect.center().y), Align2::LEFT_CENTER, tl!("Open Project…"), Tokens::ui(12.5), t.text);
    if op_resp.clicked() {
        let _ = crate::menus::invoke(app, &ctx, "file.open", json!({}));
        app.ui.mode = crate::state::Mode::Edit;
    }

    // Navigation Category items
    let tab_id = egui::Id::unique("import-sidebar-tab");
    let mut active_tab = ui.ctx().data(|d| d.get_temp::<u8>(tab_id)).unwrap_or(0);

    let mut ny = sb_rect.min.y + 116.0;
    p.text(pos2(sb_rect.min.x + 18.0, ny), Align2::LEFT_CENTER, tl!("PROJECTS"), Tokens::ui(10.5), t.text_faint);
    ny += 16.0;

    let nav_items = [
        ("Home", crate::icons::Icon::Home),
        ("Recent Projects", crate::icons::Icon::Sequence),
        ("Quick Demos", crate::icons::Icon::Film),
        ("Tutorials & Guide", crate::icons::Icon::Info),
    ];
    for (idx, (label_text, icon)) in nav_items.iter().enumerate() {
        let idx_u8 = idx as u8;
        let active = active_tab == idx_u8;
        let nr = Rect::from_min_size(pos2(sb_rect.min.x + 12.0, ny), vec2(sidebar_w - 24.0, 30.0));
        let resp = ui.interact(nr, egui::Id::unique(("sb-nav", *label_text)), Sense::click());
        app.auto.add(&format!("import.nav.{idx}"), nr, label_text);
        if active {
            p.rect_filled(nr, 5.0, Color32::from_rgba_unmultiplied(0, 110, 240, 35));
            p.rect_filled(Rect::from_min_size(nr.min + vec2(0.0, 6.0), vec2(3.0, 18.0)), 1.5, Color32::from_rgb(0, 140, 255));
        } else if resp.hovered() {
            p.rect_filled(nr, 5.0, t.hover);
        }
        let col = if active {
            Color32::WHITE
        } else if resp.hovered() {
            t.text
        } else {
            t.text_dim
        };
        crate::icons::paint(&p, Rect::from_center_size(pos2(nr.min.x + 16.0, nr.center().y), vec2(14.0, 14.0)), *icon, col);
        p.text(pos2(nr.min.x + 32.0, nr.center().y), Align2::LEFT_CENTER, crate::i18n::t(label_text), Tokens::ui(12.5), col);
        if resp.clicked() {
            active_tab = idx_u8;
            ui.ctx().data_mut(|d| d.insert_temp(tab_id, active_tab));
        }
        ny += 32.0;
    }

    // Bottom Hardware Info Badge
    let hw_rect = Rect::from_min_size(pos2(sb_rect.min.x + 12.0, sb_rect.max.y - 74.0), vec2(sidebar_w - 24.0, 60.0));
    p.rect_filled(hw_rect, 6.0, Color32::from_rgb(22, 24, 28));
    p.rect_stroke(hw_rect, 6.0, Stroke::new(1.0, Color32::from_rgb(34, 38, 46)), egui::StrokeKind::Inside);
    p.circle_filled(pos2(hw_rect.min.x + 16.0, hw_rect.min.y + 18.0), 3.5, Color32::from_rgb(0, 210, 130));
    p.text(pos2(hw_rect.min.x + 28.0, hw_rect.min.y + 18.0), Align2::LEFT_CENTER, "Direct3D 12 Native", Tokens::semibold(11.5), t.text);
    p.text(pos2(hw_rect.min.x + 16.0, hw_rect.min.y + 38.0), Align2::LEFT_CENTER, "100% On-Device · Zero Cloud", Tokens::ui(10.5), t.text_dim);

    // -------------------------------------------------------------
    // Main Content Hub (Right of Sidebar)
    // -------------------------------------------------------------
    let main_x = sb_rect.max.x + 32.0;
    let main_top = rect.min.y + 24.0;
    let content_w = (rect.width() - sidebar_w - 64.0).max(400.0);

    let (hero_title, hero_sub) = match active_tab {
        1 => (tl!("Recent Projects"), tl!("Reopen your recent FrameCut Studio projects or browse from disk.")),
        2 => (tl!("Quick Demos & Sample Media"), tl!("Explore multi-track editing, transitions, color grading, and calibration signals.")),
        3 => (tl!("Tutorials & Guide"), tl!("Jump straight into any FrameCut Studio workspace or customize your keyboard shortcuts.")),
        _ => (tl!("Welcome to FrameCut Studio"), tl!("Professional Non-Linear Video Editing & Compositor · Real-time 60fps Native Pipeline")),
    };

    // Hero Greeting (clipped to content_w so text never overflows on small screens)
    let mut ht_job = egui::text::LayoutJob::simple_singleline(hero_title.to_string(), Tokens::semibold(22.0), t.text);
    ht_job.wrap.max_width = content_w;
    ht_job.wrap.max_rows = 1;
    let ht_g = p.layout_job(ht_job);
    p.galley(pos2(main_x, main_top + 4.0), ht_g, t.text);

    let mut hs_job = egui::text::LayoutJob::simple(hero_sub.to_string(), Tokens::ui(12.5), t.text_dim, content_w);
    hs_job.wrap.max_rows = 2;
    let hs_g = p.layout_job(hs_job);
    p.galley(pos2(main_x, main_top + 32.0), hs_g, t.text_dim);

    let section1_y = main_top + 68.0;
    let next_y = match active_tab {
        1 => recent(app, ui, pos2(main_x, section1_y), content_w, 10),
        2 => {
            p.text(pos2(main_x, section1_y), Align2::LEFT_CENTER, tl!("Sample Sequences & Calibration"), Tokens::semibold(14.0), t.text);
            let next = draw_quick_start_cards(app, ui, &ctx, pos2(main_x, section1_y + 14.0), content_w, 165.0);
            next + 16.0
        }
        3 => draw_tutorials_view(app, ui, &ctx, pos2(main_x, section1_y), content_w),
        _ => {
            p.text(pos2(main_x, section1_y), Align2::LEFT_CENTER, tl!("Quick Start & Sample Sequences"), Tokens::semibold(14.0), t.text);
            let cards_bottom = draw_quick_start_cards(app, ui, &ctx, pos2(main_x, section1_y + 14.0), content_w, 145.0);
            recent(app, ui, pos2(main_x, cards_bottom + 22.0), content_w, RECENT_ROWS)
        }
    };

    // Section 3: Bottom Premiere Pro Ingest / Launch Bar
    let launch_bar_h = 50.0_f32;
    let launch_y = (next_y + 12.0).max(rect.max.y - launch_bar_h - 60.0);
    let lb_rect = Rect::from_min_size(pos2(main_x, launch_y), vec2(content_w, launch_bar_h));
    p.rect_filled(lb_rect, 8.0, Color32::from_rgb(18, 19, 22));
    p.rect_stroke(lb_rect, 8.0, Stroke::new(1.0, Color32::from_rgb(34, 38, 46)), egui::StrokeKind::Inside);

    let mut lb_job = egui::text::LayoutJob::simple_singleline(
        tl!("Project Sequence: 1920 × 1080 @ 24 fps · Rec.709 Working Space · 32-bit Float Audio").to_string(),
        Tokens::ui(12.5),
        t.text_dim,
    );
    lb_job.wrap.max_width = (lb_rect.width() - 246.0).max(120.0);
    lb_job.wrap.max_rows = 1;
    let lb_g = p.layout_job(lb_job);
    p.galley(pos2(lb_rect.min.x + 20.0, lb_rect.center().y - lb_g.size().y / 2.0), lb_g, t.text_dim);

    let start_btn_rect = Rect::from_min_size(pos2(lb_rect.max.x - 220.0, lb_rect.center().y - 17.0), vec2(206.0, 34.0));
    let start_resp = ui.interact(start_btn_rect, egui::Id::unique("start-editing-btn"), Sense::click());
    let start_bg = if start_resp.hovered() { Color32::from_rgb(20, 135, 255) } else { Color32::from_rgb(0, 110, 240) };
    p.rect_filled(start_btn_rect, 6.0, start_bg);
    p.text(pos2(start_btn_rect.center().x, start_btn_rect.center().y), Align2::CENTER_CENTER, tl!("Start Editing →"), Tokens::semibold(13.0), Color32::WHITE);
    if start_resp.clicked() {
        app.ui.mode = crate::state::Mode::Edit;
    }

    // Community and Resources footer links
    let res_y = launch_y + launch_bar_h + 16.0;
    let mut rx_link = main_x;
    let resource_links = [
        ("website", "FrameCut Studio Website", crate::links::WEBSITE, crate::icons::Icon::Globe),
        ("appPage", "FrameCut Studio Documentation", crate::links::APP_PAGE, crate::icons::Icon::Globe),
        ("github", "FrameCut Studio on GitHub", crate::links::GITHUB, crate::icons::Icon::Code),
        ("discord", "FrameCut Studio Community", crate::links::DISCORD, crate::icons::Icon::Globe),
        ("issues", "Report an Issue", crate::links::ISSUES, crate::icons::Icon::Code),
    ];
    for (id, label_text, url, icon) in resource_links {
        let label = crate::i18n::t(label_text);
        let g = p.layout_no_wrap(label.to_string(), Tokens::ui(12.0), t.text_dim);
        let r = Rect::from_min_size(pos2(rx_link, res_y), vec2(g.size().x + 36.0, 26.0));
        let resp = ui.interact(r, egui::Id::unique(("imp-link", id)), Sense::click()).on_hover_text(url);
        app.auto.add(&format!("import.link.{id}"), r, label);
        let bg = if resp.hovered() { t.hover } else { Color32::from_rgb(24, 26, 30) };
        p.rect_filled(r, 6.0, bg);
        crate::icons::paint(
            &p,
            Rect::from_center_size(pos2(r.min.x + 14.0, r.center().y), vec2(13.0, 13.0)),
            icon,
            if resp.hovered() { t.text } else { t.text_dim },
        );
        p.galley(pos2(r.min.x + 26.0, r.center().y - g.size().y / 2.0), g, if resp.hovered() { t.text } else { t.text_dim });
        if resp.clicked() {
            crate::links::open(app, &ctx, url);
        }
        rx_link = r.max.x + 10.0;
    }
}

fn draw_quick_start_cards(app: &mut FilmcraftApp, ui: &mut egui::Ui, ctx: &egui::Context, pos: egui::Pos2, max_w: f32, ch: f32) -> f32 {
    let t = app.tokens;
    let p = ui.painter().clone();
    let cards = [
        (tl!("Demo project"), "file.openDemoProject", tl!("Multi-track cut with transitions, grades & audio"), "FEATURED", Color32::from_rgb(180, 150, 255)),
        (tl!("Browse files…"), "file.import", tl!("Import video clips, 4K footage, audio & stills"), "INGEST", Color32::from_rgb(100, 180, 255)),
        (
            tl!("Demo footage"),
            "file.importDemoFootage",
            tl!("Six high-bitrate procedural 1080p video clips"),
            "PROCEDURAL UHD",
            Color32::from_rgb(85, 230, 185),
        ),
        (tl!("Bars and Tone"), "file.newBarsAndTone", tl!("SMPTE HD reference bars with 1 kHz audio"), "CALIBRATION", Color32::from_rgb(255, 185, 90)),
    ];

    let gap = 14.0_f32;
    let cw = ((max_w - gap * 3.0) / 4.0).clamp(190.0, 260.0);
    for (i, (title, cmd, sub, badge, accent_col)) in cards.iter().enumerate() {
        let r = Rect::from_min_size(pos + vec2(i as f32 * (cw + gap), 0.0), vec2(cw, ch));
        let resp = ui.interact(r, egui::Id::unique(("imp", *cmd)), Sense::click());
        app.auto.add(&format!("import.{cmd}"), r, title);

        let bg = if resp.hovered() { Color32::from_rgb(32, 34, 40) } else { Color32::from_rgb(24, 25, 29) };
        p.rect_filled(r, 8.0, bg);
        let border_stroke = if resp.hovered() { Stroke::new(1.5, Color32::from_rgb(0, 140, 255)) } else { Stroke::new(1.0, Color32::from_rgb(40, 42, 48)) };
        p.rect_stroke(r, 8.0, border_stroke, egui::StrokeKind::Inside);

        crate::icons::paint(
            &p,
            Rect::from_min_size(r.min + vec2(14.0, 14.0), vec2(36.0, 36.0)),
            [crate::icons::Icon::Sequence, crate::icons::Icon::Folder, crate::icons::Icon::Film, crate::icons::Icon::Grid][i],
            *accent_col,
        );
        p.text(pos2(r.max.x - 12.0, r.min.y + 18.0), Align2::RIGHT_CENTER, *badge, Tokens::ui(10.0), *accent_col);
        p.text(r.min + vec2(14.0, 78.0), Align2::LEFT_CENTER, *title, Tokens::semibold(14.0), t.text);

        let mut sub_job = egui::text::LayoutJob::simple(sub.to_string(), Tokens::ui(11.5), t.text_dim, cw - 28.0);
        sub_job.wrap.max_rows = 2;
        let sub_g = p.layout_job(sub_job);
        p.galley(r.min + vec2(14.0, 94.0), sub_g, t.text_dim);

        if resp.clicked() {
            let _ = crate::menus::invoke(app, ctx, cmd, json!({}));
            app.ui.mode = crate::state::Mode::Edit;
        }
    }
    pos.y + ch
}

fn draw_tutorials_view(app: &mut FilmcraftApp, ui: &mut egui::Ui, ctx: &egui::Context, pos: egui::Pos2, max_w: f32) -> f32 {
    let t = app.tokens;
    let p = ui.painter().clone();
    p.text(pos, Align2::LEFT_CENTER, tl!("Interactive Workflow Guides"), Tokens::semibold(14.0), t.text);

    let guides = [
        (
            "1. Timeline & Magnetic Editing",
            "Cut, ripple-trim, slip, and arrange multi-track video and audio clips.",
            "Open Editing Workspace →",
            crate::icons::Icon::Sequence,
            "Editing",
        ),
        (
            "2. Color Grading & Scopes",
            "3-Way Color Wheels, RGB Curves, HSL Secondary, and .cube LUTs.",
            "Open Color Workspace →",
            crate::icons::Icon::Adjust,
            "Color",
        ),
        (
            "3. Audio DSP & Mixing",
            "Real-time track mixer, peak meters, parametric EQ, and voiceover.",
            "Open Audio Workspace →",
            crate::icons::Icon::Audio,
            "Audio",
        ),
        (
            "4. Keyboard Shortcuts",
            "View and customize Premiere-compatible hotkeys for fast editing.",
            "Open Shortcuts →",
            crate::icons::Icon::Gear,
            "shortcuts",
        ),
    ];

    let gap = 14.0_f32;
    let cw = ((max_w - gap) / 2.0).clamp(240.0, 480.0);
    let ch = 104.0_f32;
    for (i, (title, desc, cta, icon, target)) in guides.iter().enumerate() {
        let col_idx = (i % 2) as f32;
        let row_idx = (i / 2) as f32;
        let r = Rect::from_min_size(pos + vec2(col_idx * (cw + gap), 16.0 + row_idx * (ch + gap)), vec2(cw, ch));
        let resp = ui.interact(r, egui::Id::unique(("imp-guide", i)), Sense::click());

        let bg = if resp.hovered() { Color32::from_rgb(32, 34, 40) } else { Color32::from_rgb(24, 25, 29) };
        p.rect_filled(r, 8.0, bg);
        let stroke = if resp.hovered() { Stroke::new(1.5, Color32::from_rgb(0, 140, 255)) } else { Stroke::new(1.0, Color32::from_rgb(40, 42, 48)) };
        p.rect_stroke(r, 8.0, stroke, egui::StrokeKind::Inside);

        crate::icons::paint(&p, Rect::from_min_size(r.min + vec2(16.0, 16.0), vec2(24.0, 24.0)), *icon, Color32::from_rgb(0, 150, 255));
        p.text(r.min + vec2(50.0, 28.0), Align2::LEFT_CENTER, crate::i18n::t(title), Tokens::semibold(13.5), t.text);

        let mut job = egui::text::LayoutJob::simple(crate::i18n::t(desc).to_string(), Tokens::ui(11.5), t.text_dim, cw - 32.0);
        job.wrap.max_rows = 2;
        p.galley(r.min + vec2(16.0, 48.0), p.layout_job(job), t.text_dim);
        p.text(pos2(r.min.x + 16.0, r.max.y - 16.0), Align2::LEFT_CENTER, crate::i18n::t(cta), Tokens::semibold(11.5), Color32::from_rgb(80, 175, 255));

        if resp.clicked() {
            if *target == "shortcuts" {
                let _ = crate::menus::invoke(app, ctx, "app.keyboardShortcuts", json!({}));
            } else {
                app.set_workspace(target);
                app.ui.mode = crate::state::Mode::Edit;
            }
        }
    }

    pos.y + 16.0 + 2.0 * (ch + gap)
}

/// Recent projects listed on Home (Import mode).
const RECENT_ROWS: usize = 5;
const RECENT_ROW_H: f32 = 32.0;

fn recent(app: &mut FilmcraftApp, ui: &mut egui::Ui, pos: egui::Pos2, max_w: f32, limit: usize) -> f32 {
    let t = app.tokens;
    let p = ui.painter().clone();
    p.text(pos, Align2::LEFT_CENTER, tl!("Recent Projects"), Tokens::semibold(14.0), t.text);

    let paths: Vec<String> = app.session.prefs.general.recent_projects.iter().take(limit).cloned().collect();
    if paths.is_empty() {
        let empty_rect = Rect::from_min_size(pos + vec2(0.0, 14.0), vec2(max_w.min(560.0), 48.0));
        p.rect_filled(empty_rect, 6.0, Color32::from_rgb(24, 25, 29));
        p.rect_stroke(empty_rect, 6.0, Stroke::new(1.0, Color32::from_rgb(36, 38, 44)), egui::StrokeKind::Inside);
        p.text(
            pos2(empty_rect.min.x + 16.0, empty_rect.center().y),
            Align2::LEFT_CENTER,
            tl!("No recent projects yet. Click '+ New Project' or open the 'Demo project' above to start editing."),
            Tokens::ui(12.0),
            t.text_dim,
        );
        return pos.y + 14.0 + 48.0;
    }

    let mut open = None;
    for (i, path) in paths.iter().enumerate() {
        let row = Rect::from_min_size(pos + vec2(0.0, 14.0 + i as f32 * (RECENT_ROW_H + 4.0)), vec2(max_w.min(620.0), RECENT_ROW_H));
        let name = crate::menus::recent_name(path);
        let resp = ui.interact(row, egui::Id::unique(("imp-recent", i)), Sense::click()).on_hover_text(path.as_str());
        app.auto.add(&format!("import.recent.{i}"), row, &name);

        let bg = if resp.hovered() { t.hover } else { Color32::from_rgb(24, 25, 29) };
        p.rect_filled(row, 6.0, bg);
        p.rect_stroke(row, 6.0, Stroke::new(1.0, Color32::from_rgb(36, 38, 44)), egui::StrokeKind::Inside);

        let icon_rect = Rect::from_center_size(pos2(row.min.x + 18.0, row.center().y), vec2(16.0, 16.0));
        crate::icons::paint(&p, icon_rect, crate::icons::Icon::Sequence, Color32::from_rgb(0, 140, 255));

        let name_r = p.text(pos2(row.min.x + 36.0, row.center().y), Align2::LEFT_CENTER, &name, Tokens::semibold(13.0), t.text);
        if let Some(folder) = std::path::Path::new(path).parent() {
            p.text(pos2(name_r.max.x + 14.0, row.center().y), Align2::LEFT_CENTER, folder.to_string_lossy(), Tokens::ui(11.5), t.text_dim);
        }

        if resp.clicked() {
            open = Some(path.clone());
        }
    }

    if let Some(path) = open {
        let ctx = ui.ctx().clone();
        if crate::panels::clip_dialogs::ask_before_open(app, &path).is_some() {
            if let Some(info) = app.ui.clip_dialog.as_mut().and_then(|d| d.info.as_object_mut()) {
                info.insert("editAfterOpen".into(), json!(true));
            }
        } else if crate::menus::invoke(app, &ctx, "file.open", json!({"path": path})).is_ok() {
            app.ui.mode = crate::state::Mode::Edit;
        }
    }

    pos.y + 14.0 + paths.len() as f32 * (RECENT_ROW_H + 4.0)
}
