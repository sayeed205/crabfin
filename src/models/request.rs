use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Sort order for query results
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::Ascending
    }
}

impl ToString for SortOrder {
    fn to_string(&self) -> String {
        match self {
            SortOrder::Ascending => "Ascending".to_string(),
            SortOrder::Descending => "Descending".to_string(),
        }
    }
}

/// Query parameters for fetching items from Jellyfin API
#[derive(Debug, Clone, Default)]
pub struct ItemsQuery {
    pub parent_id: Option<String>,
    pub include_item_types: Vec<String>,
    pub exclude_item_types: Vec<String>,
    pub recursive: bool,
    pub sort_by: Vec<String>,
    pub sort_order: SortOrder,
    pub limit: Option<u32>,
    pub start_index: Option<u32>,
    pub search_term: Option<String>,
    pub filters: Vec<String>,
    pub fields: Vec<String>,
    pub enable_images: Option<bool>,
    pub image_type_limit: Option<u32>,
    pub enable_image_types: Vec<String>,
    pub enable_user_data: Option<bool>,
    pub enable_total_record_count: Option<bool>,
    pub years: Vec<i32>,
    pub genres: Vec<String>,
    pub official_ratings: Vec<String>,
    pub tags: Vec<String>,
    pub studios: Vec<String>,
    pub artists: Vec<String>,
    pub albums: Vec<String>,
    pub person_ids: Vec<String>,
    pub person_types: Vec<String>,
    pub is_hd: Option<bool>,
    pub is_4k: Option<bool>,
    pub location_types: Vec<String>,
    pub exclude_location_types: Vec<String>,
    pub is_missing: Option<bool>,
    pub is_unaired: Option<bool>,
    pub min_community_rating: Option<f32>,
    pub min_critic_rating: Option<f32>,
    pub min_premiere_date: Option<String>,
    pub min_date_last_saved: Option<String>,
    pub min_date_last_saved_for_user: Option<String>,
    pub max_premiere_date: Option<String>,
    pub has_overview: Option<bool>,
    pub has_imdb_id: Option<bool>,
    pub has_tmdb_id: Option<bool>,
    pub has_tvdb_id: Option<bool>,
    pub exclude_item_ids: Vec<String>,
    pub adjacent_to: Option<String>,
    pub min_index_number: Option<i32>,
    pub min_player_count: Option<i32>,
    pub max_player_count: Option<i32>,
    pub parent_index_number: Option<i32>,
    pub has_parental_rating: Option<bool>,
    pub is_hd_or_higher: Option<bool>,
    pub is_locked: Option<bool>,
    pub has_chapters: Option<bool>,
    pub has_subtitles: Option<bool>,
    pub has_special_feature: Option<bool>,
    pub has_trailer: Option<bool>,
    pub has_theme_song: Option<bool>,
    pub has_theme_video: Option<bool>,
    pub series_status: Vec<String>,
    pub name_starts_with_or_greater: Option<String>,
    pub name_starts_with: Option<String>,
    pub name_less_than: Option<String>,
    pub studio_ids: Vec<String>,
    pub genre_ids: Vec<String>,
    pub enable_total_record_count_bool: Option<bool>,
    pub enable_images_bool: Option<bool>,
}

impl ItemsQuery {
    /// Create a new ItemsQuery builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the parent ID to filter items
    pub fn parent_id<S: Into<String>>(mut self, parent_id: S) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    /// Include specific item types
    pub fn include_item_types(mut self, types: Vec<String>) -> Self {
        self.include_item_types = types;
        self
    }

    /// Add a single item type to include
    pub fn include_item_type<S: Into<String>>(mut self, item_type: S) -> Self {
        self.include_item_types.push(item_type.into());
        self
    }

    /// Exclude specific item types
    pub fn exclude_item_types(mut self, types: Vec<String>) -> Self {
        self.exclude_item_types = types;
        self
    }

    /// Add a single item type to exclude
    pub fn exclude_item_type<S: Into<String>>(mut self, item_type: S) -> Self {
        self.exclude_item_types.push(item_type.into());
        self
    }

