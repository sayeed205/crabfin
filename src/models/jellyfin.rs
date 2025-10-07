use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Server information response from Jellyfin API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "OperatingSystem")]
    pub operating_system: String,
    #[serde(rename = "LocalAddress")]
    pub local_address: Option<String>,
    #[serde(rename = "WanAddress")]
    pub wan_address: Option<String>,
    #[serde(rename = "ServerName")]
    pub server_name: Option<String>,
    #[serde(rename = "ProductName")]
    pub product_name: Option<String>,
    #[serde(rename = "StartupWizardCompleted")]
    pub startup_wizard_completed: Option<bool>,
}

/// Public server information (available without authentication)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicServerInfo {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "ProductName")]
    pub product_name: String,
    #[serde(rename = "OperatingSystem")]
    pub operating_system: String,
    #[serde(rename = "LocalAddress")]
    pub local_address: Option<String>,
    #[serde(rename = "WanAddress")]
    pub wan_address: Option<String>,
    #[serde(rename = "StartupWizardCompleted")]
    pub startup_wizard_completed: bool,
}

/// Authentication response from login endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    #[serde(rename = "ServerId")]
    pub server_id: String,
    #[serde(rename = "User")]
    pub user: UserInfo,
    #[serde(rename = "SessionInfo")]
    pub session_info: SessionInfo,
}

/// User information from authentication or user endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ServerId")]
    pub server_id: String,
    #[serde(rename = "HasPassword")]
    pub has_password: bool,
    #[serde(rename = "HasConfiguredPassword")]
    pub has_configured_password: bool,
    #[serde(rename = "HasConfiguredEasyPassword")]
    pub has_configured_easy_password: bool,
    #[serde(rename = "EnableAutoLogin")]
    pub enable_auto_login: Option<bool>,
    #[serde(rename = "LastLoginDate")]
    pub last_login_date: Option<String>,
    #[serde(rename = "LastActivityDate")]
    pub last_activity_date: Option<String>,
    #[serde(rename = "Configuration")]
    pub configuration: Option<UserConfiguration>,
    #[serde(rename = "Policy")]
    pub policy: Option<UserPolicy>,
}

/// User configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfiguration {
    #[serde(rename = "PlayDefaultAudioTrack")]
    pub play_default_audio_track: bool,
    #[serde(rename = "SubtitleLanguagePreference")]
    pub subtitle_language_preference: Option<String>,
    #[serde(rename = "DisplayMissingEpisodes")]
    pub display_missing_episodes: bool,
    #[serde(rename = "GroupedFolders")]
    pub grouped_folders: Vec<String>,
    #[serde(rename = "SubtitleMode")]
    pub subtitle_mode: String,
    #[serde(rename = "DisplayCollectionsView")]
    pub display_collections_view: bool,
    #[serde(rename = "EnableLocalPassword")]
    pub enable_local_password: bool,
    #[serde(rename = "OrderedViews")]
    pub ordered_views: Vec<String>,
    #[serde(rename = "LatestItemsExcludes")]
    pub latest_items_excludes: Vec<String>,
    #[serde(rename = "MyMediaExcludes")]
    pub my_media_excludes: Vec<String>,
    #[serde(rename = "HidePlayedInLatest")]
    pub hide_played_in_latest: bool,
    #[serde(rename = "RememberAudioSelections")]
    pub remember_audio_selections: bool,
    #[serde(rename = "RememberSubtitleSelections")]
    pub remember_subtitle_selections: bool,
    #[serde(rename = "EnableNextEpisodeAutoPlay")]
    pub enable_next_episode_auto_play: bool,
}

/// User policy and permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPolicy {
    #[serde(rename = "IsAdministrator")]
    pub is_administrator: bool,
    #[serde(rename = "IsHidden")]
    pub is_hidden: bool,
    #[serde(rename = "IsDisabled")]
    pub is_disabled: bool,
    #[serde(rename = "MaxParentalRating")]
    pub max_parental_rating: Option<i32>,
    #[serde(rename = "BlockedTags")]
    pub blocked_tags: Vec<String>,
    #[serde(rename = "EnableUserPreferenceAccess")]
    pub enable_user_preference_access: bool,
    #[serde(rename = "AccessSchedules")]
    pub access_schedules: Vec<AccessSchedule>,
    #[serde(rename = "BlockUnratedItems")]
    pub block_unrated_items: Vec<String>,
    #[serde(rename = "EnableRemoteControlOfOtherUsers")]
    pub enable_remote_control_of_other_users: bool,
    #[serde(rename = "EnableSharedDeviceControl")]
    pub enable_shared_device_control: bool,
    #[serde(rename = "EnableRemoteAccess")]
    pub enable_remote_access: bool,
    #[serde(rename = "EnableLiveTvManagement")]
    pub enable_live_tv_management: bool,
    #[serde(rename = "EnableLiveTvAccess")]
    pub enable_live_tv_access: bool,
    #[serde(rename = "EnableMediaPlayback")]
    pub enable_media_playback: bool,
    #[serde(rename = "EnableAudioPlaybackTranscoding")]
    pub enable_audio_playback_transcoding: bool,
    #[serde(rename = "EnableVideoPlaybackTranscoding")]
    pub enable_video_playback_transcoding: bool,
    #[serde(rename = "EnablePlaybackRemuxing")]
    pub enable_playback_remuxing: bool,
    #[serde(rename = "ForceRemoteSourceTranscoding")]
    pub force_remote_source_transcoding: bool,
    #[serde(rename = "EnableContentDeletion")]
    pub enable_content_deletion: bool,
    #[serde(rename = "EnableContentDeletionFromFolders")]
    pub enable_content_deletion_from_folders: Vec<String>,
    #[serde(rename = "EnableContentDownloading")]
    pub enable_content_downloading: bool,
    #[serde(rename = "EnableSyncTranscoding")]
    pub enable_sync_transcoding: bool,
    #[serde(rename = "EnableMediaConversion")]
    pub enable_media_conversion: bool,
    #[serde(rename = "EnabledDevices")]
    pub enabled_devices: Vec<String>,
    #[serde(rename = "EnableAllDevices")]
    pub enable_all_devices: bool,
    #[serde(rename = "EnabledChannels")]
    pub enabled_channels: Vec<String>,
    #[serde(rename = "EnableAllChannels")]
    pub enable_all_channels: bool,
    #[serde(rename = "EnabledFolders")]
    pub enabled_folders: Vec<String>,
    #[serde(rename = "EnableAllFolders")]
    pub enable_all_folders: bool,
    #[serde(rename = "InvalidLoginAttemptCount")]
    pub invalid_login_attempt_count: i32,
    #[serde(rename = "LoginAttemptsBeforeLockout")]
    pub login_attempts_before_lockout: i32,
    #[serde(rename = "MaxActiveSessions")]
    pub max_active_sessions: i32,
    #[serde(rename = "EnablePublicSharing")]
    pub enable_public_sharing: bool,
    #[serde(rename = "BlockedMediaFolders")]
    pub blocked_media_folders: Vec<String>,
    #[serde(rename = "BlockedChannels")]
    pub blocked_channels: Vec<String>,
    #[serde(rename = "RemoteClientBitrateLimit")]
    pub remote_client_bitrate_limit: i32,
    #[serde(rename = "AuthenticationProviderId")]
    pub authentication_provider_id: String,
    #[serde(rename = "PasswordResetProviderId")]
    pub password_reset_provider_id: String,
    #[serde(rename = "SyncPlayAccess")]
    pub sync_play_access: String,
}

