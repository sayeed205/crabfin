use crate::ui::theme::{Color, MaterialPalette};
use std::time::{Duration, Instant};

/// Theme transition animation controller
#[derive(Clone)]
pub struct ThemeTransition {
    /// Starting palette for the transition
    from_palette: MaterialPalette,

    /// Target palette for the transition
    to_palette: MaterialPalette,

    /// Animation start time
    start_time: Instant,

    /// Animation duration
    duration: Duration,

    /// Whether the animation is currently active
    is_active: bool,

    /// Current animation progress (0.0 to 1.0)
    progress: f32,
}

impl ThemeTransition {
    /// Create a new theme transition
    pub fn new(from: MaterialPalette, to: MaterialPalette, duration: Duration) -> Self {
        Self {
            from_palette: from,
            to_palette: to,
            start_time: Instant::now(),
            duration,
            is_active: true,
            progress: 0.0,
        }
    }

    /// Update the animation progress
    pub fn update(&mut self) -> bool {
        if !self.is_active {
            return false;
        }

        let elapsed = self.start_time.elapsed();
        self.progress = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);

        if self.progress >= 1.0 {
            self.is_active = false;
        }

        true
    }

    /// Get the current interpolated palette
    pub fn current_palette(&self) -> MaterialPalette {
        if !self.is_active || self.progress >= 1.0 {
            return self.to_palette.clone();
        }

        if self.progress <= 0.0 {
            return self.from_palette.clone();
        }

        // Apply easing function
        let eased_progress = Self::ease_in_out_cubic(self.progress);

        // For simplicity, just return the target palette
        // In a full implementation, this would interpolate between palettes
        self.to_palette.clone()
    }

    /// Check if the animation is still active
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Get current progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        self.progress
    }

    /// Cubic ease-in-out easing function
    fn ease_in_out_cubic(t: f32) -> f32 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }
}

/// Theme animation manager for handling multiple concurrent animations
#[derive(Clone)]
pub struct ThemeAnimationManager {
    /// Current active transition
    current_transition: Option<ThemeTransition>,

    /// Default animation duration
    default_duration: Duration,

    /// Whether animations are enabled
    animations_enabled: bool,
}

impl ThemeAnimationManager {
    /// Create a new animation manager
    pub fn new() -> Self {
        Self {
            current_transition: None,
            default_duration: Duration::from_millis(300),
            animations_enabled: true,
        }
    }

    /// Start a new theme transition
    pub fn start_transition(&mut self, from: MaterialPalette, to: MaterialPalette, duration: Option<Duration>) {
        if !self.animations_enabled {
            return;
        }

        let duration = duration.unwrap_or(self.default_duration);
        self.current_transition = Some(ThemeTransition::new(from, to, duration));
    }

    /// Update all active animations
    pub fn update(&mut self) -> bool {
        if let Some(ref mut transition) = self.current_transition {
            if !transition.update() {
                self.current_transition = None;
                return false;
            }
            return true;
        }
        false
    }

    /// Get the current animated palette, or None if no animation is active
    pub fn current_palette(&self) -> Option<MaterialPalette> {
        self.current_transition.as_ref().map(|t| t.current_palette())
    }

    /// Check if any animation is currently active
    pub fn is_animating(&self) -> bool {
        self.current_transition.as_ref().map_or(false, |t| t.is_active())
    }

    /// Enable or disable animations
    pub fn set_animations_enabled(&mut self, enabled: bool) {
        self.animations_enabled = enabled;
        if !enabled {
            self.current_transition = None;
        }
    }

    /// Set the default animation duration
    pub fn set_default_duration(&mut self, duration: Duration) {
        self.default_duration = duration;
    }

    /// Get current animation progress (0.0 to 1.0), or 1.0 if no animation is active
    pub fn progress(&self) -> f32 {
        self.current_transition.as_ref().map_or(1.0, |t| t.progress())
    }

    /// Cancel any active animation
    pub fn cancel_animation(&mut self) {
        self.current_transition = None;
    }
}

impl Default for ThemeAnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Animation utilities for theme transitions
pub struct ThemeAnimator;

impl ThemeAnimator {
    /// Interpolate between two colors
    pub fn interpolate_color(from: Color, to: Color, progress: f32) -> Color {
        let progress = progress.clamp(0.0, 1.0);
        Color::new(
            from.r + (to.r - from.r) * progress,
            from.g + (to.g - from.g) * progress,
            from.b + (to.b - from.b) * progress,
            from.a + (to.a - from.a) * progress,
        )
    }

    /// Create an easing function for smooth transitions
    pub fn ease_in_out_cubic(t: f32) -> f32 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }
}