    /// Set recursive search
    pub fn recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }

    /// Set sort fields
    pub fn sort_by(mut self, fields: Vec<String>) -> Self {
        self.sort_by = fields;
        self
    }

    /// Add a single sort field
    pub fn add_sort_by<S: Into<String>>(mut self, field: S) -> Self {
        self.sort_by.push(field.into());
        self
    }

    /// Set sort order
    pub fn sort_order(mut self, order: SortOrder) -> Self {
        self.sort_order = order;
        self
    }

    /// Set result limit
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set start index for pagination
    pub fn start_index(mut self, index: u32) -> Self {
        self.start_index = Some(index);
        self
    }

    /// Set search term
    pub fn search_term<S: Into<String>>(mut self, term: S) -> Self {
        self.search_term = Some(term.into());
        self
    }

    /// Set filters
    pub fn filters(mut self, filters: Vec<String>) -> Self {
        self.filters = filters;
        self
    }

    /// Add a single filter
    pub fn add_filter<S: Into<String>>(mut self, filter: S) -> Self {
        self.filters.push(filter.into());
        self
    }

    /// Set fields to include in response
    pub fn fields(mut self, fields: Vec<String>) -> Self {
        self.fields = fields;
        self
    }

    /// Add a single field to include
    pub fn add_field<S: Into<String>>(mut self, field: S) -> Self {
        self.fields.push(field.into());
        self
    }

    /// Enable or disable images in response
    pub fn enable_images(mut self, enable: bool) -> Self {
        self.enable_images = Some(enable);
        self
    }

    /// Set image type limit
    pub fn image_type_limit(mut self, limit: u32) -> Self {
        self.image_type_limit = Some(limit);
        self
    }

    /// Enable specific image types
    pub fn enable_image_types(mut self, types: Vec<String>) -> Self {
        self.enable_image_types = types;
        self
    }

    /// Enable or disable user data in response
    pub fn enable_user_data(mut self, enable: bool) -> Self {
        self.enable_user_data = Some(enable);
        self
    }

    /// Enable or disable total record count
    pub fn enable_total_record_count(mut self, enable: bool) -> Self {
        self.enable_total_record_count = Some(enable);
        self
    }

    /// Filter by years
    pub fn years(mut self, years: Vec<i32>) -> Self {
        self.years = years;
        self
    }

    /// Add a single year filter
    pub fn add_year(mut self, year: i32) -> Self {
        self.years.push(year);
        self
    }

    /// Filter by genres
    pub fn genres(mut self, genres: Vec<String>) -> Self {
        self.genres = genres;
        self
    }

    /// Add a single genre filter
    pub fn add_genre<S: Into<String>>(mut self, genre: S) -> Self {
        self.genres.push(genre.into());
        self
    }

    /// Filter by official ratings
    pub fn official_ratings(mut self, ratings: Vec<String>) -> Self {
        self.official_ratings = ratings;
        self
    }

    /// Filter by tags
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Filter by studios
    pub fn studios(mut self, studios: Vec<String>) -> Self {
        self.studios = studios;
        self
    }

    /// Filter by artists
    pub fn artists(mut self, artists: Vec<String>) -> Self {
        self.artists = artists;
        self
    }

    /// Filter by albums
    pub fn albums(mut self, albums: Vec<String>) -> Self {
        self.albums = albums;
        self
    }

    /// Filter by HD content
    pub fn is_hd(mut self, is_hd: bool) -> Self {
        self.is_hd = Some(is_hd);
        self
    }

    /// Filter by 4K content
    pub fn is_4k(mut self, is_4k: bool) -> Self {
        self.is_4k = Some(is_4k);
        self
    }

    /// Set minimum community rating
    pub fn min_community_rating(mut self, rating: f32) -> Self {
        self.min_community_rating = Some(rating);
        self
    }

    /// Set minimum critic rating
    pub fn min_critic_rating(mut self, rating: f32) -> Self {
        self.min_critic_rating = Some(rating);
        self
    }

    /// Validate the query parameters
    pub fn validate(&self) -> Result<(), String> {
        if let Some(limit) = self.limit {
            if limit == 0 {
                return Err("Limit must be greater than 0".to_string());
            }
            if limit > 10000 {
                return Err("Limit cannot exceed 10000".to_string());
            }
        }

        if let Some(rating) = self.min_community_rating {
            if rating < 0.0 || rating > 10.0 {
                return Err("Community rating must be between 0.0 and 10.0".to_string());
            }
        }

        if let Some(rating) = self.min_critic_rating {
            if rating < 0.0 || rating > 100.0 {
                return Err("Critic rating must be between 0.0 and 100.0".to_string());
            }
        }

        Ok(())
    }

    /// Convert to query parameters for HTTP request
    pub fn to_query_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();

        if let Some(ref parent_id) = self.parent_id {
            params.insert("ParentId".to_string(), parent_id.clone());
        }

        if !self.include_item_types.is_empty() {
            params.insert("IncludeItemTypes".to_string(), self.include_item_types.join(","));
        }

        if !self.exclude_item_types.is_empty() {
            params.insert("ExcludeItemTypes".to_string(), self.exclude_item_types.join(","));
        }

        if self.recursive {
            params.insert("Recursive".to_string(), "true".to_string());
        }

        if !self.sort_by.is_empty() {
            params.insert("SortBy".to_string(), self.sort_by.join(","));
        }

        params.insert("SortOrder".to_string(), self.sort_order.to_string());

        if let Some(limit) = self.limit {
            params.insert("Limit".to_string(), limit.to_string());
        }

        if let Some(start_index) = self.start_index {
            params.insert("StartIndex".to_string(), start_index.to_string());
        }

        if let Some(ref search_term) = self.search_term {
            params.insert("SearchTerm".to_string(), search_term.clone());
        }

        if !self.filters.is_empty() {
            params.insert("Filters".to_string(), self.filters.join(","));
        }

        if !self.fields.is_empty() {
            params.insert("Fields".to_string(), self.fields.join(","));
        }

        if let Some(enable_images) = self.enable_images {
            params.insert("EnableImages".to_string(), enable_images.to_string());
        }

        if let Some(image_type_limit) = self.image_type_limit {
            params.insert("ImageTypeLimit".to_string(), image_type_limit.to_string());
        }

        if !self.enable_image_types.is_empty() {
            params.insert("EnableImageTypes".to_string(), self.enable_image_types.join(","));
        }

        if let Some(enable_user_data) = self.enable_user_data {
            params.insert("EnableUserData".to_string(), enable_user_data.to_string());
        }

        if let Some(enable_total_record_count) = self.enable_total_record_count {
            params.insert("EnableTotalRecordCount".to_string(), enable_total_record_count.to_string());
        }

        if !self.years.is_empty() {
            let years_str: Vec<String> = self.years.iter().map(|y| y.to_string()).collect();
            params.insert("Years".to_string(), years_str.join(","));
        }

        if !self.genres.is_empty() {
            params.insert("Genres".to_string(), self.genres.join(","));
        }

        if !self.official_ratings.is_empty() {
            params.insert("OfficialRatings".to_string(), self.official_ratings.join(","));
        }

        if !self.tags.is_empty() {
            params.insert("Tags".to_string(), self.tags.join(","));
        }

        if !self.studios.is_empty() {
            params.insert("Studios".to_string(), self.studios.join(","));
        }

        if !self.artists.is_empty() {
            params.insert("Artists".to_string(), self.artists.join(","));
        }

        if !self.albums.is_empty() {
            params.insert("Albums".to_string(), self.albums.join(","));
        }

        if let Some(is_hd) = self.is_hd {
            params.insert("IsHD".to_string(), is_hd.to_string());
        }

        if let Some(is_4k) = self.is_4k {
            params.insert("Is4K".to_string(), is_4k.to_string());
        }

        if let Some(rating) = self.min_community_rating {
            params.insert("MinCommunityRating".to_string(), rating.to_string());
        }

        if let Some(rating) = self.min_critic_rating {
            params.insert("MinCriticRating".to_string(), rating.to_string());
        }

        params
    }
}