/// Access schedule for user policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessSchedule {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "DayOfWeek")]
    pub day_of_week: String,
    #[serde(rename = "StartHour")]
    pub start_hour: f64,
    #[serde(rename = "EndHour")]
    pub end_hour: f64,
}

/// Session information from authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    #[serde(rename = "PlayState")]
    pub play_state: Option<PlayState>,
    #[serde(rename = "AdditionalUsers")]
    pub additional_users: Vec<SessionUser>,
    #[serde(rename = "Capabilities")]
    pub capabilities: Option<ClientCapabilities>,
    #[serde(rename = "RemoteEndPoint")]
    pub remote_end_point: Option<String>,
    #[serde(rename = "PlayableMediaTypes")]
    pub playable_media_types: Vec<String>,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "UserName")]
    pub user_name: String,
    #[serde(rename = "Client")]
    pub client: String,
    #[serde(rename = "LastActivityDate")]
    pub last_activity_date: String,
    #[serde(rename = "LastPlaybackCheckIn")]
    pub last_playback_check_in: String,
    #[serde(rename = "DeviceName")]
    pub device_name: String,
    #[serde(rename = "DeviceType")]
    pub device_type: Option<String>,
    #[serde(rename = "NowPlayingItem")]
    pub now_playing_item: Option<BaseItem>,
    #[serde(rename = "FullNowPlayingItem")]
    pub full_now_playing_item: Option<BaseItem>,
    #[serde(rename = "NowViewingItem")]
    pub now_viewing_item: Option<BaseItem>,
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    #[serde(rename = "ApplicationVersion")]
    pub application_version: String,
    #[serde(rename = "TranscodingInfo")]
    pub transcoding_info: Option<TranscodingInfo>,
    #[serde(rename = "IsActive")]
    pub is_active: bool,
    #[serde(rename = "SupportsMediaControl")]
    pub supports_media_control: bool,
    #[serde(rename = "SupportsRemoteControl")]
    pub supports_remote_control: bool,
    #[serde(rename = "NowPlayingQueue")]
    pub now_playing_queue: Vec<QueueItem>,
    #[serde(rename = "HasCustomDeviceName")]
    pub has_custom_device_name: bool,
    #[serde(rename = "PlaylistItemId")]
    pub playlist_item_id: Option<String>,
    #[serde(rename = "ServerId")]
    pub server_id: String,
    #[serde(rename = "UserPrimaryImageTag")]
    pub user_primary_image_tag: Option<String>,
    #[serde(rename = "SupportedCommands")]
    pub supported_commands: Vec<String>,
}

/// Play state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayState {
    #[serde(rename = "CanSeek")]
    pub can_seek: bool,
    #[serde(rename = "IsPaused")]
    pub is_paused: bool,
    #[serde(rename = "IsMuted")]
    pub is_muted: bool,
    #[serde(rename = "RepeatMode")]
    pub repeat_mode: String,
    #[serde(rename = "MaxStreamingBitrate")]
    pub max_streaming_bitrate: Option<i64>,
    #[serde(rename = "PositionTicks")]
    pub position_ticks: Option<i64>,
    #[serde(rename = "PlaybackStartTimeTicks")]
    pub playback_start_time_ticks: Option<i64>,
    #[serde(rename = "VolumeLevel")]
    pub volume_level: Option<i32>,
    #[serde(rename = "Brightness")]
    pub brightness: Option<i32>,
    #[serde(rename = "AspectRatio")]
    pub aspect_ratio: Option<String>,
    #[serde(rename = "PlayMethod")]
    pub play_method: String,
    #[serde(rename = "LiveStreamId")]
    pub live_stream_id: Option<String>,
    #[serde(rename = "PlaySessionId")]
    pub play_session_id: Option<String>,
    #[serde(rename = "PlaylistItemId")]
    pub playlist_item_id: Option<String>,
    #[serde(rename = "MediaSourceId")]
    pub media_source_id: Option<String>,
    #[serde(rename = "AudioStreamIndex")]
    pub audio_stream_index: Option<i32>,
    #[serde(rename = "SubtitleStreamIndex")]
    pub subtitle_stream_index: Option<i32>,
}

