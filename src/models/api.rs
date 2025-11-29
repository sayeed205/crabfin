//! API response models
//!
//! This module contains data structures that directly map to API responses.
//! These are separate from the application's internal models to maintain clear boundaries.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Server information response from API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServerInfo {
    /// Server unique identifier
    pub id: String,
    /// Server name
    pub name: String,
    /// Server version
    pub version: String,
    /// Operating system the server is running on
    pub operating_system: String,
    /// Local network address
    pub local_address: Option<String>,
    /// WAN address for external access
    pub wan_address: Option<String>,
    /// Server product name
    pub product_name: Option<String>,
    /// Startup wizard completed
    pub startup_wizard_completed: Option<bool>,
}

/// Public server information (available without authentication)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublicServerInfo {
    /// Server unique identifier
    #[serde(rename = "Id")]
    pub id: String,
    /// Server name
    #[serde(rename = "ServerName")]
    pub name: String,
    /// Server version
    #[serde(rename = "Version")]
    pub version: String,
    /// Product name
    #[serde(rename = "ProductName")]
    pub product_name: Option<String>,
    /// Operating system
    #[serde(rename = "OperatingSystem")]
    pub operating_system: Option<String>,
    /// Local address
    #[serde(rename = "LocalAddress")]
    pub local_address: Option<String>,
    /// Startup wizard completed
    #[serde(rename = "StartupWizardCompleted")]
    pub startup_wizard_completed: Option<bool>,
}

/// Authentication request for login endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthRequest {
    /// Username for authentication
    pub username: String,
    /// Password for authentication (plain text)
    pub pw: String,
}

impl AuthRequest {
    /// Create a new authentication request
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            pw: password,
        }
    }
}

/// Authentication response from login endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthResponse {
    /// Access token for API requests
    pub access_token: String,
    /// Server ID
    pub server_id: String,
    /// User information
    pub user: UserInfo,
    /// Session information
    pub session_info: SessionInfo,
}

/// User information from API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserInfo {
    /// User unique identifier
    pub id: String,
    /// Username
    pub name: String,
    /// Server ID
    pub server_id: Option<String>,
    /// Primary image tag
    pub primary_image_tag: Option<String>,
    /// Whether user has password
    pub has_password: bool,
    /// Whether user has configured password
    pub has_configured_password: bool,
    /// Whether user has configured easy password
    pub has_configured_easy_password: bool,
    /// Whether user can manage emby
    pub enable_auto_login: Option<bool>,
    /// Last login date
    pub last_login_date: Option<String>,
    /// Last activity date
    pub last_activity_date: Option<String>,
    /// User configuration
    pub configuration: Option<UserConfiguration>,
    /// User policy
    pub policy: Option<UserPolicy>,
}

/// User configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserConfiguration {
    /// Audio language preference
    pub audio_language_preference: Option<String>,
    /// Play default audio track
    pub play_default_audio_track: bool,
    /// Subtitle language preference
    pub subtitle_language_preference: Option<String>,
    /// Display missing episodes
    pub display_missing_episodes: bool,
    /// Grouped folders
    pub grouped_folders: Vec<String>,
    /// Subtitle mode
    pub subtitle_mode: Option<String>,
    /// Display collections view
    pub display_collections_view: bool,
    /// Enable local password
    pub enable_local_password: bool,
    /// Ordered views
    pub ordered_views: Vec<String>,
    /// Latest items excludes
    pub latest_items_excludes: Vec<String>,
    /// My media excludes
    pub my_media_excludes: Vec<String>,
    /// Hide played in latest
    pub hide_played_in_latest: bool,
    /// Remember audio selections
    pub remember_audio_selections: bool,
    /// Remember subtitle selections
    pub remember_subtitle_selections: bool,
    /// Enable next episode auto play
    pub enable_next_episode_auto_play: bool,
}

/// User policy information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserPolicy {
    /// Whether user is administrator
    pub is_administrator: bool,
    /// Whether user is hidden
    pub is_hidden: bool,
    /// Whether user is disabled
    pub is_disabled: bool,
    /// Max parental rating
    pub max_parental_rating: Option<i32>,
    /// Blocked tags
    pub blocked_tags: Vec<String>,
    /// Enable user preference access
    pub enable_user_preference_access: bool,
    /// Access schedules
    pub access_schedules: Vec<AccessSchedule>,
    /// Block unrated items
    pub block_unrated_items: Vec<String>,
    /// Enable remote control of other users
    pub enable_remote_control_of_other_users: bool,
    /// Enable shared device control
    pub enable_shared_device_control: bool,
    /// Enable remote access
    pub enable_remote_access: bool,
    /// Enable live tv management
    pub enable_live_tv_management: bool,
    /// Enable live tv access
    pub enable_live_tv_access: bool,
    /// Enable media playback
    pub enable_media_playback: bool,
    /// Enable audio playback transcoding
    pub enable_audio_playback_transcoding: bool,
    /// Enable video playback transcoding
    pub enable_video_playback_transcoding: bool,
    /// Enable playback remuxing
    pub enable_playback_remuxing: bool,
    /// Force remote source transcoding
    pub force_remote_source_transcoding: bool,
    /// Enable content deletion
    pub enable_content_deletion: bool,
    /// Enable content deletion from folders
    pub enable_content_deletion_from_folders: Vec<String>,
    /// Enable content downloading
    pub enable_content_downloading: bool,
    /// Enable sync transcoding
    pub enable_sync_transcoding: bool,
    /// Enable media conversion
    pub enable_media_conversion: bool,
    /// Enabled devices
    pub enabled_devices: Vec<String>,
    /// Enable all devices
    pub enable_all_devices: bool,
    /// Enabled channels
    pub enabled_channels: Vec<String>,
    /// Enable all channels
    pub enable_all_channels: bool,
    /// Enabled folders
    pub enabled_folders: Vec<String>,
    /// Enable all folders
    pub enable_all_folders: bool,
    /// Invalid login attempt count
    pub invalid_login_attempt_count: i32,
    /// Login attempts before lockout
    pub login_attempts_before_lockout: i32,
    /// Max active sessions
    pub max_active_sessions: i32,
    /// Enable public sharing
    pub enable_public_sharing: bool,
    /// Blocked media folders
    pub blocked_media_folders: Vec<String>,
    /// Blocked channels
    pub blocked_channels: Vec<String>,
    /// Remote client bitrate limit
    pub remote_client_bitrate_limit: i32,
    /// Authentication provider id
    pub authentication_provider_id: Option<String>,
    /// Password reset provider id
    pub password_reset_provider_id: Option<String>,
    /// Sync play access
    pub sync_play_access: Option<String>,
}