/// Parameters for streaming media content
#[derive(Debug, Clone, Default)]
pub struct StreamParams {
    pub max_streaming_bitrate: Option<u32>,
    pub max_audio_channels: Option<u32>,
    pub container: Option<String>,
    pub audio_codec: Option<String>,
    pub video_codec: Option<String>,
    pub audio_sample_rate: Option<u32>,
    pub max_audio_bitrate: Option<u32>,
    pub max_video_bitrate: Option<u32>,
    pub profile: Option<String>,
    pub level: Option<String>,
    pub framerate: Option<f32>,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub start_time_ticks: Option<i64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub video_bitrate: Option<u32>,
    pub audio_bitrate: Option<u32>,
    pub audio_channels: Option<u32>,
    pub device_id: Option<String>,
    pub play_session_id: Option<String>,
    pub live_stream_id: Option<String>,
    pub media_source_id: Option<String>,
    pub subtitle_stream_index: Option<i32>,
    pub audio_stream_index: Option<i32>,
    pub video_stream_index: Option<i32>,
    pub segment_container: Option<String>,
    pub segment_length: Option<u32>,
    pub min_segments: Option<u32>,
    pub break_on_non_key_frames: Option<bool>,
    pub require_avc: Option<bool>,
    pub de_interlace: Option<bool>,
    pub require_non_anamorphic: Option<bool>,
    pub transcoding_max_audio_channels: Option<u32>,
    pub cpu_core_limit: Option<u32>,
    pub enable_mpegts_m2ts_mode: Option<bool>,
    pub video_range: Option<String>,
    pub enable_subtitles_in_manifest: Option<bool>,
}

