use anyhow::Result;
use crate::auth::UserSession;
use crate::client::JellyfinClient;
use crate::models::{MediaItem, Library, PlaybackState};

/// Main application service that coordinates other services
pub struct AppService {
    auth_service: AuthService,
    library_service: LibraryService,
    playback_service: PlaybackService,
    settings_service: SettingsService,
}

impl AppService {
    pub fn new() -> Self {
        Self {
            auth_service: AuthService::new(),
            library_service: LibraryService::new(),
            playback_service: PlaybackService::new(),
            settings_service: SettingsService::new(),
        }
    }

    pub fn auth(&mut self) -> &mut AuthService {
        &mut self.auth_service
    }

    pub fn library(&mut self) -> &mut LibraryService {
        &mut self.library_service
    }

    pub fn playback(&mut self) -> &mut PlaybackService {
        &mut self.playback_service
    }

    pub fn settings(&mut self) -> &mut SettingsService {
        &mut self.settings_service
    }
}

/// Authentication service for managing user sessions
pub struct AuthService {
    current_session: Option<UserSession>,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            current_session: None,
        }
    }

    pub fn current_session(&self) -> Option<&UserSession> {
        self.current_session.as_ref()
    }

    pub fn set_session(&mut self, session: UserSession) {
        self.current_session = Some(session);
    }

    pub fn clear_session(&mut self) {
        self.current_session = None;
    }

    pub fn is_authenticated(&self) -> bool {
        self.current_session.is_some()
    }
}

/// Library service for managing media content
pub struct LibraryService {
    current_library: Option<Library>,
    cached_items: Vec<MediaItem>,
}

impl LibraryService {
    pub fn new() -> Self {
        Self {
            current_library: None,
            cached_items: Vec::new(),
        }
    }

    pub async fn load_library(&mut self, _client: &JellyfinClient) -> Result<()> {
        // Library loading will be implemented in later tasks
        Ok(())
    }

    pub fn get_cached_items(&self) -> &[MediaItem] {
        &self.cached_items
    }

    pub fn clear_cache(&mut self) {
        self.cached_items.clear();
    }
}

/// Playback service for managing media playback
pub struct PlaybackService {
    current_state: PlaybackState,
    queue: Vec<MediaItem>,
    current_index: Option<usize>,
}

impl PlaybackService {
    pub fn new() -> Self {
        Self {
            current_state: PlaybackState::Stopped,
            queue: Vec::new(),
            current_index: None,
        }
    }

    pub fn current_state(&self) -> &PlaybackState {
        &self.current_state
    }

    pub fn play(&mut self, item: MediaItem) -> Result<()> {
        self.queue.clear();
        self.queue.push(item);
        self.current_index = Some(0);
        self.current_state = PlaybackState::Playing;
        Ok(())
    }

    pub fn pause(&mut self) {
        if self.current_state == PlaybackState::Playing {
            self.current_state = PlaybackState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if self.current_state == PlaybackState::Paused {
            self.current_state = PlaybackState::Playing;
        }
    }

    pub fn stop(&mut self) {
        self.current_state = PlaybackState::Stopped;
        self.current_index = None;
    }

    pub fn current_item(&self) -> Option<&MediaItem> {
        if let Some(index) = self.current_index {
            self.queue.get(index)
        } else {
            None
        }
    }
}

/// Settings service for managing application configuration
pub struct SettingsService {
    // Settings will be implemented in later tasks
}

impl SettingsService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn load_settings(&mut self) -> Result<()> {
        // Settings loading will be implemented in later tasks
        Ok(())
    }

    pub async fn save_settings(&self) -> Result<()> {
        // Settings saving will be implemented in later tasks
        Ok(())
    }
}