use crate::ui::theme::manager::ThemeManager;
use crate::ui::theme::{Color, ThemeAccess, ThemeContext, ThemeMode, ThemeSettings};
use gpui::prelude::*;
use gpui::InteractiveElement;
use gpui::*;

/// Main theme settings view component
pub struct ThemeSettingsView {
    /// Focus handle for the view
    focus_handle: FocusHandle,

    /// Current settings being edited (before apply)
    pending_settings: ThemeSettings,

    /// Whether there are unsaved changes
    has_changes: bool,

    /// Current color picker color
    picker_color: Color,

    /// Color picker hue (0-360)
    picker_hue: f32,

    /// Color picker saturation (0-1)
    picker_saturation: f32,

    /// Color picker brightness (0-1)
    picker_brightness: f32,
}

impl ThemeSettingsView {
    /// Create a new theme settings view
    pub fn new(cx: &mut Context<Self>) -> Self {
        let theme_context = cx.global::<ThemeContext>();
        let current_settings = theme_context.get_settings().clone();
        let initial_color = current_settings.custom_color.unwrap_or(Color::from_hex("#6750A4").unwrap());
        let (hue, saturation, brightness) = Self::color_to_hsv(initial_color);

        Self {
            focus_handle: cx.focus_handle(),
            pending_settings: current_settings,
            has_changes: false,
            picker_color: initial_color,
            picker_hue: hue,
            picker_saturation: saturation,
            picker_brightness: brightness,
        }
    }

    /// Convert color to HSV
    fn color_to_hsv(color: Color) -> (f32, f32, f32) {
        let (h, s, l) = color.to_hsl();
        // Convert HSL to HSV
        let v = l + s * l.min(1.0 - l);
        let s_new = if v == 0.0 { 0.0 } else { 2.0 * (1.0 - l / v) };
        (h, s_new, v)
    }

    /// Convert HSV to color
    fn hsv_to_color(h: f32, s: f32, v: f32) -> Color {
        // Convert HSV to HSL first
        let l = v * (1.0 - s / 2.0);
        let s_new = if l == 0.0 || l == 1.0 { 0.0 } else { (v - l) / l.min(1.0 - l) };
        Color::from_hsl(h, s_new, l)
    }

    /// Update picker color from HSV values
    fn update_picker_color(&mut self) {
        self.picker_color = Self::hsv_to_color(self.picker_hue, self.picker_saturation, self.picker_brightness);
        self.pending_settings.set_custom_color(self.picker_color);
        self.has_changes = true;
    }

    /// Handle dynamic theming toggle
    fn toggle_dynamic_theming(&mut self, cx: &mut Context<Self>) {
        self.pending_settings.set_dynamic_enabled(!self.pending_settings.dynamic_enabled);
        self.has_changes = true;
        cx.notify();
    }

    /// Handle theme mode change
    fn set_theme_mode(&mut self, mode: ThemeMode, cx: &mut Context<Self>) {
        self.pending_settings.set_theme_mode(mode);
        self.has_changes = true;
        cx.notify();
    }

    /// Apply pending changes
    fn apply_changes(&mut self, cx: &mut Context<Self>) {
        if !self.has_changes {
            return;
        }

        let settings = self.pending_settings.clone();

        // Update the theme manager with new settings
        if let Some(manager) = cx.try_global::<ThemeManager>() {
            let mut manager = manager.clone();
            let _ = manager.update_settings(settings, cx);
            cx.set_global(manager);
        }

        self.has_changes = false;
        cx.notify();
    }

    /// Cancel pending changes
    fn cancel_changes(&mut self, cx: &mut Context<Self>) {
        if !self.has_changes {
            return;
        }

        // Revert to current settings
        let theme_context = cx.global::<ThemeContext>();
        self.pending_settings = theme_context.get_settings().clone();

        // Update color picker
        if let Some(color) = self.pending_settings.custom_color {
            self.picker_color = color;
            let (hue, saturation, brightness) = Self::color_to_hsv(color);
            self.picker_hue = hue;
            self.picker_saturation = saturation;
            self.picker_brightness = brightness;
        }

        self.has_changes = false;
        cx.notify();
    }