/// Session user information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionUser {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "UserName")]
    pub user_name: String,
}

/// Client capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCapabilities {
    #[serde(rename = "PlayableMediaTypes")]
    pub playable_media_types: Vec<String>,
    #[serde(rename = "SupportedCommands")]
    pub supported_commands: Vec<String>,
    #[serde(rename = "SupportsMediaControl")]
    pub supports_media_control: bool,
    #[serde(rename = "SupportsContentUploading")]
    pub supports_content_uploading: bool,
    #[serde(rename = "MessageCallbackUrl")]
    pub message_callback_url: Option<String>,
    #[serde(rename = "SupportsPersistentIdentifier")]
    pub supports_persistent_identifier: bool,
    #[serde(rename = "SupportsSync")]
    pub supports_sync: bool,
    #[serde(rename = "DeviceProfile")]
    pub device_profile: Option<DeviceProfile>,
    #[serde(rename = "AppStoreUrl")]
    pub app_store_url: Option<String>,
    #[serde(rename = "IconUrl")]
    pub icon_url: Option<String>,
}

/// Device profile for transcoding capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceProfile {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Identification")]
    pub identification: Option<DeviceIdentification>,
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,
    #[serde(rename = "ManufacturerUrl")]
    pub manufacturer_url: Option<String>,
    #[serde(rename = "ModelName")]
    pub model_name: Option<String>,
    #[serde(rename = "ModelDescription")]
    pub model_description: Option<String>,
    #[serde(rename = "ModelNumber")]
    pub model_number: Option<String>,
    #[serde(rename = "ModelUrl")]
    pub model_url: Option<String>,
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,
    #[serde(rename = "EnableAlbumArtInDidl")]
    pub enable_album_art_in_didl: bool,
    #[serde(rename = "EnableSingleAlbumArtLimit")]
    pub enable_single_album_art_limit: bool,
    #[serde(rename = "EnableSingleSubtitleLimit")]
    pub enable_single_subtitle_limit: bool,
    #[serde(rename = "SupportedMediaTypes")]
    pub supported_media_types: String,
    #[serde(rename = "UserId")]
    pub user_id: Option<String>,
    #[serde(rename = "AlbumArtPn")]
    pub album_art_pn: Option<String>,
    #[serde(rename = "MaxAlbumArtWidth")]
    pub max_album_art_width: Option<i32>,
    #[serde(rename = "MaxAlbumArtHeight")]
    pub max_album_art_height: Option<i32>,
    #[serde(rename = "MaxIconWidth")]
    pub max_icon_width: Option<i32>,
    #[serde(rename = "MaxIconHeight")]
    pub max_icon_height: Option<i32>,
    #[serde(rename = "MaxStreamingBitrate")]
    pub max_streaming_bitrate: Option<i64>,
    #[serde(rename = "MaxStaticBitrate")]
    pub max_static_bitrate: Option<i64>,
    #[serde(rename = "MusicStreamingTranscodingBitrate")]
    pub music_streaming_transcoding_bitrate: Option<i32>,
    #[serde(rename = "MaxStaticMusicBitrate")]
    pub max_static_music_bitrate: Option<i32>,
    #[serde(rename = "SonyAggregationFlags")]
    pub sony_aggregation_flags: Option<String>,
    #[serde(rename = "ProtocolInfo")]
    pub protocol_info: Option<String>,
    #[serde(rename = "TimelineOffsetSeconds")]
    pub timeline_offset_seconds: i32,
    #[serde(rename = "RequiresPlainVideoItems")]
    pub requires_plain_video_items: bool,
    #[serde(rename = "RequiresPlainFolders")]
    pub requires_plain_folders: bool,
    #[serde(rename = "EnableMSMediaReceiverRegistrar")]
    pub enable_ms_media_receiver_registrar: bool,
    #[serde(rename = "IgnoreTranscodeByteRangeRequests")]
    pub ignore_transcode_byte_range_requests: bool,
    #[serde(rename = "XmlRootAttributes")]
    pub xml_root_attributes: Vec<XmlAttribute>,
    #[serde(rename = "DirectPlayProfiles")]
    pub direct_play_profiles: Vec<DirectPlayProfile>,
    #[serde(rename = "TranscodingProfiles")]
    pub transcoding_profiles: Vec<TranscodingProfile>,
    #[serde(rename = "ContainerProfiles")]
    pub container_profiles: Vec<ContainerProfile>,
    #[serde(rename = "CodecProfiles")]
    pub codec_profiles: Vec<CodecProfile>,
    #[serde(rename = "ResponseProfiles")]
    pub response_profiles: Vec<ResponseProfile>,
    #[serde(rename = "SubtitleProfiles")]
    pub subtitle_profiles: Vec<SubtitleProfile>,
}

/// Device identification information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceIdentification {
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,
    #[serde(rename = "ModelNumber")]
    pub model_number: Option<String>,
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,
    #[serde(rename = "ModelName")]
    pub model_name: Option<String>,
    #[serde(rename = "ModelDescription")]
    pub model_description: Option<String>,
    #[serde(rename = "ModelUrl")]
    pub model_url: Option<String>,
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,
    #[serde(rename = "ManufacturerUrl")]
    pub manufacturer_url: Option<String>,
    #[serde(rename = "Headers")]
    pub headers: Vec<HttpHeaderInfo>,
}

/// HTTP header information for device identification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpHeaderInfo {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Match")]
    pub match_type: String,
}

/// XML attribute for device profiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlAttribute {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,
}