/// Access schedule for user policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccessSchedule {
    /// User ID
    pub user_id: Option<String>,
    /// Day of week
    pub day_of_week: Option<String>,
    /// Start hour
    pub start_hour: f64,
    /// End hour
    pub end_hour: f64,
}

/// Session information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SessionInfo {
    /// Session ID
    pub id: String,
    /// User ID
    pub user_id: String,
    /// Username
    pub user_name: String,
    /// Client name
    pub client: String,
    /// Last activity date
    pub last_activity_date: String,
    /// Last playback check in
    pub last_playback_check_in: String,
    /// Device name
    pub device_name: String,
    /// Device type
    pub device_type: Option<String>,
    /// Device ID
    pub device_id: String,
    /// Application version
    pub application_version: String,
    /// Whether session is active
    pub is_active: bool,
    /// Whether supports media control
    pub supports_media_control: bool,
    /// Whether supports remote control
    pub supports_remote_control: bool,
    /// Now playing item
    pub now_playing_item: Option<BaseItem>,
    /// Now playing queue
    pub now_playing_queue: Option<Vec<QueueItem>>,
    /// Has custom device name
    pub has_custom_device_name: bool,
    /// Playlist item ID
    pub playlist_item_id: Option<String>,
    /// Server ID
    pub server_id: Option<String>,
    /// User primary image tag
    pub user_primary_image_tag: Option<String>,
    /// Supported commands
    pub supported_commands: Vec<String>,
}

/// Queue item for now playing queue
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueueItem {
    /// Item ID
    pub id: String,
    /// Playlist item ID
    pub playlist_item_id: Option<String>,
}