    /// Reset to default settings
    fn reset_to_defaults(&mut self, cx: &mut Context<Self>) {
        self.pending_settings = ThemeSettings::default();
        let default_color = Color::from_hex("#6750A4").unwrap();
        self.picker_color = default_color;
        let (hue, saturation, brightness) = Self::color_to_hsv(default_color);
        self.picker_hue = hue;
        self.picker_saturation = saturation;
        self.picker_brightness = brightness;
        self.has_changes = true;
        cx.notify();
    }
}

impl Render for ThemeSettingsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_context = cx.global::<ThemeContext>();
        let surface_colors = theme_context.get_surface_colors();
        let text_colors = theme_context.get_text_colors();
        let accent_colors = theme_context.get_accent_colors();

        div()
            .id("theme-settings")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(ThemeAccess::to_hsla(surface_colors.surface))
            .text_color(ThemeAccess::to_hsla(text_colors.primary))
            .p_6()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(ThemeAccess::to_hsla(text_colors.primary))
                            .child("Theme Settings")
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .when(self.has_changes, |el| {
                                el.child(
                                    div()
                                        .px_3()
                                        .py_1()
                                        .bg(ThemeAccess::to_hsla(accent_colors.secondary))
                                        .text_color(ThemeAccess::to_hsla(text_colors.primary))
                                        .rounded_md()
                                        .text_sm()
                                        .child("Unsaved changes")
                                )
                            })
                    )
            )
            .child(
                // Main content area
                div()
                    .flex_1()
                    .flex()
                    .gap_8()
                    .child(
                        // Settings panel
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_6()
                            .child(self.render_theme_mode_section(cx))
                            .child(self.render_dynamic_theming_section(cx))
                            .child(self.render_color_picker_section(cx))
                            .child(self.render_animation_settings(cx))
                    )
                    .child(
                        // Preview panel
                        div()
                            .w(px(400.0))
                            .child(self.render_preview_panel(cx))
                    )
            )
            .child(
                // Action buttons
                div()
                    .flex()
                    .justify_between()
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .bg(ThemeAccess::to_hsla(surface_colors.surface_variant))
                            .text_color(ThemeAccess::to_hsla(text_colors.secondary))
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|style| style.bg(ThemeAccess::to_hsla(surface_colors.surface_container_high)))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _phase, cx| {
                                this.reset_to_defaults(cx);
                            }))
                            .child("Reset to Defaults")
                    )
                    .child(
                        div()
                            .flex()
                            .gap_3()
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .bg(ThemeAccess::to_hsla(surface_colors.surface_variant))
                                    .text_color(ThemeAccess::to_hsla(text_colors.secondary))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .hover(|style| style.bg(ThemeAccess::to_hsla(surface_colors.surface_container_high)))
                                    .when(self.has_changes, |el| {
                                        el.on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _phase, cx| {
                                            this.cancel_changes(cx);
                                        }))
                                    })
                                    .when(!self.has_changes, |el| {
                                        el.opacity(0.5)
                                    })
                                    .child("Cancel")
                            )
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .bg(ThemeAccess::to_hsla(accent_colors.primary))
                                    .text_color(ThemeAccess::to_hsla(text_colors.primary))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .hover(|style| style.opacity(0.9))
                                    .when(self.has_changes, |el| {
                                        el.on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _phase, cx| {
                                            this.apply_changes(cx);
                                        }))
                                    })
                                    .when(!self.has_changes, |el| {
                                        el.opacity(0.5)
                                    })
                                    .child("Apply")
                            )
                    )
            )
    }
}