/// Direct play profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectPlayProfile {
    #[serde(rename = "Container")]
    pub container: String,
    #[serde(rename = "AudioCodec")]
    pub audio_codec: Option<String>,
    #[serde(rename = "VideoCodec")]
    pub video_codec: Option<String>,
    #[serde(rename = "Type")]
    pub profile_type: String,
}

/// Transcoding profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscodingProfile {
    #[serde(rename = "Container")]
    pub container: String,
    #[serde(rename = "Type")]
    pub profile_type: String,
    #[serde(rename = "VideoCodec")]
    pub video_codec: Option<String>,
    #[serde(rename = "AudioCodec")]
    pub audio_codec: String,
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,
    #[serde(rename = "EstimateContentLength")]
    pub estimate_content_length: bool,
    #[serde(rename = "EnableMpegtsM2TsMode")]
    pub enable_mpegts_m2ts_mode: bool,
    #[serde(rename = "TranscodeSeekInfo")]
    pub transcode_seek_info: String,
    #[serde(rename = "CopyTimestamps")]
    pub copy_timestamps: bool,
    #[serde(rename = "Context")]
    pub context: String,
    #[serde(rename = "EnableSubtitlesInManifest")]
    pub enable_subtitles_in_manifest: bool,
    #[serde(rename = "MaxAudioChannels")]
    pub max_audio_channels: Option<String>,
    #[serde(rename = "MinSegments")]
    pub min_segments: i32,
    #[serde(rename = "SegmentLength")]
    pub segment_length: i32,
    #[serde(rename = "BreakOnNonKeyFrames")]
    pub break_on_non_key_frames: bool,
}

/// Container profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerProfile {
    #[serde(rename = "Type")]
    pub profile_type: String,
    #[serde(rename = "Conditions")]
    pub conditions: Vec<ProfileCondition>,
    #[serde(rename = "Container")]
    pub container: String,
}

/// Codec profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodecProfile {
    #[serde(rename = "Type")]
    pub profile_type: String,
    #[serde(rename = "Conditions")]
    pub conditions: Vec<ProfileCondition>,
    #[serde(rename = "ApplyConditions")]
    pub apply_conditions: Vec<ProfileCondition>,
    #[serde(rename = "Codec")]
    pub codec: String,
    #[serde(rename = "Container")]
    pub container: Option<String>,
}

/// Response profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseProfile {
    #[serde(rename = "Container")]
    pub container: String,
    #[serde(rename = "AudioCodec")]
    pub audio_codec: Option<String>,
    #[serde(rename = "VideoCodec")]
    pub video_codec: Option<String>,
    #[serde(rename = "Type")]
    pub profile_type: String,
    #[serde(rename = "OrgPn")]
    pub org_pn: Option<String>,
    #[serde(rename = "MimeType")]
    pub mime_type: Option<String>,
    #[serde(rename = "Conditions")]
    pub conditions: Vec<ProfileCondition>,
}

/// Subtitle profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleProfile {
    #[serde(rename = "Format")]
    pub format: String,
    #[serde(rename = "Method")]
    pub method: String,
    #[serde(rename = "DidlMode")]
    pub didl_mode: Option<String>,
    #[serde(rename = "Language")]
    pub language: Option<String>,
    #[serde(rename = "Container")]
    pub container: Option<String>,
}

/// Profile condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileCondition {
    #[serde(rename = "Condition")]
    pub condition: String,
    #[serde(rename = "Property")]
    pub property: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "IsRequired")]
    pub is_required: bool,
}

/// Transcoding information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscodingInfo {
    #[serde(rename = "AudioCodec")]
    pub audio_codec: Option<String>,
    #[serde(rename = "VideoCodec")]
    pub video_codec: Option<String>,
    #[serde(rename = "Container")]
    pub container: Option<String>,
    #[serde(rename = "IsVideoDirect")]
    pub is_video_direct: bool,
    #[serde(rename = "IsAudioDirect")]
    pub is_audio_direct: bool,
    #[serde(rename = "Bitrate")]
    pub bitrate: Option<i32>,
    #[serde(rename = "Framerate")]
    pub framerate: Option<f32>,
    #[serde(rename = "CompletionPercentage")]
    pub completion_percentage: Option<f64>,
    #[serde(rename = "Width")]
    pub width: Option<i32>,
    #[serde(rename = "Height")]
    pub height: Option<i32>,
    #[serde(rename = "AudioChannels")]
    pub audio_channels: Option<i32>,
    #[serde(rename = "HardwareAccelerationType")]
    pub hardware_acceleration_type: Option<String>,
    #[serde(rename = "TranscodeReasons")]
    pub transcode_reasons: Vec<String>,
}

/// Queue item for now playing queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueItem {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "PlaylistItemId")]
    pub playlist_item_id: String,
}