/// Base item structure for media items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BaseItem {
    /// Item unique identifier
    pub id: String,
    /// Item name
    pub name: String,
    /// Original title
    pub original_title: Option<String>,
    /// Server ID
    pub server_id: Option<String>,
    /// Item type (Movie, Series, Episode, etc.)
    #[serde(rename = "Type")]
    pub item_type: String,
    /// Channel ID
    pub channel_id: Option<String>,
    /// Channel name
    pub channel_name: Option<String>,
    /// Overview/description
    pub overview: Option<String>,
    /// Taglines
    pub taglines: Vec<String>,
    /// Genres
    pub genres: Vec<String>,
    /// Community rating
    pub community_rating: Option<f32>,
    /// Cumulative run time ticks
    pub cumulative_run_time_ticks: Option<i64>,
    /// Runtime ticks
    pub runtime_ticks: Option<i64>,
    /// Play access
    pub play_access: Option<String>,
    /// Aspect ratio
    pub aspect_ratio: Option<String>,
    /// Production year
    pub production_year: Option<i32>,
    /// Is place holder
    pub is_place_holder: Option<bool>,
    /// Number
    pub number: Option<String>,
    /// Channel number
    pub channel_number: Option<String>,
    /// Index number
    pub index_number: Option<i32>,
    /// Index number end
    pub index_number_end: Option<i32>,
    /// Parent index number
    pub parent_index_number: Option<i32>,
    /// Remote trailers
    pub remote_trailers: Vec<MediaUrl>,
    /// Provider IDs
    pub provider_ids: HashMap<String, String>,
    /// Is HD
    pub is_hd: Option<bool>,
    /// Is folder
    pub is_folder: Option<bool>,
    /// Parent ID
    pub parent_id: Option<String>,
    /// Parent type
    #[serde(rename = "ParentType")]
    pub parent_type: Option<String>,
    /// Parent backdrop image tags
    pub parent_backdrop_image_tags: Vec<String>,
    /// Parent primary image item ID
    pub parent_primary_image_item_id: Option<String>,
    /// Parent primary image tag
    pub parent_primary_image_tag: Option<String>,
    /// People
    pub people: Vec<BaseItemPerson>,
    /// Studios
    pub studios: Vec<NameGuidPair>,
    /// Genre items
    pub genre_items: Vec<NameGuidPair>,
    /// Parent logo image tag
    pub parent_logo_image_tag: Option<String>,
    /// Parent art item ID
    pub parent_art_item_id: Option<String>,
    /// Parent art image tag
    pub parent_art_image_tag: Option<String>,
    /// Series name
    pub series_name: Option<String>,
    /// Series ID
    pub series_id: Option<String>,
    /// Season ID
    pub season_id: Option<String>,
    /// Special feature count
    pub special_feature_count: Option<i32>,
    /// Display preferences ID
    pub display_preferences_id: Option<String>,
    /// Status
    pub status: Option<String>,
    /// Air time
    pub air_time: Option<String>,
    /// Air days
    pub air_days: Vec<String>,
    /// Tags
    pub tags: Vec<String>,
    /// Primary image aspect ratio
    pub primary_image_aspect_ratio: Option<f64>,
    /// Artists
    pub artists: Vec<String>,
    /// Artist items
    pub artist_items: Vec<NameGuidPair>,
    /// Album
    pub album: Option<String>,
    /// Collection type
    pub collection_type: Option<String>,
    /// Display order
    pub display_order: Option<String>,
    /// Album ID
    pub album_id: Option<String>,
    /// Album primary image tag
    pub album_primary_image_tag: Option<String>,
    /// Series primary image tag
    pub series_primary_image_tag: Option<String>,
    /// Album artist
    pub album_artist: Option<String>,
    /// Album artists
    pub album_artists: Vec<NameGuidPair>,
    /// Season name
    pub season_name: Option<String>,
    /// Media streams
    pub media_streams: Vec<MediaStreamInfo>,
    /// Video type
    pub video_type: Option<String>,
    /// Part count
    pub part_count: Option<i32>,
    /// Media source count
    pub media_source_count: Option<i32>,
    /// Image tags
    pub image_tags: HashMap<String, String>,
    /// Backdrop image tags
    pub backdrop_image_tags: Vec<String>,
    /// Screenshot image tags
    pub screenshot_image_tags: Vec<String>,
    /// Parent logo item ID
    pub parent_logo_item_id: Option<String>,
    /// Parent backdrop item ID
    pub parent_backdrop_item_id: Option<String>,
    /// Parent backdrop image tags
    pub parent_backdrop_image_tags_2: Vec<String>,
    /// Total bitrate
    pub total_bitrate: Option<i32>,
    /// Video 3D format
    pub video3_d_format: Option<String>,
    /// Premiere date
    pub premiere_date: Option<String>,
    /// External URLs
    pub external_urls: Vec<ExternalUrl>,
    /// Media sources
    pub media_sources: Vec<MediaSourceInfo>,
    /// Critic rating
    pub critic_rating: Option<f32>,
    /// Production locations
    pub production_locations: Vec<String>,
    /// Path
    pub path: Option<String>,
    /// Enable media source display
    pub enable_media_source_display: Option<bool>,
    /// Official rating
    pub official_rating: Option<String>,
    /// Custom rating
    pub custom_rating: Option<String>,
    /// Channel primary image tag
    pub channel_primary_image_tag: Option<String>,
    /// Movie count
    pub movie_count: Option<i32>,
    /// Series count
    pub series_count: Option<i32>,
    /// Program count
    pub program_count: Option<i32>,
    /// Episode count
    pub episode_count: Option<i32>,
    /// Song count
    pub song_count: Option<i32>,
    /// Album count
    pub album_count: Option<i32>,
    /// Artist count
    pub artist_count: Option<i32>,
    /// Music video count
    pub music_video_count: Option<i32>,
    /// Lock data
    pub lock_data: Option<bool>,
    /// Width
    pub width: Option<i32>,
    /// Height
    pub height: Option<i32>,
    /// Camera make
    pub camera_make: Option<String>,
    /// Camera model
    pub camera_model: Option<String>,
    /// Software
    pub software: Option<String>,
    /// Exposure time
    pub exposure_time: Option<f64>,
    /// Focal length
    pub focal_length: Option<f64>,
    /// Image orientation
    pub image_orientation: Option<String>,
    /// Aperture
    pub aperture: Option<f64>,
    /// Shutter speed
    pub shutter_speed: Option<f64>,
    /// Latitude
    pub latitude: Option<f64>,
    /// Longitude
    pub longitude: Option<f64>,
    /// Altitude
    pub altitude: Option<f64>,
    /// ISO speed rating
    pub iso_speed_rating: Option<i32>,
    /// Series timer ID
    pub series_timer_id: Option<String>,
    /// Program ID
    pub program_id: Option<String>,
    /// Channel primary image tag
    pub channel_logo_image_tag: Option<String>,
    /// Start date
    pub start_date: Option<String>,
    /// End date
    pub end_date: Option<String>,
    /// Completion percentage
    pub completion_percentage: Option<f64>,
    /// Is repeat
    pub is_repeat: Option<bool>,
    /// Episode title
    pub episode_title: Option<String>,
    /// Channel type
    pub channel_type: Option<String>,
    /// Audio
    pub audio: Option<String>,
    /// Is movie
    pub is_movie: Option<bool>,
    /// Is sports
    pub is_sports: Option<bool>,
    /// Is series
    pub is_series: Option<bool>,
    /// Is live
    pub is_live: Option<bool>,
    /// Is news
    pub is_news: Option<bool>,
    /// Is kids
    pub is_kids: Option<bool>,
    /// Is premiere
    pub is_premiere: Option<bool>,
    /// Timer ID
    pub timer_id: Option<String>,
    /// Current program
    pub current_program: Option<Box<BaseItem>>,
    /// User data
    pub user_data: Option<UserData>,
}

/// User data for items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserData {
    /// Rating
    pub rating: Option<f64>,
    /// Played percentage
    pub played_percentage: Option<f64>,
    /// Unplayed item count
    pub unplayed_item_count: Option<i32>,
    /// Playback position ticks
    pub playback_position_ticks: i64,
    /// Play count
    pub play_count: i32,
    /// Is favorite
    pub is_favorite: bool,
    /// Likes
    pub likes: Option<bool>,
    /// Last played date
    pub last_played_date: Option<String>,
    /// Played
    pub played: bool,
    /// Key
    pub key: Option<String>,
    /// Item ID
    pub item_id: Option<String>,
}