impl StreamParams {
    /// Create a new StreamParams builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum streaming bitrate
    pub fn max_streaming_bitrate(mut self, bitrate: u32) -> Self {
        self.max_streaming_bitrate = Some(bitrate);
        self
    }

    /// Set maximum audio channels
    pub fn max_audio_channels(mut self, channels: u32) -> Self {
        self.max_audio_channels = Some(channels);
        self
    }

    /// Set container format
    pub fn container<S: Into<String>>(mut self, container: S) -> Self {
        self.container = Some(container.into());
        self
    }

    /// Set audio codec
    pub fn audio_codec<S: Into<String>>(mut self, codec: S) -> Self {
        self.audio_codec = Some(codec.into());
        self
    }

    /// Set video codec
    pub fn video_codec<S: Into<String>>(mut self, codec: S) -> Self {
        self.video_codec = Some(codec.into());
        self
    }

    /// Set audio sample rate
    pub fn audio_sample_rate(mut self, rate: u32) -> Self {
        self.audio_sample_rate = Some(rate);
        self
    }

    /// Set maximum audio bitrate
    pub fn max_audio_bitrate(mut self, bitrate: u32) -> Self {
        self.max_audio_bitrate = Some(bitrate);
        self
    }

    /// Set maximum video bitrate
    pub fn max_video_bitrate(mut self, bitrate: u32) -> Self {
        self.max_video_bitrate = Some(bitrate);
        self
    }

    /// Set video profile
    pub fn profile<S: Into<String>>(mut self, profile: S) -> Self {
        self.profile = Some(profile.into());
        self
    }

    /// Set video level
    pub fn level<S: Into<String>>(mut self, level: S) -> Self {
        self.level = Some(level.into());
        self
    }

    /// Set framerate
    pub fn framerate(mut self, framerate: f32) -> Self {
        self.framerate = Some(framerate);
        self
    }

    /// Set maximum width
    pub fn max_width(mut self, width: u32) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Set maximum height
    pub fn max_height(mut self, height: u32) -> Self {
        self.max_height = Some(height);
        self
    }

    /// Set start time in ticks
    pub fn start_time_ticks(mut self, ticks: i64) -> Self {
        self.start_time_ticks = Some(ticks);
        self
    }

    /// Set device ID
    pub fn device_id<S: Into<String>>(mut self, device_id: S) -> Self {
        self.device_id = Some(device_id.into());
        self
    }

    /// Set play session ID
    pub fn play_session_id<S: Into<String>>(mut self, session_id: S) -> Self {
        self.play_session_id = Some(session_id.into());
        self
    }

    /// Set media source ID
    pub fn media_source_id<S: Into<String>>(mut self, source_id: S) -> Self {
        self.media_source_id = Some(source_id.into());
        self
    }

    /// Set subtitle stream index
    pub fn subtitle_stream_index(mut self, index: i32) -> Self {
        self.subtitle_stream_index = Some(index);
        self
    }

    /// Set audio stream index
    pub fn audio_stream_index(mut self, index: i32) -> Self {
        self.audio_stream_index = Some(index);
        self
    }

    /// Set video stream index
    pub fn video_stream_index(mut self, index: i32) -> Self {
        self.video_stream_index = Some(index);
        self
    }