/// Base item representing any media item in Jellyfin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseItem {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "OriginalTitle")]
    pub original_title: Option<String>,
    #[serde(rename = "ServerId")]
    pub server_id: String,
    #[serde(rename = "Etag")]
    pub etag: Option<String>,
    #[serde(rename = "SourceType")]
    pub source_type: Option<String>,
    #[serde(rename = "PlaylistItemId")]
    pub playlist_item_id: Option<String>,
    #[serde(rename = "DateCreated")]
    pub date_created: Option<String>,
    #[serde(rename = "DateLastMediaAdded")]
    pub date_last_media_added: Option<String>,
    #[serde(rename = "ExtraType")]
    pub extra_type: Option<String>,
    #[serde(rename = "AirsBeforeSeasonNumber")]
    pub airs_before_season_number: Option<i32>,
    #[serde(rename = "AirsAfterSeasonNumber")]
    pub airs_after_season_number: Option<i32>,
    #[serde(rename = "AirsBeforeEpisodeNumber")]
    pub airs_before_episode_number: Option<i32>,
    #[serde(rename = "CanDelete")]
    pub can_delete: Option<bool>,
    #[serde(rename = "CanDownload")]
    pub can_download: Option<bool>,
    #[serde(rename = "HasSubtitles")]
    pub has_subtitles: Option<bool>,
    #[serde(rename = "PreferredMetadataLanguage")]
    pub preferred_metadata_language: Option<String>,
    #[serde(rename = "PreferredMetadataCountryCode")]
    pub preferred_metadata_country_code: Option<String>,
    #[serde(rename = "SupportsSync")]
    pub supports_sync: Option<bool>,
    #[serde(rename = "Container")]
    pub container: Option<String>,
    #[serde(rename = "SortName")]
    pub sort_name: Option<String>,
    #[serde(rename = "ForcedSortName")]
    pub forced_sort_name: Option<String>,
    #[serde(rename = "Video3DFormat")]
    pub video_3d_format: Option<String>,
    #[serde(rename = "PremiereDate")]
    pub premiere_date: Option<String>,
    #[serde(rename = "ExternalUrls")]
    pub external_urls: Vec<ExternalUrl>,
    #[serde(rename = "MediaSources")]
    pub media_sources: Vec<MediaSourceInfo>,
    #[serde(rename = "CriticRating")]
    pub critic_rating: Option<f32>,
    #[serde(rename = "ProductionLocations")]
    pub production_locations: Vec<String>,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "EnableMediaSourceDisplay")]
    pub enable_media_source_display: Option<bool>,
    #[serde(rename = "OfficialRating")]
    pub official_rating: Option<String>,
    #[serde(rename = "CustomRating")]
    pub custom_rating: Option<String>,
    #[serde(rename = "ChannelId")]
    pub channel_id: Option<String>,
    #[serde(rename = "ChannelName")]
    pub channel_name: Option<String>,
    #[serde(rename = "Overview")]
    pub overview: Option<String>,
    #[serde(rename = "Taglines")]
    pub taglines: Vec<String>,
    #[serde(rename = "Genres")]
    pub genres: Vec<String>,
    #[serde(rename = "CommunityRating")]
    pub community_rating: Option<f32>,
    #[serde(rename = "CumulativeRunTimeTicks")]
    pub cumulative_run_time_ticks: Option<i64>,
    #[serde(rename = "RunTimeTicks")]
    pub runtime_ticks: Option<i64>,
    #[serde(rename = "PlayAccess")]
    pub play_access: Option<String>,
    #[serde(rename = "AspectRatio")]
    pub aspect_ratio: Option<String>,
    #[serde(rename = "ProductionYear")]
    pub production_year: Option<i32>,
    #[serde(rename = "IsPlaceHolder")]
    pub is_place_holder: Option<bool>,
    #[serde(rename = "Number")]
    pub number: Option<String>,
    #[serde(rename = "ChannelNumber")]
    pub channel_number: Option<String>,
    #[serde(rename = "IndexNumber")]
    pub index_number: Option<i32>,
    #[serde(rename = "IndexNumberEnd")]
    pub index_number_end: Option<i32>,
    #[serde(rename = "ParentIndexNumber")]
    pub parent_index_number: Option<i32>,
    #[serde(rename = "RemoteTrailers")]
    pub remote_trailers: Vec<MediaUrl>,
    #[serde(rename = "ProviderIds")]
    pub provider_ids: HashMap<String, String>,
    #[serde(rename = "IsHD")]
    pub is_hd: Option<bool>,
    #[serde(rename = "IsFolder")]
    pub is_folder: Option<bool>,
    #[serde(rename = "ParentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "Type")]
    pub item_type: String,
    #[serde(rename = "People")]
    pub people: Vec<BaseItemPerson>,
    #[serde(rename = "Studios")]
    pub studios: Vec<NameGuidPair>,
    #[serde(rename = "GenreItems")]
    pub genre_items: Vec<NameGuidPair>,
    #[serde(rename = "ParentLogoItemId")]
    pub parent_logo_item_id: Option<String>,
    #[serde(rename = "ParentBackdropItemId")]
    pub parent_backdrop_item_id: Option<String>,
    #[serde(rename = "ParentBackdropImageTags")]
    pub parent_backdrop_image_tags: Vec<String>,
    #[serde(rename = "LocalTrailerCount")]
    pub local_trailer_count: Option<i32>,
    #[serde(rename = "UserData")]
    pub user_data: Option<UserItemData>,
    #[serde(rename = "RecursiveItemCount")]
    pub recursive_item_count: Option<i32>,
    #[serde(rename = "ChildCount")]
    pub child_count: Option<i32>,
    #[serde(rename = "SeriesName")]
    pub series_name: Option<String>,
    #[serde(rename = "SeriesId")]
    pub series_id: Option<String>,
    #[serde(rename = "SeasonId")]
    pub season_id: Option<String>,
    #[serde(rename = "SpecialFeatureCount")]
    pub special_feature_count: Option<i32>,
    #[serde(rename = "DisplayPreferencesId")]
    pub display_preferences_id: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "AirTime")]
    pub air_time: Option<String>,
    #[serde(rename = "AirDays")]
    pub air_days: Vec<String>,
    #[serde(rename = "Tags")]
    pub tags: Vec<String>,
    #[serde(rename = "PrimaryImageAspectRatio")]
    pub primary_image_aspect_ratio: Option<f64>,
    #[serde(rename = "Artists")]
    pub artists: Vec<String>,
    #[serde(rename = "ArtistItems")]
    pub artist_items: Vec<NameGuidPair>,
    #[serde(rename = "Album")]
    pub album: Option<String>,
    #[serde(rename = "CollectionType")]
    pub collection_type: Option<String>,
    #[serde(rename = "DisplayOrder")]
    pub display_order: Option<String>,
    #[serde(rename = "AlbumId")]
    pub album_id: Option<String>,
    #[serde(rename = "AlbumPrimaryImageTag")]
    pub album_primary_image_tag: Option<String>,
    #[serde(rename = "SeriesPrimaryImageTag")]
    pub series_primary_image_tag: Option<String>,
    #[serde(rename = "AlbumArtist")]
    pub album_artist: Option<String>,
    #[serde(rename = "AlbumArtists")]
    pub album_artists: Vec<NameGuidPair>,
    #[serde(rename = "SeasonName")]
    pub season_name: Option<String>,
    #[serde(rename = "MediaStreams")]
    pub media_streams: Vec<MediaStream>,
    #[serde(rename = "VideoType")]
    pub video_type: Option<String>,
    #[serde(rename = "PartCount")]
    pub part_count: Option<i32>,
    #[serde(rename = "MediaSourceCount")]
    pub media_source_count: Option<i32>,
    #[serde(rename = "ImageTags")]
    pub image_tags: HashMap<String, String>,
    #[serde(rename = "BackdropImageTags")]
    pub backdrop_image_tags: Vec<String>,
    #[serde(rename = "ScreenshotImageTags")]
    pub screenshot_image_tags: Vec<String>,
    #[serde(rename = "ParentLogoImageTag")]
    pub parent_logo_image_tag: Option<String>,
    #[serde(rename = "ParentArtItemId")]
    pub parent_art_item_id: Option<String>,
    #[serde(rename = "ParentArtImageTag")]
    pub parent_art_image_tag: Option<String>,
    #[serde(rename = "SeriesThumbImageTag")]
    pub series_thumb_image_tag: Option<String>,
    #[serde(rename = "ImageBlurHashes")]
    pub image_blur_hashes: HashMap<String, HashMap<String, String>>,
    #[serde(rename = "SeriesStudio")]
    pub series_studio: Option<String>,
    #[serde(rename = "ParentThumbItemId")]
    pub parent_thumb_item_id: Option<String>,
    #[serde(rename = "ParentThumbImageTag")]
    pub parent_thumb_image_tag: Option<String>,
    #[serde(rename = "ParentPrimaryImageItemId")]
    pub parent_primary_image_item_id: Option<String>,
    #[serde(rename = "ParentPrimaryImageTag")]
    pub parent_primary_image_tag: Option<String>,
    #[serde(rename = "Chapters")]
    pub chapters: Vec<ChapterInfo>,
    #[serde(rename = "LocationType")]
    pub location_type: Option<String>,
    #[serde(rename = "IsoType")]
    pub iso_type: Option<String>,
    #[serde(rename = "MediaType")]
    pub media_type: Option<String>,
    #[serde(rename = "EndDate")]
    pub end_date: Option<String>,
    #[serde(rename = "LockedFields")]
    pub locked_fields: Vec<String>,
    #[serde(rename = "TrailerCount")]
    pub trailer_count: Option<i32>,
    #[serde(rename = "MovieCount")]
    pub movie_count: Option<i32>,
    #[serde(rename = "SeriesCount")]
    pub series_count: Option<i32>,
    #[serde(rename = "ProgramCount")]
    pub program_count: Option<i32>,
    #[serde(rename = "EpisodeCount")]
    pub episode_count: Option<i32>,
    #[serde(rename = "SongCount")]
    pub song_count: Option<i32>,
    #[serde(rename = "AlbumCount")]
    pub album_count: Option<i32>,
    #[serde(rename = "ArtistCount")]
    pub artist_count: Option<i32>,
    #[serde(rename = "MusicVideoCount")]
    pub music_video_count: Option<i32>,
    #[serde(rename = "LockData")]
    pub lock_data: Option<bool>,
    #[serde(rename = "Width")]
    pub width: Option<i32>,
    #[serde(rename = "Height")]
    pub height: Option<i32>,
    #[serde(rename = "CameraMake")]
    pub camera_make: Option<String>,
    #[serde(rename = "CameraModel")]
    pub camera_model: Option<String>,
    #[serde(rename = "Software")]
    pub software: Option<String>,
    #[serde(rename = "ExposureTime")]
    pub exposure_time: Option<f64>,
    #[serde(rename = "FocalLength")]
    pub focal_length: Option<f64>,
    #[serde(rename = "ImageOrientation")]
    pub image_orientation: Option<String>,
    #[serde(rename = "Aperture")]
    pub aperture: Option<f64>,
    #[serde(rename = "ShutterSpeed")]
    pub shutter_speed: Option<f64>,
    #[serde(rename = "Latitude")]
    pub latitude: Option<f64>,
    #[serde(rename = "Longitude")]
    pub longitude: Option<f64>,
    #[serde(rename = "Altitude")]
    pub altitude: Option<f64>,
    #[serde(rename = "IsoSpeedRating")]
    pub iso_speed_rating: Option<i32>,
    #[serde(rename = "SeriesTimerId")]
    pub series_timer_id: Option<String>,
    #[serde(rename = "ProgramId")]
    pub program_id: Option<String>,
    #[serde(rename = "ChannelPrimaryImageTag")]
    pub channel_primary_image_tag: Option<String>,
    #[serde(rename = "StartDate")]
    pub start_date: Option<String>,
    #[serde(rename = "CompletionPercentage")]
    pub completion_percentage: Option<f64>,
    #[serde(rename = "IsRepeat")]
    pub is_repeat: Option<bool>,
    #[serde(rename = "EpisodeTitle")]
    pub episode_title: Option<String>,
    #[serde(rename = "ChannelType")]
    pub channel_type: Option<String>,
    #[serde(rename = "Audio")]
    pub audio: Option<String>,
    #[serde(rename = "IsMovie")]
    pub is_movie: Option<bool>,
    #[serde(rename = "IsSports")]
    pub is_sports: Option<bool>,
    #[serde(rename = "IsSeries")]
    pub is_series: Option<bool>,
    #[serde(rename = "IsLive")]
    pub is_live: Option<bool>,
    #[serde(rename = "IsNews")]
    pub is_news: Option<bool>,
    #[serde(rename = "IsKids")]
    pub is_kids: Option<bool>,
    #[serde(rename = "IsPremiere")]
    pub is_premiere: Option<bool>,
    #[serde(rename = "TimerId")]
    pub timer_id: Option<String>,
    #[serde(rename = "CurrentProgram")]
    pub current_program: Option<Box<BaseItem>>,
}