/// Person information for base items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BaseItemPerson {
    /// Name
    pub name: String,
    /// ID
    pub id: String,
    /// Role
    pub role: Option<String>,
    /// Type
    #[serde(rename = "Type")]
    pub person_type: String,
    /// Primary image tag
    pub primary_image_tag: Option<String>,
}

/// Name and GUID pair
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NameGuidPair {
    /// Name
    pub name: String,
    /// ID
    pub id: String,
}

/// Media URL structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaUrl {
    /// URL
    pub url: String,
    /// Name
    pub name: Option<String>,
}

/// External URL structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExternalUrl {
    /// Name
    pub name: String,
    /// URL
    pub url: String,
}

/// Media stream information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaStreamInfo {
    /// Codec
    pub codec: Option<String>,
    /// Codec tag
    pub codec_tag: Option<String>,
    /// Language
    pub language: Option<String>,
    /// Color range
    pub color_range: Option<String>,
    /// Color space
    pub color_space: Option<String>,
    /// Color transfer
    pub color_transfer: Option<String>,
    /// Color primaries
    pub color_primaries: Option<String>,
    /// Comment
    pub comment: Option<String>,
    /// Time base
    pub time_base: Option<String>,
    /// Codec time base
    pub codec_time_base: Option<String>,
    /// Title
    pub title: Option<String>,
    /// Video range
    pub video_range: Option<String>,
    /// Video range type
    pub video_range_type: Option<String>,
    /// Video do vi title
    pub video_do_vi_title: Option<String>,
    /// Local ized undefined
    pub localized_undefined: Option<String>,
    /// Local ized default
    pub localized_default: Option<String>,
    /// Local ized forced
    pub localized_forced: Option<String>,
    /// Local ized external
    pub localized_external: Option<String>,
    /// Display title
    pub display_title: Option<String>,
    /// Nal length size
    pub nal_length_size: Option<String>,
    /// Is interlaced
    pub is_interlaced: bool,
    /// Is AVC
    pub is_avc: Option<bool>,
    /// Channel layout
    pub channel_layout: Option<String>,
    /// Bit rate
    pub bit_rate: Option<i32>,
    /// Bit depth
    pub bit_depth: Option<i32>,
    /// Ref frames
    pub ref_frames: Option<i32>,
    /// Packet length
    pub packet_length: Option<i32>,
    /// Channels
    pub channels: Option<i32>,
    /// Sample rate
    pub sample_rate: Option<i32>,
    /// Is default
    pub is_default: bool,
    /// Is forced
    pub is_forced: bool,
    /// Height
    pub height: Option<i32>,
    /// Width
    pub width: Option<i32>,
    /// Average frame rate
    pub average_frame_rate: Option<f32>,
    /// Real frame rate
    pub real_frame_rate: Option<f32>,
    /// Profile
    pub profile: Option<String>,
    /// Type
    #[serde(rename = "Type")]
    pub stream_type: String,
    /// Aspect ratio
    pub aspect_ratio: Option<String>,
    /// Index
    pub index: i32,
    /// Score
    pub score: Option<i32>,
    /// Is external
    pub is_external: bool,
    /// Delivery method
    pub delivery_method: Option<String>,
    /// Delivery url
    pub delivery_url: Option<String>,
    /// Is external url
    pub is_external_url: Option<bool>,
    /// Is text subtitle stream
    pub is_text_subtitle_stream: bool,
    /// Supports external stream
    pub supports_external_stream: bool,
    /// Path
    pub path: Option<String>,
    /// Pixel format
    pub pixel_format: Option<String>,
    /// Level
    pub level: Option<f64>,
    /// Is anamorphic
    pub is_anamorphic: Option<bool>,
}

/// Media source information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaSourceInfo {
    /// Protocol
    pub protocol: Option<String>,
    /// ID
    pub id: String,
    /// Path
    pub path: Option<String>,
    /// Encoder path
    pub encoder_path: Option<String>,
    /// Encoder protocol
    pub encoder_protocol: Option<String>,
    /// Type
    #[serde(rename = "Type")]
    pub source_type: String,
    /// Container
    pub container: Option<String>,
    /// Size
    pub size: Option<i64>,
    /// Name
    pub name: Option<String>,
    /// Is remote
    pub is_remote: bool,
    /// ETag
    pub e_tag: Option<String>,
    /// Run time ticks
    pub run_time_ticks: Option<i64>,
    /// Read at native framerate
    pub read_at_native_framerate: bool,
    /// Ignore DTS
    pub ignore_dts: bool,
    /// Ignore index
    pub ignore_index: bool,
    /// Gen PTS input
    pub gen_pts_input: bool,
    /// Supports transcoding
    pub supports_transcoding: bool,
    /// Supports direct stream
    pub supports_direct_stream: bool,
    /// Supports direct play
    pub supports_direct_play: bool,
    /// Is infinite stream
    pub is_infinite_stream: bool,
    /// Requires opening
    pub requires_opening: bool,
    /// Opening token
    pub opening_token: Option<String>,
    /// Requires closing
    pub requires_closing: bool,
    /// Live stream id
    pub live_stream_id: Option<String>,
    /// Buffer ms
    pub buffer_ms: Option<i32>,
    /// Requires looping
    pub requires_looping: bool,
    /// Supports probing
    pub supports_probing: bool,
    /// Video type
    pub video_type: Option<String>,
    /// Iso type
    pub iso_type: Option<String>,
    /// Video3D format
    pub video3_d_format: Option<String>,
    /// Media streams
    pub media_streams: Vec<MediaStreamInfo>,
    /// Media attachments
    pub media_attachments: Vec<MediaAttachment>,
    /// Formats
    pub formats: Vec<String>,
    /// Bitrate
    pub bitrate: Option<i32>,
    /// Timestamp
    pub timestamp: Option<String>,
    /// Required HTTP headers
    pub required_http_headers: HashMap<String, String>,
    /// Transcoding URL
    pub transcoding_url: Option<String>,
    /// Transcoding sub protocol
    pub transcoding_sub_protocol: Option<String>,
    /// Transcoding container
    pub transcoding_container: Option<String>,
    /// Analyze duration ms
    pub analyze_duration_ms: Option<i32>,
    /// Default audio stream index
    pub default_audio_stream_index: Option<i32>,
    /// Default subtitle stream index
    pub default_subtitle_stream_index: Option<i32>,
}

