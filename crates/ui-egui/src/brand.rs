//! The Pixelune brand mark at the left of the title bar, where Photoshop shows its "Ps" tile.
//!
//! The mark is the Septet "7": seven strokes, three across the top and four down the diagonal,
//! with the middle top stroke in the accent — the same mark the landing page draws, so the
//! editor and the site read as one product. It is drawn as line segments rather than an image,
//! so it stays crisp at any display scale and needs no texture.

use egui::{Color32, Rect, Ui, vec2};

/// The mark's design box, matching the site's `viewBox="0 0 28 30"`.
const BOX: (f32, f32) = (28.0, 30.0);
/// Stroke width in the design box (the site uses 3.2).
const STROKE: f32 = 3.2;

/// The three top strokes and the four diagonal ones, in design-box coordinates.
const STROKES: [(f32, f32, f32, f32); 7] = [
    (3.4, 4.2, 10.4, 4.2),
    (10.4, 4.2, 17.4, 4.2),
    (17.4, 4.2, 24.4, 4.2),
    (24.4, 4.2, 21.1, 10.4),
    (21.1, 10.4, 18.4, 15.6),
    (18.4, 15.6, 15.7, 20.8),
    (15.7, 20.8, 13.0, 26.0),
];

/// Where the mark was painted this frame (tests and screenshots).
fn rect_id() -> egui::Id {
    egui::Id::new("photocraft-brand-mark-rect")
}

/// Paint the mark into `r`, keeping its proportions and centring it.
pub fn paint_mark(ui: &Ui, r: Rect) {
    let t = crate::theme::Tokens::get(ui.ctx());
    let scale = (r.width() / BOX.0).min(r.height() / BOX.1);
    let origin = r.center() - vec2(BOX.0, BOX.1) * scale / 2.0;
    let at = |x: f32, y: f32| origin + vec2(x, y) * scale;
    let width = (STROKE * scale).max(1.0);
    let p = ui.painter();
    for (i, (x1, y1, x2, y2)) in STROKES.iter().enumerate() {
        // The site carries the accent on the second top stroke.
        let color: Color32 = if i == 1 { t.accent } else { t.text };
        let stroke = egui::Stroke::new(width, color);
        let (a, b) = (at(*x1, *y1), at(*x2, *y2));
        p.line_segment([a, b], stroke);
        // egui's line segments have square ends; a disc at each end gives the site's round caps
        // so the strokes meet without notches.
        p.circle_filled(a, width / 2.0, color);
        p.circle_filled(b, width / 2.0, color);
    }
    ui.ctx().data_mut(|d| d.insert_temp(rect_id(), r));
}

/// The rect the mark was last painted into.
#[cfg(test)]
pub fn mark_rect(ctx: &egui::Context) -> Option<Rect> {
    ctx.data(|d| d.get_temp(rect_id()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_mark_has_seven_strokes_and_stays_inside_its_rect() {
        assert_eq!(STROKES.len(), 7);
        for (x1, y1, x2, y2) in STROKES {
            assert!((0.0..=BOX.0).contains(&x1) && (0.0..=BOX.0).contains(&x2));
            assert!((0.0..=BOX.1).contains(&y1) && (0.0..=BOX.1).contains(&y2));
        }
    }

    #[test]
    fn the_title_bar_paints_the_mark() {
        let ctx = egui::Context::default();
        crate::PhotocraftApp::setup_context(&ctx, crate::theme::ThemeKind::ALL[0]);
        let mut app = crate::PhotocraftApp::new(photocraft_engine::Session::new(), crate::Services::default());
        // The system title bar hides the in-app mark (the OS draws its own icon), so paint the
        // app-drawn bar here.
        app.custom_titlebar = true;
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| crate::panels::title_bar(&mut app, ui));
        out.textures_delta.clear();
        assert!(mark_rect(&ctx).is_some(), "the brand mark is painted");
    }
}