/// External URL information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalUrl {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Url")]
    pub url: String,
}

/// Media source information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaSourceInfo {
    #[serde(rename = "Protocol")]
    pub protocol: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "EncoderPath")]
    pub encoder_path: Option<String>,
    #[serde(rename = "EncoderProtocol")]
    pub encoder_protocol: Option<String>,
    #[serde(rename = "Type")]
    pub source_type: String,
    #[serde(rename = "Container")]
    pub container: Option<String>,
    #[serde(rename = "Size")]
    pub size: Option<i64>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "IsRemote")]
    pub is_remote: bool,
    #[serde(rename = "ETag")]
    pub etag: Option<String>,
    #[serde(rename = "RunTimeTicks")]
    pub runtime_ticks: Option<i64>,
    #[serde(rename = "ReadAtNativeFramerate")]
    pub read_at_native_framerate: bool,
    #[serde(rename = "IgnoreDts")]
    pub ignore_dts: bool,
    #[serde(rename = "IgnoreIndex")]
    pub ignore_index: bool,
    #[serde(rename = "GenPtsInput")]
    pub gen_pts_input: bool,
    #[serde(rename = "SupportsTranscoding")]
    pub supports_transcoding: bool,
    #[serde(rename = "SupportsDirectStream")]
    pub supports_direct_stream: bool,
    #[serde(rename = "SupportsDirectPlay")]
    pub supports_direct_play: bool,
    #[serde(rename = "IsInfiniteStream")]
    pub is_infinite_stream: bool,
    #[serde(rename = "RequiresOpening")]
    pub requires_opening: bool,
    #[serde(rename = "OpenToken")]
    pub open_token: Option<String>,
    #[serde(rename = "RequiresClosing")]
    pub requires_closing: bool,
    #[serde(rename = "LiveStreamId")]
    pub live_stream_id: Option<String>,
    #[serde(rename = "BufferMs")]
    pub buffer_ms: Option<i32>,
    #[serde(rename = "RequiresLooping")]
    pub requires_looping: bool,
    #[serde(rename = "SupportsProbing")]
    pub supports_probing: bool,
    #[serde(rename = "VideoType")]
    pub video_type: Option<String>,
    #[serde(rename = "IsoType")]
    pub iso_type: Option<String>,
    #[serde(rename = "Video3DFormat")]
    pub video_3d_format: Option<String>,
    #[serde(rename = "MediaStreams")]
    pub media_streams: Vec<MediaStream>,
    #[serde(rename = "MediaAttachments")]
    pub media_attachments: Vec<MediaAttachment>,
    #[serde(rename = "Formats")]
    pub formats: Vec<String>,
    #[serde(rename = "Bitrate")]
    pub bitrate: Option<i32>,
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<String>,
    #[serde(rename = "RequiredHttpHeaders")]
    pub required_http_headers: HashMap<String, String>,
    #[serde(rename = "TranscodingUrl")]
    pub transcoding_url: Option<String>,
    #[serde(rename = "TranscodingSubProtocol")]
    pub transcoding_sub_protocol: Option<String>,
    #[serde(rename = "TranscodingContainer")]
    pub transcoding_container: Option<String>,
    #[serde(rename = "AnalyzeDurationMs")]
    pub analyze_duration_ms: Option<i32>,
    #[serde(rename = "DefaultAudioStreamIndex")]
    pub default_audio_stream_index: Option<i32>,
    #[serde(rename = "DefaultSubtitleStreamIndex")]
    pub default_subtitle_stream_index: Option<i32>,
}