/// Media attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaAttachment {
    /// Codec
    pub codec: Option<String>,
    /// Codec tag
    pub codec_tag: Option<String>,
    /// Comment
    pub comment: Option<String>,
    /// Index
    pub index: i32,
    /// File name
    pub file_name: Option<String>,
    /// Mime type
    pub mime_type: Option<String>,
    /// Delivery url
    pub delivery_url: Option<String>,
}

/// Response from Items endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemsResponse {
    /// List of items
    pub items: Vec<BaseItem>,
    /// Total number of records
    pub total_record_count: i32,
    /// Starting index
    pub start_index: i32,
}

/// Search hint result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchHintResult {
    /// Search hints
    pub search_hints: Vec<SearchHint>,
    /// Total record count
    pub total_record_count: i32,
}

/// Individual search hint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchHint {
    /// Item ID
    pub item_id: String,
    /// Item ID as GUID
    pub id: String,
    /// Name
    pub name: String,
    /// Matched term
    pub matched_term: Option<String>,
    /// Index number
    pub index_number: Option<i32>,
    /// Production year
    pub production_year: Option<i32>,
    /// Parent index number
    pub parent_index_number: Option<i32>,
    /// Primary image tag
    pub primary_image_tag: Option<String>,
    /// Thumb image tag
    pub thumb_image_tag: Option<String>,
    /// Thumb image item ID
    pub thumb_image_item_id: Option<String>,
    /// Backdrop image tag
    pub backdrop_image_tag: Option<String>,
    /// Backdrop image item ID
    pub backdrop_image_item_id: Option<String>,
    /// Type
    #[serde(rename = "Type")]
    pub item_type: String,
    /// Is folder
    pub is_folder: Option<bool>,
    /// Run time ticks
    pub run_time_ticks: Option<i64>,
    /// Media type
    pub media_type: Option<String>,
    /// Start date
    pub start_date: Option<String>,
    /// End date
    pub end_date: Option<String>,
    /// Series
    pub series: Option<String>,
    /// Status
    pub status: Option<String>,
    /// Album
    pub album: Option<String>,
    /// Album ID
    pub album_id: Option<String>,
    /// Album artist
    pub album_artist: Option<String>,
    /// Artists
    pub artists: Vec<String>,
    /// Song count
    pub song_count: Option<i32>,
    /// Episode count
    pub episode_count: Option<i32>,
    /// Channel ID
    pub channel_id: Option<String>,
    /// Channel name
    pub channel_name: Option<String>,
    /// Primary image aspect ratio
    pub primary_image_aspect_ratio: Option<f64>,
}

// Request parameter types

/// Sort order enumeration
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