impl ThemeSettingsView {
    /// Render theme mode selection section
    fn render_theme_mode_section(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_context = cx.global::<ThemeContext>();
        let _surface_colors = theme_context.get_surface_colors();
        let text_colors = theme_context.get_text_colors();
        let _accent_colors = theme_context.get_accent_colors();

        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(ThemeAccess::to_hsla(text_colors.primary))
                    .child("Appearance")
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(self.render_theme_mode_button("System", ThemeMode::System, cx))
                    .child(self.render_theme_mode_button("Light", ThemeMode::Light, cx))
                    .child(self.render_theme_mode_button("Dark", ThemeMode::Dark, cx))
            )
    }

    /// Render a theme mode button
    fn render_theme_mode_button(&mut self, label: &str, mode: ThemeMode, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_context = cx.global::<ThemeContext>();
        let surface_colors = theme_context.get_surface_colors();
        let text_colors = theme_context.get_text_colors();
        let accent_colors = theme_context.get_accent_colors();
        let is_selected = self.pending_settings.theme_mode == mode;

        div()
            .px_4()
            .py_2()
            .rounded_md()
            .cursor_pointer()
            .when(is_selected, |el| {
                el.bg(ThemeAccess::to_hsla(accent_colors.primary))
                    .text_color(ThemeAccess::to_hsla(text_colors.primary))
            })
            .when(!is_selected, |el| {
                el.bg(ThemeAccess::to_hsla(surface_colors.surface_variant))
                    .text_color(ThemeAccess::to_hsla(text_colors.secondary))
                    .hover(|style| style.bg(ThemeAccess::to_hsla(surface_colors.surface_container_high)))
            })
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _phase, cx| {
                this.set_theme_mode(mode, cx);
            }))
            .child(label.to_string())
    }

    /// Render dynamic theming section
    fn render_dynamic_theming_section(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_context = cx.global::<ThemeContext>();
        let surface_colors = theme_context.get_surface_colors();
        let text_colors = theme_context.get_text_colors();
        let _accent_colors = theme_context.get_accent_colors();

        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(ThemeAccess::to_hsla(text_colors.primary))
                    .child("Color Source")
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .bg(ThemeAccess::to_hsla(surface_colors.surface_container))
                    .rounded_lg()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(ThemeAccess::to_hsla(text_colors.primary))
                                    .child("Dynamic from Wallpaper")
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(ThemeAccess::to_hsla(text_colors.secondary))
                                    .child("Automatically adapt colors from your system wallpaper")
                            )
                    )
                    .child(self.render_toggle_switch(self.pending_settings.dynamic_enabled, cx))
            )
            .when(!self.pending_settings.dynamic_enabled, |el| {
                el.child(
                    div()
                        .text_sm()
                        .text_color(ThemeAccess::to_hsla(text_colors.secondary))
                        .child("When disabled, you can select a custom color below")
                )
            })
    }

    /// Render toggle switch
    fn render_toggle_switch(&mut self, enabled: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_context = cx.global::<ThemeContext>();
        let surface_colors = theme_context.get_surface_colors();
        let text_colors = theme_context.get_text_colors();
        let accent_colors = theme_context.get_accent_colors();

        div()
            .w(px(48.0))
            .h(px(24.0))
            .bg(if enabled {
                ThemeAccess::to_hsla(accent_colors.primary)
            } else {
                ThemeAccess::to_hsla(surface_colors.outline)
            })
            .rounded_full()
            .cursor_pointer()
            .flex()
            .items_center()
            .px_1()
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _phase, cx| {
                this.toggle_dynamic_theming(cx);
            }))
            .child(
                div()
                    .w(px(20.0))
                    .h(px(20.0))
                    .bg(if enabled {
                        ThemeAccess::to_hsla(text_colors.primary)
                    } else {
                        ThemeAccess::to_hsla(text_colors.secondary)
                    })
                    .rounded_full()
                    .when(enabled, |el| {
                        el.ml_auto()
                    })
            )
    }

    /// Render color picker section
    fn render_color_picker_section(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_context = cx.global::<ThemeContext>();
        let text_colors = theme_context.get_text_colors();
        let surface_colors = theme_context.get_surface_colors();

        div()
            .flex()
            .flex_col()
            .gap_3()
            .when(!self.pending_settings.dynamic_enabled, |el| {
                el.child(
                    div()
                        .text_lg()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(ThemeAccess::to_hsla(text_colors.primary))
                        .child("Custom Color")
                )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            .child(
                                // Color preview
                                div()
                                    .w(px(64.0))
                                    .h(px(64.0))
                                    .bg(ThemeAccess::to_hsla(self.picker_color))
                                    .rounded_lg()
                                    .border_2()
                                    .border_color(ThemeAccess::to_hsla(surface_colors.outline))
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(ThemeAccess::to_hsla(text_colors.primary))
                                            .child("Selected Color")
                                    )
                                    .child(
                                        div()
                                            .px_3()
                                            .py_2()
                                            .bg(ThemeAccess::to_hsla(surface_colors.surface_variant))
                                            .text_color(ThemeAccess::to_hsla(text_colors.primary))
                                            .rounded_md()
                                            .font_family("monospace")
                                            .child(self.picker_color.to_hex())
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(ThemeAccess::to_hsla(text_colors.secondary))
                                            .child("Click to change color (simplified picker)")
                                    )
                            )
                    )
            })
    }

    /// Render animation settings
    fn render_animation_settings(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_context = cx.global::<ThemeContext>();
        let surface_colors = theme_context.get_surface_colors();
        let text_colors = theme_context.get_text_colors();

        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(ThemeAccess::to_hsla(text_colors.primary))
                    .child("Animations")
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .bg(ThemeAccess::to_hsla(surface_colors.surface_container))
                    .rounded_lg()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(ThemeAccess::to_hsla(text_colors.primary))
                                    .child("Theme Transitions")
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(ThemeAccess::to_hsla(text_colors.secondary))
                                    .child("Smooth animations when theme colors change")
                            )
                    )
                    .child(
                        div()
                            .w(px(48.0))
                            .h(px(24.0))
                            .bg(if self.pending_settings.transition_animations {
                                ThemeAccess::to_hsla(theme_context.get_accent_colors().primary)
                            } else {
                                ThemeAccess::to_hsla(surface_colors.outline)
                            })
                            .rounded_full()
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .px_1()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _phase, cx| {
                                this.pending_settings.set_animations_enabled(!this.pending_settings.transition_animations);
                                this.has_changes = true;
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .w(px(20.0))
                                    .h(px(20.0))
                                    .bg(if self.pending_settings.transition_animations {
                                        ThemeAccess::to_hsla(text_colors.primary)
                                    } else {
                                        ThemeAccess::to_hsla(text_colors.secondary)
                                    })
                                    .rounded_full()
                                    .when(self.pending_settings.transition_animations, |el| {
                                        el.ml_auto()
                                    })
                            )
                    )
            )
    }

    /// Render preview panel
    fn render_preview_panel(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_context = cx.global::<ThemeContext>();
        let surface_colors = theme_context.get_surface_colors();
        let text_colors = theme_context.get_text_colors();
        let accent_colors = theme_context.get_accent_colors();

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(ThemeAccess::to_hsla(text_colors.primary))
                    .child("Preview")
            )
            .child(
                // Preview container
                div()
                    .p_4()
                    .bg(ThemeAccess::to_hsla(surface_colors.surface_container))
                    .rounded_lg()
                    .border_1()
                    .border_color(ThemeAccess::to_hsla(surface_colors.outline))
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        // Sample header
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .p_3()
                            .bg(ThemeAccess::to_hsla(surface_colors.surface_container_high))
                            .rounded_md()
                            .child(
                                div()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(ThemeAccess::to_hsla(text_colors.primary))
                                    .child("Sample Header")
                            )
                            .child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .bg(ThemeAccess::to_hsla(accent_colors.primary))
                                    .text_color(ThemeAccess::to_hsla(text_colors.primary))
                                    .rounded_md()
                                    .text_sm()
                                    .child("Action")
                            )
                    )
                    .child(
                        // Sample content
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(ThemeAccess::to_hsla(text_colors.primary))
                                    .child("This is how your content will look with the selected theme.")
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(ThemeAccess::to_hsla(text_colors.secondary))
                                    .child("Secondary text appears in a muted color for better hierarchy.")
                            )
                    )
                    .child(
                        // Sample buttons
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .bg(ThemeAccess::to_hsla(accent_colors.primary))
                                    .text_color(ThemeAccess::to_hsla(text_colors.primary))
                                    .rounded_md()
                                    .child("Primary")
                            )
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .bg(ThemeAccess::to_hsla(accent_colors.secondary))
                                    .text_color(ThemeAccess::to_hsla(text_colors.primary))
                                    .rounded_md()
                                    .child("Secondary")
                            )
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .bg(ThemeAccess::to_hsla(surface_colors.surface_variant))
                                    .text_color(ThemeAccess::to_hsla(text_colors.secondary))
                                    .rounded_md()
                                    .border_1()
                                    .border_color(ThemeAccess::to_hsla(surface_colors.outline))
                                    .child("Outlined")
                            )
                    )
                    .child(
                        // Mode indicator
                        div()
                            .mt_2()
                            .pt_2()
                            .border_t_1()
                            .border_color(ThemeAccess::to_hsla(surface_colors.outline_variant))
                            .text_xs()
                            .text_color(ThemeAccess::to_hsla(text_colors.secondary))
                            .child(format!(
                                "Preview Mode: {} ({})",
                                match self.pending_settings.theme_mode {
                                    ThemeMode::Light => "Light",
                                    ThemeMode::Dark => "Dark",
                                    ThemeMode::System => "System",
                                },
                                if theme_context.is_dark_mode() { "Dark" } else { "Light" }
                            ))
                    )
            )
    }
}