    /// Validate the stream parameters
    pub fn validate(&self) -> Result<(), String> {
        if let Some(bitrate) = self.max_streaming_bitrate {
            if bitrate == 0 {
                return Err("Max streaming bitrate must be greater than 0".to_string());
            }
        }

        if let Some(channels) = self.max_audio_channels {
            if channels == 0 || channels > 8 {
                return Err("Max audio channels must be between 1 and 8".to_string());
            }
        }

        if let Some(framerate) = self.framerate {
            if framerate <= 0.0 || framerate > 120.0 {
                return Err("Framerate must be between 0.0 and 120.0".to_string());
            }
        }

        if let Some(width) = self.max_width {
            if width == 0 || width > 7680 {
                return Err("Max width must be between 1 and 7680".to_string());
            }
        }

        if let Some(height) = self.max_height {
            if height == 0 || height > 4320 {
                return Err("Max height must be between 1 and 4320".to_string());
            }
        }

        Ok(())
    }

    /// Convert to query parameters for HTTP request
    pub fn to_query_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();

        if let Some(bitrate) = self.max_streaming_bitrate {
            params.insert("MaxStreamingBitrate".to_string(), bitrate.to_string());
        }

        if let Some(channels) = self.max_audio_channels {
            params.insert("MaxAudioChannels".to_string(), channels.to_string());
        }

        if let Some(ref container) = self.container {
            params.insert("Container".to_string(), container.clone());
        }

        if let Some(ref codec) = self.audio_codec {
            params.insert("AudioCodec".to_string(), codec.clone());
        }

        if let Some(ref codec) = self.video_codec {
            params.insert("VideoCodec".to_string(), codec.clone());
        }

        if let Some(rate) = self.audio_sample_rate {
            params.insert("AudioSampleRate".to_string(), rate.to_string());
        }

        if let Some(bitrate) = self.max_audio_bitrate {
            params.insert("MaxAudioBitrate".to_string(), bitrate.to_string());
        }

        if let Some(bitrate) = self.max_video_bitrate {
            params.insert("MaxVideoBitrate".to_string(), bitrate.to_string());
        }

        if let Some(ref profile) = self.profile {
            params.insert("Profile".to_string(), profile.clone());
        }

        if let Some(ref level) = self.level {
            params.insert("Level".to_string(), level.clone());
        }

        if let Some(framerate) = self.framerate {
            params.insert("Framerate".to_string(), framerate.to_string());
        }

        if let Some(width) = self.max_width {
            params.insert("MaxWidth".to_string(), width.to_string());
        }

        if let Some(height) = self.max_height {
            params.insert("MaxHeight".to_string(), height.to_string());
        }

        if let Some(ticks) = self.start_time_ticks {
            params.insert("StartTimeTicks".to_string(), ticks.to_string());
        }

        if let Some(ref device_id) = self.device_id {
            params.insert("DeviceId".to_string(), device_id.clone());
        }

        if let Some(ref session_id) = self.play_session_id {
            params.insert("PlaySessionId".to_string(), session_id.clone());
        }

        if let Some(ref source_id) = self.media_source_id {
            params.insert("MediaSourceId".to_string(), source_id.clone());
        }

        if let Some(index) = self.subtitle_stream_index {
            params.insert("SubtitleStreamIndex".to_string(), index.to_string());
        }

        if let Some(index) = self.audio_stream_index {
            params.insert("AudioStreamIndex".to_string(), index.to_string());
        }

        if let Some(index) = self.video_stream_index {
            params.insert("VideoStreamIndex".to_string(), index.to_string());
        }

        params
    }
}

/// Parameters for authentication requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    #[serde(rename = "Username")]
    pub username: String,
    #[serde(rename = "Pw")]
    pub password: String,
}

impl AuthRequest {
    pub fn new<S: Into<String>>(username: S, password: S) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
}

/// Parameters for playback reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackStartInfo {
    #[serde(rename = "ItemId")]
    pub item_id: String,
    #[serde(rename = "SessionId")]
    pub session_id: Option<String>,
    #[serde(rename = "MediaSourceId")]
    pub media_source_id: Option<String>,
    #[serde(rename = "AudioStreamIndex")]
    pub audio_stream_index: Option<i32>,
    #[serde(rename = "SubtitleStreamIndex")]
    pub subtitle_stream_index: Option<i32>,
    #[serde(rename = "PlayMethod")]
    pub play_method: String,
    #[serde(rename = "LiveStreamId")]
    pub live_stream_id: Option<String>,
    #[serde(rename = "PlaySessionId")]
    pub play_session_id: Option<String>,
    #[serde(rename = "CanSeek")]
    pub can_seek: bool,
}