/// Query parameters for items endpoint
#[derive(Debug, Clone, Default)]
pub struct ItemsQuery {
    /// Parent ID to get items from
    pub parent_id: Option<String>,
    /// Include item types filter
    pub include_item_types: Vec<String>,
    /// Exclude item types filter
    pub exclude_item_types: Vec<String>,
    /// Whether to search recursively
    pub recursive: bool,
    /// Sort by fields
    pub sort_by: Vec<String>,
    /// Sort order
    pub sort_order: SortOrder,
    /// Maximum number of items to return
    pub limit: Option<u32>,
    /// Starting index for pagination
    pub start_index: Option<u32>,
    /// Search term
    pub search_term: Option<String>,
    /// User ID for user-specific data
    pub user_id: Option<String>,
    /// Fields to include in response
    pub fields: Vec<String>,
    /// Enable images
    pub enable_images: Option<bool>,
    /// Image type limit
    pub image_type_limit: Option<u32>,
    /// Enable image types
    pub enable_image_types: Vec<String>,
    /// Enable user data
    pub enable_user_data: Option<bool>,
    /// Enable total record count
    pub enable_total_record_count: Option<bool>,
    /// Filters
    pub filters: Vec<String>,
    /// Years
    pub years: Vec<u32>,
    /// Genres
    pub genres: Vec<String>,
    /// Official ratings
    pub official_ratings: Vec<String>,
    /// Tags
    pub tags: Vec<String>,
    /// Studios
    pub studios: Vec<String>,
    /// Artists
    pub artists: Vec<String>,
    /// Albums
    pub albums: Vec<String>,
    /// Person IDs
    pub person_ids: Vec<String>,
    /// Person types
    pub person_types: Vec<String>,
    /// Is played filter
    pub is_played: Option<bool>,
    /// Is favorite filter
    pub is_favorite: Option<bool>,
    /// Has trailer filter
    pub has_trailer: Option<bool>,
    /// Has theme song filter
    pub has_theme_song: Option<bool>,
    /// Has theme video filter
    pub has_theme_video: Option<bool>,
    /// Has subtitles filter
    pub has_subtitles: Option<bool>,
    /// Has special features filter
    pub has_special_features: Option<bool>,
    /// Has lyrics filter
    pub has_lyrics: Option<bool>,
    /// Has overview filter
    pub has_overview: Option<bool>,
    /// Has imdb id filter
    pub has_imdb_id: Option<bool>,
    /// Has tmdb id filter
    pub has_tmdb_id: Option<bool>,
    /// Has tvdb id filter
    pub has_tvdb_id: Option<bool>,
    /// Is HD filter
    pub is_hd: Option<bool>,
    /// Is 4K filter
    pub is_4k: Option<bool>,
    /// Location types
    pub location_types: Vec<String>,
    /// Exclude location types
    pub exclude_location_types: Vec<String>,
    /// Media types
    pub media_types: Vec<String>,
    /// Video types
    pub video_types: Vec<String>,
    /// 3D video types
    pub video_3d_formats: Vec<String>,
    /// Series statuses
    pub series_statuses: Vec<String>,
    /// Name starts with or greater
    pub name_starts_with_or_greater: Option<String>,
    /// Name starts with
    pub name_starts_with: Option<String>,
    /// Name less than
    pub name_less_than: Option<String>,
    /// Adjacent to
    pub adjacent_to: Option<String>,
    /// Min official rating
    pub min_official_rating: Option<String>,
    /// Max official rating
    pub max_official_rating: Option<String>,
    /// Min sort name
    pub min_sort_name: Option<String>,
    /// Min date created
    pub min_date_created: Option<String>,
    /// Max date created
    pub max_date_created: Option<String>,
    /// Min premiere date
    pub min_premiere_date: Option<String>,
    /// Max premiere date
    pub max_premiere_date: Option<String>,
    /// Min community rating
    pub min_community_rating: Option<f64>,
    /// Min critic rating
    pub min_critic_rating: Option<f64>,
    /// Min index number
    pub min_index_number: Option<i32>,
    /// Min player count
    pub min_player_count: Option<i32>,
    /// Max player count
    pub max_player_count: Option<i32>,
    /// Parent index number
    pub parent_index_number: Option<i32>,
    /// Has parental rating
    pub has_parental_rating: Option<bool>,
    /// Is airing
    pub is_airing: Option<bool>,
    /// Max air date
    pub max_air_date: Option<String>,
    /// Min air date
    pub min_air_date: Option<String>,
    /// Series timer ID
    pub series_timer_id: Option<String>,
    /// Min resume percentage
    pub min_resume_percentage: Option<f64>,
    /// Max resume percentage
    pub max_resume_percentage: Option<f64>,
    /// Has aired
    pub has_aired: Option<bool>,
    /// Has official rating
    pub has_official_rating: Option<bool>,
    /// Collapse box set items
    pub collapse_box_set_items: Option<bool>,
    /// Min width
    pub min_width: Option<i32>,
    /// Min height
    pub min_height: Option<i32>,
}

impl ItemsQuery {
    /// Create a new empty ItemsQuery
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert the query parameters to a URL query string
    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        // Helper macro to add optional parameters
        macro_rules! add_param {
            ($field:expr, $name:expr) => {
                if let Some(ref value) = $field {
                    params.push(format!("{}={}", $name, urlencoding::encode(&value.to_string())));
                }
            };
        }

        // Helper macro to add boolean parameters
        macro_rules! add_bool_param {
            ($field:expr, $name:expr) => {
                if let Some(value) = $field {
                    params.push(format!("{}={}", $name, value));
                }
            };
        }

        // Helper macro to add vector parameters
        macro_rules! add_vec_param {
            ($field:expr, $name:expr) => {
                if !$field.is_empty() {
                    let encoded_values: Vec<String> = $field.iter()
                        .map(|v| urlencoding::encode(&v.to_string()).to_string())
                        .collect();
                    params.push(format!("{}={}", $name, encoded_values.join(",")));
                }
            };
        }

        // Add all parameters
        add_param!(self.parent_id, "parentId");
        add_vec_param!(self.include_item_types, "includeItemTypes");
        add_vec_param!(self.exclude_item_types, "excludeItemTypes");

        if self.recursive {
            params.push("recursive=true".to_string());
        }

        add_vec_param!(self.sort_by, "sortBy");

        match self.sort_order {
            SortOrder::Ascending => params.push("sortOrder=Ascending".to_string()),
            SortOrder::Descending => params.push("sortOrder=Descending".to_string()),
        }