impl Focusable for ThemeSettingsView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

/// Simple color picker component (simplified for this implementation)
pub struct ColorPicker {
    /// Current selected color
    selected_color: Color,

    /// Focus handle
    focus_handle: FocusHandle,
}

impl ColorPicker {
    /// Create a new color picker with initial color
    pub fn new(initial_color: Color, cx: &mut Context<Self>) -> Self {
        Self {
            selected_color: initial_color,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Set the color picker to a specific color
    pub fn set_color(&mut self, color: Color) {
        self.selected_color = color;
    }

    /// Get the currently selected color
    pub fn get_color(&self) -> Color {
        self.selected_color
    }
}

impl Render for ColorPicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_context = cx.global::<ThemeContext>();
        let surface_colors = theme_context.get_surface_colors();
        let text_colors = theme_context.get_text_colors();

        div()
            .track_focus(&self.focus_handle)
            .flex()
            .items_center()
            .gap_4()
            .child(
                // Color preview
                div()
                    .w(px(48.0))
                    .h(px(48.0))
                    .bg(ThemeAccess::to_hsla(self.selected_color))
                    .rounded_lg()
                    .border_1()
                    .border_color(ThemeAccess::to_hsla(surface_colors.outline))
            )
            .child(
                // Hex input (read-only for now)
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(ThemeAccess::to_hsla(text_colors.primary))
                            .child("Hex Color")
                    )
                    .child(
                        div()
                            .px_3()
                            .py_2()
                            .bg(ThemeAccess::to_hsla(surface_colors.surface_variant))
                            .text_color(ThemeAccess::to_hsla(text_colors.primary))
                            .rounded_md()
                            .font_family("monospace")
                            .child(self.selected_color.to_hex())
                    )
            )
    }
}