/// Media URL information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaUrl {
    #[serde(rename = "Url")]
    pub url: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

/// Person information for media items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseItemPerson {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Role")]
    pub role: Option<String>,
    #[serde(rename = "Type")]
    pub person_type: String,
    #[serde(rename = "PrimaryImageTag")]
    pub primary_image_tag: Option<String>,
    #[serde(rename = "ImageBlurHashes")]
    pub image_blur_hashes: Option<HashMap<String, HashMap<String, String>>>,
}

/// Name and GUID pair for various entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameGuidPair {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,
}

/// User item data (watch status, ratings, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserItemData {
    #[serde(rename = "Rating")]
    pub rating: Option<f64>,
    #[serde(rename = "PlayedPercentage")]
    pub played_percentage: Option<f64>,
    #[serde(rename = "UnplayedItemCount")]
    pub unplayed_item_count: Option<i32>,
    #[serde(rename = "PlaybackPositionTicks")]
    pub playback_position_ticks: i64,
    #[serde(rename = "PlayCount")]
    pub play_count: i32,
    #[serde(rename = "IsFavorite")]
    pub is_favorite: bool,
    #[serde(rename = "Likes")]
    pub likes: Option<bool>,
    #[serde(rename = "LastPlayedDate")]
    pub last_played_date: Option<String>,
    #[serde(rename = "Played")]
    pub played: bool,
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "ItemId")]
    pub item_id: String,
}