/// Parameters for playback progress reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackProgressInfo {
    #[serde(rename = "ItemId")]
    pub item_id: String,
    #[serde(rename = "SessionId")]
    pub session_id: Option<String>,
    #[serde(rename = "MediaSourceId")]
    pub media_source_id: Option<String>,
    #[serde(rename = "PositionTicks")]
    pub position_ticks: i64,
    #[serde(rename = "AudioStreamIndex")]
    pub audio_stream_index: Option<i32>,
    #[serde(rename = "SubtitleStreamIndex")]
    pub subtitle_stream_index: Option<i32>,
    #[serde(rename = "VolumeLevel")]
    pub volume_level: Option<i32>,
    #[serde(rename = "IsPaused")]
    pub is_paused: bool,
    #[serde(rename = "IsMuted")]
    pub is_muted: bool,
    #[serde(rename = "PlayMethod")]
    pub play_method: String,
    #[serde(rename = "LiveStreamId")]
    pub live_stream_id: Option<String>,
    #[serde(rename = "PlaySessionId")]
    pub play_session_id: Option<String>,
    #[serde(rename = "RepeatMode")]
    pub repeat_mode: Option<String>,
    #[serde(rename = "MaxStreamingBitrate")]
    pub max_streaming_bitrate: Option<i64>,
}

/// Parameters for playback stop reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackStopInfo {
    #[serde(rename = "ItemId")]
    pub item_id: String,
    #[serde(rename = "SessionId")]
    pub session_id: Option<String>,
    #[serde(rename = "MediaSourceId")]
    pub media_source_id: Option<String>,
    #[serde(rename = "PositionTicks")]
    pub position_ticks: i64,
    #[serde(rename = "LiveStreamId")]
    pub live_stream_id: Option<String>,
    #[serde(rename = "PlaySessionId")]
    pub play_session_id: Option<String>,
    #[serde(rename = "Failed")]
    pub failed: Option<bool>,
    #[serde(rename = "NextMediaType")]
    pub next_media_type: Option<String>,
    #[serde(rename = "PlaylistItemId")]
    pub playlist_item_id: Option<String>,
    #[serde(rename = "NowPlayingQueue")]
    pub now_playing_queue: Option<Vec<String>>,
}

/// Search query parameters
#[derive(Debug, Clone, Default)]
pub struct SearchQuery {
    pub search_term: String,
    pub include_item_types: Vec<String>,
    pub exclude_item_types: Vec<String>,
    pub media_types: Vec<String>,
    pub parent_id: Option<String>,
    pub is_movie: Option<bool>,
    pub is_series: Option<bool>,
    pub is_news: Option<bool>,
    pub is_kids: Option<bool>,
    pub is_sports: Option<bool>,
    pub start_index: Option<u32>,
    pub limit: Option<u32>,
    pub user_id: Option<String>,
}

impl SearchQuery {
    pub fn new<S: Into<String>>(search_term: S) -> Self {
        Self {
            search_term: search_term.into(),
            ..Default::default()
        }
    }

    pub fn include_item_types(mut self, types: Vec<String>) -> Self {
        self.include_item_types = types;
        self
    }

    pub fn exclude_item_types(mut self, types: Vec<String>) -> Self {
        self.exclude_item_types = types;
        self
    }

    pub fn media_types(mut self, types: Vec<String>) -> Self {
        self.media_types = types;
        self
    }

    pub fn parent_id<S: Into<String>>(mut self, parent_id: S) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn start_index(mut self, index: u32) -> Self {
        self.start_index = Some(index);
        self
    }

    pub fn to_query_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();

        params.insert("SearchTerm".to_string(), self.search_term.clone());

        if !self.include_item_types.is_empty() {
            params.insert("IncludeItemTypes".to_string(), self.include_item_types.join(","));
        }

        if !self.exclude_item_types.is_empty() {
            params.insert("ExcludeItemTypes".to_string(), self.exclude_item_types.join(","));
        }

        if !self.media_types.is_empty() {
            params.insert("MediaTypes".to_string(), self.media_types.join(","));
        }

        if let Some(ref parent_id) = self.parent_id {
            params.insert("ParentId".to_string(), parent_id.clone());
        }

        if let Some(limit) = self.limit {
            params.insert("Limit".to_string(), limit.to_string());
        }

        if let Some(start_index) = self.start_index {
            params.insert("StartIndex".to_string(), start_index.to_string());
        }

        params
    }
}