        add_param!(self.limit, "limit");
        add_param!(self.start_index, "startIndex");
        add_param!(self.search_term, "searchTerm");
        add_param!(self.user_id, "userId");
        add_vec_param!(self.fields, "fields");
        add_bool_param!(self.enable_images, "enableImages");
        add_param!(self.image_type_limit, "imageTypeLimit");
        add_vec_param!(self.enable_image_types, "enableImageTypes");
        add_bool_param!(self.enable_user_data, "enableUserData");
        add_bool_param!(self.enable_total_record_count, "enableTotalRecordCount");
        add_vec_param!(self.filters, "filters");
        add_vec_param!(self.years, "years");
        add_vec_param!(self.genres, "genres");
        add_vec_param!(self.official_ratings, "officialRatings");
        add_vec_param!(self.tags, "tags");
        add_vec_param!(self.studios, "studios");
        add_vec_param!(self.artists, "artists");
        add_vec_param!(self.albums, "albums");
        add_vec_param!(self.person_ids, "personIds");
        add_vec_param!(self.person_types, "personTypes");
        add_bool_param!(self.is_played, "isPlayed");
        add_bool_param!(self.is_favorite, "isFavorite");
        add_bool_param!(self.has_trailer, "hasTrailer");
        add_bool_param!(self.has_theme_song, "hasThemeSong");
        add_bool_param!(self.has_theme_video, "hasThemeVideo");
        add_bool_param!(self.has_subtitles, "hasSubtitles");
        add_bool_param!(self.has_special_features, "hasSpecialFeatures");
        add_bool_param!(self.has_lyrics, "hasLyrics");
        add_bool_param!(self.has_overview, "hasOverview");
        add_bool_param!(self.has_imdb_id, "hasImdbId");
        add_bool_param!(self.has_tmdb_id, "hasTmdbId");
        add_bool_param!(self.has_tvdb_id, "hasTvdbId");
        add_bool_param!(self.is_hd, "isHd");
        add_bool_param!(self.is_4k, "is4K");
        add_vec_param!(self.location_types, "locationTypes");
        add_vec_param!(self.exclude_location_types, "excludeLocationTypes");
        add_vec_param!(self.media_types, "mediaTypes");
        add_vec_param!(self.video_types, "videoTypes");
        add_vec_param!(self.video_3d_formats, "video3DFormats");
        add_vec_param!(self.series_statuses, "seriesStatuses");
        add_param!(self.name_starts_with_or_greater, "nameStartsWithOrGreater");
        add_param!(self.name_starts_with, "nameStartsWith");
        add_param!(self.name_less_than, "nameLessThan");
        add_param!(self.adjacent_to, "adjacentTo");
        add_param!(self.min_official_rating, "minOfficialRating");
        add_param!(self.max_official_rating, "maxOfficialRating");
        add_param!(self.min_sort_name, "minSortName");
        add_param!(self.min_date_created, "minDateCreated");
        add_param!(self.max_date_created, "maxDateCreated");
        add_param!(self.min_premiere_date, "minPremiereDate");
        add_param!(self.max_premiere_date, "maxPremiereDate");
        add_param!(self.min_community_rating, "minCommunityRating");
        add_param!(self.min_critic_rating, "minCriticRating");
        add_param!(self.min_index_number, "minIndexNumber");
        add_param!(self.min_player_count, "minPlayerCount");
        add_param!(self.max_player_count, "maxPlayerCount");
        add_param!(self.parent_index_number, "parentIndexNumber");
        add_bool_param!(self.has_parental_rating, "hasParentalRating");
        add_bool_param!(self.is_airing, "isAiring");
        add_param!(self.max_air_date, "maxAirDate");
        add_param!(self.min_air_date, "minAirDate");
        add_param!(self.series_timer_id, "seriesTimerId");
        add_param!(self.min_resume_percentage, "minResumePercentage");
        add_param!(self.max_resume_percentage, "maxResumePercentage");
        add_bool_param!(self.has_aired, "hasAired");
        add_bool_param!(self.has_official_rating, "hasOfficialRating");
        add_bool_param!(self.collapse_box_set_items, "collapseBoxSetItems");
        add_param!(self.min_width, "minWidth");
        add_param!(self.min_height, "minHeight");

        params.join("&")
    }

    /// Builder method to set parent ID
    pub fn parent_id(mut self, parent_id: String) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// Builder method to set recursive search
    pub fn recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }

    /// Builder method to set limit
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Builder method to set start index
    pub fn start_index(mut self, start_index: u32) -> Self {
        self.start_index = Some(start_index);
        self
    }

    /// Builder method to add include item types
    pub fn include_item_types(mut self, types: Vec<String>) -> Self {
        self.include_item_types = types;
        self
    }

    /// Builder method to add sort by fields
    pub fn sort_by(mut self, fields: Vec<String>) -> Self {
        self.sort_by = fields;
        self
    }

    /// Builder method to set sort order
    pub fn sort_order(mut self, order: SortOrder) -> Self {
        self.sort_order = order;
        self
    }

    /// Builder method to set search term
    pub fn search_term(mut self, term: String) -> Self {
        self.search_term = Some(term);
        self
    }

    /// Builder method to set user ID
    pub fn user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Builder method to enable user data
    pub fn enable_user_data(mut self, enable: bool) -> Self {
        self.enable_user_data = Some(enable);
        self
    }

    /// Builder method to enable total record count
    pub fn enable_total_record_count(mut self, enable: bool) -> Self {
        self.enable_total_record_count = Some(enable);
        self
    }
}

/// Playback info response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaybackInfoResponse {
    /// Media sources
    pub media_sources: Vec<MediaSourceInfo>,
    /// Play session ID
    pub play_session_id: Option<String>,
    /// Error code
    pub error_code: Option<String>,
}

/// Stream parameters for generating streaming URLs
#[derive(Debug, Clone, Default)]
pub struct StreamParams {
    /// Maximum streaming bitrate
    pub max_streaming_bitrate: Option<u32>,
    /// Container format
    pub container: Option<String>,
    /// Audio codec
    pub audio_codec: Option<String>,
    /// Video codec
    pub video_codec: Option<String>,
    /// Play session ID
    pub play_session_id: Option<String>,
    /// Device profile ID
    pub device_profile_id: Option<String>,
    /// Static streaming (no transcoding)
    pub static_streaming: Option<bool>,
    /// Media source ID
    pub media_source_id: Option<String>,
    /// Audio stream index
    pub audio_stream_index: Option<i32>,
    /// Subtitle stream index
    pub subtitle_stream_index: Option<i32>,
    /// Start time in ticks
    pub start_time_ticks: Option<i64>,
    /// Tag
    pub tag: Option<String>,
}