/// Media stream information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaStream {
    #[serde(rename = "Codec")]
    pub codec: Option<String>,
    #[serde(rename = "CodecTag")]
    pub codec_tag: Option<String>,
    #[serde(rename = "Language")]
    pub language: Option<String>,
    #[serde(rename = "ColorRange")]
    pub color_range: Option<String>,
    #[serde(rename = "ColorSpace")]
    pub color_space: Option<String>,
    #[serde(rename = "ColorTransfer")]
    pub color_transfer: Option<String>,
    #[serde(rename = "ColorPrimaries")]
    pub color_primaries: Option<String>,
    #[serde(rename = "DvVersionMajor")]
    pub dv_version_major: Option<i32>,
    #[serde(rename = "DvVersionMinor")]
    pub dv_version_minor: Option<i32>,
    #[serde(rename = "DvProfile")]
    pub dv_profile: Option<i32>,
    #[serde(rename = "DvLevel")]
    pub dv_level: Option<i32>,
    #[serde(rename = "RpuPresentFlag")]
    pub rpu_present_flag: Option<i32>,
    #[serde(rename = "ElPresentFlag")]
    pub el_present_flag: Option<i32>,
    #[serde(rename = "BlPresentFlag")]
    pub bl_present_flag: Option<i32>,
    #[serde(rename = "DvBlSignalCompatibilityId")]
    pub dv_bl_signal_compatibility_id: Option<i32>,
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
    #[serde(rename = "TimeBase")]
    pub time_base: Option<String>,
    #[serde(rename = "CodecTimeBase")]
    pub codec_time_base: Option<String>,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "VideoRange")]
    pub video_range: Option<String>,
    #[serde(rename = "VideoRangeType")]
    pub video_range_type: Option<String>,
    #[serde(rename = "VideoDoViTitle")]
    pub video_do_vi_title: Option<String>,
    #[serde(rename = "LocalizedUndefined")]
    pub localized_undefined: Option<String>,
    #[serde(rename = "LocalizedDefault")]
    pub localized_default: Option<String>,
    #[serde(rename = "LocalizedForced")]
    pub localized_forced: Option<String>,
    #[serde(rename = "LocalizedExternal")]
    pub localized_external: Option<String>,
    #[serde(rename = "DisplayTitle")]
    pub display_title: Option<String>,
    #[serde(rename = "NalLengthSize")]
    pub nal_length_size: Option<String>,
    #[serde(rename = "IsInterlaced")]
    pub is_interlaced: bool,
    #[serde(rename = "IsAVC")]
    pub is_avc: Option<bool>,
    #[serde(rename = "ChannelLayout")]
    pub channel_layout: Option<String>,
    #[serde(rename = "BitRate")]
    pub bit_rate: Option<i32>,
    #[serde(rename = "BitDepth")]
    pub bit_depth: Option<i32>,
    #[serde(rename = "RefFrames")]
    pub ref_frames: Option<i32>,
    #[serde(rename = "PacketLength")]
    pub packet_length: Option<i32>,
    #[serde(rename = "Channels")]
    pub channels: Option<i32>,
    #[serde(rename = "SampleRate")]
    pub sample_rate: Option<i32>,
    #[serde(rename = "IsDefault")]
    pub is_default: bool,
    #[serde(rename = "IsForced")]
    pub is_forced: bool,
    #[serde(rename = "Height")]
    pub height: Option<i32>,
    #[serde(rename = "Width")]
    pub width: Option<i32>,
    #[serde(rename = "AverageFrameRate")]
    pub average_frame_rate: Option<f32>,
    #[serde(rename = "RealFrameRate")]
    pub real_frame_rate: Option<f32>,
    #[serde(rename = "Profile")]
    pub profile: Option<String>,
    #[serde(rename = "Type")]
    pub stream_type: String,
    #[serde(rename = "AspectRatio")]
    pub aspect_ratio: Option<String>,
    #[serde(rename = "Index")]
    pub index: i32,
    #[serde(rename = "Score")]
    pub score: Option<i32>,
    #[serde(rename = "IsExternal")]
    pub is_external: bool,
    #[serde(rename = "DeliveryMethod")]
    pub delivery_method: Option<String>,
    #[serde(rename = "DeliveryUrl")]
    pub delivery_url: Option<String>,
    #[serde(rename = "IsExternalUrl")]
    pub is_external_url: Option<bool>,
    #[serde(rename = "IsTextSubtitleStream")]
    pub is_text_subtitle_stream: bool,
    #[serde(rename = "SupportsExternalStream")]
    pub supports_external_stream: bool,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "PixelFormat")]
    pub pixel_format: Option<String>,
    #[serde(rename = "Level")]
    pub level: Option<f64>,
    #[serde(rename = "IsAnamorphic")]
    pub is_anamorphic: Option<bool>,
}

/// Media attachment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAttachment {
    #[serde(rename = "Codec")]
    pub codec: Option<String>,
    #[serde(rename = "CodecTag")]
    pub codec_tag: Option<String>,
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
    #[serde(rename = "Index")]
    pub index: i32,
    #[serde(rename = "FileName")]
    pub file_name: Option<String>,
    #[serde(rename = "MimeType")]
    pub mime_type: Option<String>,
    #[serde(rename = "DeliveryUrl")]
    pub delivery_url: Option<String>,
}

/// Chapter information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterInfo {
    #[serde(rename = "StartPositionTicks")]
    pub start_position_ticks: i64,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ImagePath")]
    pub image_path: Option<String>,
    #[serde(rename = "ImageDateModified")]
    pub image_date_modified: Option<String>,
    #[serde(rename = "ImageTag")]
    pub image_tag: Option<String>,
}