impl Focusable for ColorPicker {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

/// Theme preview component showing sample UI elements
pub struct ThemePreview {
    /// Focus handle
    focus_handle: FocusHandle,
}

impl ThemePreview {
    /// Create a new theme preview
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for ThemePreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_context = cx.global::<ThemeContext>();
        let surface_colors = theme_context.get_surface_colors();
        let text_colors = theme_context.get_text_colors();
        let accent_colors = theme_context.get_accent_colors();

        div()
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(ThemeAccess::to_hsla(text_colors.primary))
                    .child("Live Preview")
            )
            .child(
                // Preview container with sample components
                div()
                    .p_4()
                    .bg(ThemeAccess::to_hsla(surface_colors.surface_container))
                    .rounded_lg()
                    .border_1()
                    .border_color(ThemeAccess::to_hsla(surface_colors.outline))
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .text_color(ThemeAccess::to_hsla(text_colors.primary))
                            .child("Sample content with current theme colors")
                    )
                    .child(
                        div()
                            .px_3()
                            .py_2()
                            .bg(ThemeAccess::to_hsla(accent_colors.primary))
                            .text_color(ThemeAccess::to_hsla(text_colors.primary))
                            .rounded_md()
                            .child("Primary Button")
                    )
            )
    }
}

impl Focusable for ThemePreview {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}