impl StreamParams {
    /// Create new empty stream parameters
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert to query string
    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(bitrate) = self.max_streaming_bitrate {
            params.push(format!("maxStreamingBitrate={}", bitrate));
        }

        if let Some(ref container) = self.container {
            params.push(format!("container={}", urlencoding::encode(container)));
        }

        if let Some(ref audio_codec) = self.audio_codec {
            params.push(format!("audioCodec={}", urlencoding::encode(audio_codec)));
        }

        if let Some(ref video_codec) = self.video_codec {
            params.push(format!("videoCodec={}", urlencoding::encode(video_codec)));
        }

        if let Some(ref session_id) = self.play_session_id {
            params.push(format!("playSessionId={}", urlencoding::encode(session_id)));
        }

        if let Some(ref profile_id) = self.device_profile_id {
            params.push(format!("deviceProfileId={}", urlencoding::encode(profile_id)));
        }

        if let Some(static_val) = self.static_streaming {
            params.push(format!("static={}", static_val));
        }

        if let Some(ref source_id) = self.media_source_id {
            params.push(format!("mediaSourceId={}", urlencoding::encode(source_id)));
        }

        if let Some(audio_index) = self.audio_stream_index {
            params.push(format!("audioStreamIndex={}", audio_index));
        }

        if let Some(subtitle_index) = self.subtitle_stream_index {
            params.push(format!("subtitleStreamIndex={}", subtitle_index));
        }

        if let Some(start_ticks) = self.start_time_ticks {
            params.push(format!("startTimeTicks={}", start_ticks));
        }

        if let Some(ref tag) = self.tag {
            params.push(format!("tag={}", urlencoding::encode(tag)));
        }

        params.join("&")
    }

    /// Builder method to set max streaming bitrate
    pub fn max_streaming_bitrate(mut self, bitrate: u32) -> Self {
        self.max_streaming_bitrate = Some(bitrate);
        self
    }

    /// Builder method to set container
    pub fn container(mut self, container: String) -> Self {
        self.container = Some(container);
        self
    }

    /// Builder method to set play session ID
    pub fn play_session_id(mut self, session_id: String) -> Self {
        self.play_session_id = Some(session_id);
        self
    }

    /// Builder method to set static streaming
    pub fn static_streaming(mut self, static_val: bool) -> Self {
        self.static_streaming = Some(static_val);
        self
    }

    /// Builder method to set media source ID
    pub fn media_source_id(mut self, source_id: String) -> Self {
        self.media_source_id = Some(source_id);
        self
    }
}

/// Playback start info for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaybackStartInfo {
    /// Can seek
    pub can_seek: bool,
    /// Item
    pub item: BaseItem,
    /// Item ID
    pub item_id: String,
    /// Session ID
    pub session_id: Option<String>,
    /// Media source ID
    pub media_source_id: String,
    /// Audio stream index
    pub audio_stream_index: Option<i32>,
    /// Subtitle stream index
    pub subtitle_stream_index: Option<i32>,
    /// Is paused
    pub is_paused: bool,
    /// Is muted
    pub is_muted: bool,
    /// Position ticks
    pub position_ticks: i64,
    /// Volume level
    pub volume_level: Option<i32>,
    /// Brightness
    pub brightness: Option<i32>,
    /// Aspect ratio
    pub aspect_ratio: Option<String>,
    /// Play method
    pub play_method: String,
    /// Live stream ID
    pub live_stream_id: Option<String>,
    /// Play session ID
    pub play_session_id: String,
    /// Repeat mode
    pub repeat_mode: String,
    /// Now playing queue
    pub now_playing_queue: Option<Vec<QueueItem>>,
    /// Playlist item ID
    pub playlist_item_id: Option<String>,
}

/// Playback progress info for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaybackProgressInfo {
    /// Can seek
    pub can_seek: bool,
    /// Item
    pub item: Option<BaseItem>,
    /// Item ID
    pub item_id: String,
    /// Session ID
    pub session_id: Option<String>,
    /// Media source ID
    pub media_source_id: String,
    /// Audio stream index
    pub audio_stream_index: Option<i32>,
    /// Subtitle stream index
    pub subtitle_stream_index: Option<i32>,
    /// Is paused
    pub is_paused: bool,
    /// Is muted
    pub is_muted: bool,
    /// Position ticks
    pub position_ticks: i64,
    /// Volume level
    pub volume_level: Option<i32>,
    /// Brightness
    pub brightness: Option<i32>,
    /// Aspect ratio
    pub aspect_ratio: Option<String>,
    /// Play method
    pub play_method: String,
    /// Live stream ID
    pub live_stream_id: Option<String>,
    /// Play session ID
    pub play_session_id: String,
    /// Repeat mode
    pub repeat_mode: String,
    /// Now playing queue
    pub now_playing_queue: Option<Vec<QueueItem>>,
    /// Playlist item ID
    pub playlist_item_id: Option<String>,
}

/// Playback stop info for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaybackStopInfo {
    /// Item
    pub item: Option<BaseItem>,
    /// Item ID
    pub item_id: String,
    /// Session ID
    pub session_id: Option<String>,
    /// Media source ID
    pub media_source_id: String,
    /// Position ticks
    pub position_ticks: i64,
    /// Live stream ID
    pub live_stream_id: Option<String>,
    /// Play session ID
    pub play_session_id: String,
    /// Failed
    pub failed: bool,
    /// Next media type
    pub next_media_type: Option<String>,
    /// Playlist item ID
    pub playlist_item_id: Option<String>,
    /// Now playing queue
    pub now_playing_queue: Option<Vec<QueueItem>>,
}