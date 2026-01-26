use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CollectionType {
    Movies,
    TvShows,
    Music,
    MusicVideos,
    Trailers,
    HomeVideos,
    BoxSets,
    Books,
    Photos,
    LiveTv,
    Playlists,
    Folders,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserItemDataDto {
    pub playback_position_ticks: Option<i64>,
    pub play_count: Option<i32>,
    pub is_favorite: Option<bool>,
    pub played: Option<bool>,
    pub played_percentage: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BaseItemPerson {
    pub id: Option<String>,
    pub name: Option<String>,
    pub role: Option<String>,
    #[serde(rename = "Type")]
    pub type_: Option<String>,
    pub primary_image_tag: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NameIdPair {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExternalUrl {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BaseItemDto {
    pub id: String,
    pub name: Option<String>,
    pub server_id: Option<String>,
    #[serde(rename = "Type")]
    pub type_: String, // This is required
    pub collection_type: Option<CollectionType>,
    pub is_folder: Option<bool>,
    pub parent_id: Option<String>,
    pub overview: Option<String>,
    pub production_year: Option<i32>,
    pub premiere_date: Option<String>,
    pub run_time_ticks: Option<i64>,
    pub community_rating: Option<f32>,
    pub official_rating: Option<String>,
    pub image_tags: Option<HashMap<String, String>>,
    pub backdrop_image_tags: Option<Vec<String>>,
    pub user_data: Option<UserItemDataDto>,
    pub series_id: Option<String>,
    pub series_name: Option<String>,
    pub season_id: Option<String>,
    pub season_name: Option<String>,
    pub genres: Option<Vec<String>>,
    pub studios: Option<Vec<NameIdPair>>,
    pub people: Option<Vec<BaseItemPerson>>,
    pub taglines: Option<Vec<String>>,
    pub index_number: Option<i32>,
    pub parent_index_number: Option<i32>,
    pub child_count: Option<i32>,
    pub external_urls: Option<Vec<ExternalUrl>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BaseItemDtoQueryResult {
    pub items: Vec<BaseItemDto>,
    pub total_record_count: Option<i32>,
    pub start_index: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublicServerInfo {
    pub local_address: Option<String>,
    pub server_name: String,
    pub version: String,
    pub product_name: String,
    pub operating_system: Option<String>,
    pub id: String,
    pub startup_wizard_completed: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublicUserInfo {
    pub id: String,
    pub name: String,
    pub has_password: bool,
    pub primary_image_tag: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticateRequest {
    pub username: String,
    #[serde(rename = "Pw")]
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticationResult {
    pub user: UserInfo,
    pub access_token: String,
    pub server_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub has_password: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_public_server_info() {
        let json = r#"{
            "LocalAddress": "http://172.18.0.9:8096",
            "ServerName": "Local",
            "Version": "10.11.6",
            "ProductName": "Jellyfin Server",
            "OperatingSystem": "Linux",
            "Id": "3b77d74a3bca411b924cf99fecb915e9",
            "StartupWizardCompleted": true
        }"#;

        let info: PublicServerInfo =
            from_str(json).expect("Failed to deserialize PublicServerInfo");

        assert_eq!(info.server_name, "Local");
        assert_eq!(info.version, "10.11.6");
        assert_eq!(info.product_name, "Jellyfin Server");
        assert_eq!(info.id, "3b77d74a3bca411b924cf99fecb915e9");
        assert!(info.startup_wizard_completed);
    }

    #[test]
    fn test_serialize_authenticate_request() {
        let req = AuthenticateRequest {
            username: "testuser".into(),
            password: "password123".into(),
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains(r#""Username":"testuser""#));
        assert!(json.contains(r#""Pw":"password123""#));
    }

    #[test]
    fn test_deserialize_authentication_result() {
        let json = r#"{
            "User": {
                "Id": "user-123",
                "Name": "testuser",
                "HasPassword": true
            },
            "AccessToken": "token-abc-123",
            "ServerId": "server-xyz-789"
        }"#;

        let result: AuthenticationResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.access_token, "token-abc-123");
        assert_eq!(result.server_id, "server-xyz-789");
        assert_eq!(result.user.id, "user-123");
        assert_eq!(result.user.name, "testuser");
        assert!(result.user.has_password);
    }

    #[test]
    fn test_deserialize_collection_type() {
        let movies: CollectionType = serde_json::from_str(r#""movies""#).unwrap();
        assert_eq!(movies, CollectionType::Movies);

        let unknown: CollectionType = serde_json::from_str(r#""future_type""#).unwrap();
        assert_eq!(unknown, CollectionType::Unknown);
    }

    #[test]
    fn test_deserialize_user_item_data() {
        let json = r#"{
            "PlaybackPositionTicks": 1000,
            "PlayCount": 5,
            "IsFavorite": true,
            "Played": true
        }"#;

        let data: UserItemDataDto = serde_json::from_str(json).unwrap();
        assert_eq!(data.playback_position_ticks, Some(1000));
        assert_eq!(data.play_count, Some(5));
        assert!(data.is_favorite.unwrap());
        assert!(data.played.unwrap());
    }

    #[test]
    fn test_deserialize_base_item_dto() {
        let json = r#"{
            "Id": "item-123",
            "Name": "Test Movie",
            "Type": "Movie",
            "ProductionYear": 2023,
            "RunTimeTicks": 72000000000
        }"#;

        let item: BaseItemDto = serde_json::from_str(json).unwrap();
        assert_eq!(item.id, "item-123");
        assert_eq!(item.name, Some("Test Movie".to_string()));
        assert_eq!(item.type_, "Movie");
        assert_eq!(item.production_year, Some(2023));
        assert_eq!(item.run_time_ticks, Some(72000000000));
    }

    #[test]
    fn test_deserialize_query_result() {
        let json = r#"{
            "Items": [
                {
                    "Id": "item-1",
                    "Name": "Item 1",
                    "Type": "Movie"
                },
                {
                    "Id": "item-2",
                    "Name": "Item 2",
                    "Type": "Series"
                }
            ],
            "TotalRecordCount": 2,
            "StartIndex": 0
        }"#;

        let result: BaseItemDtoQueryResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.items.len(), 2);
        assert_eq!(result.total_record_count, Some(2));
        assert_eq!(result.items[0].name, Some("Item 1".to_string()));
    }

    #[test]
    fn test_deserialize_base_item_person() {
        let json = r#"{
            "Id": "person-1",
            "Name": "Actor Name",
            "Role": "Hero",
            "Type": "Actor",
            "PrimaryImageTag": "tag-123"
        }"#;

        let person: BaseItemPerson = serde_json::from_str(json).unwrap();
        assert_eq!(person.id, Some("person-1".to_string()));
        assert_eq!(person.name, Some("Actor Name".to_string()));
        assert_eq!(person.role, Some("Hero".to_string()));
        assert_eq!(person.type_, Some("Actor".to_string()));
        assert_eq!(person.primary_image_tag, Some("tag-123".to_string()));
    }

    #[test]
    fn test_deserialize_base_item_dto_with_details() {
        let json = r#"{
            "Id": "item-123",
            "Name": "Test Movie",
            "Type": "Movie",
            "Genres": ["Action", "Adventure"],
            "Studios": [
                { "Id": "studio-1", "Name": "Studio One" }
            ],
            "People": [
                { "Name": "Actor 1", "Type": "Actor" }
            ],
            "Taglines": ["Just when you thought it was safe..."],
            "IndexNumber": 1,
            "ParentIndexNumber": 2,
            "ChildCount": 5,
            "ExternalUrls": [
                { "Name": "IMDb", "Url": "https://imdb.com/title/tt1234567" }
            ]
        }"#;

        let item: BaseItemDto = serde_json::from_str(json).unwrap();
        assert_eq!(item.id, "item-123");
        assert_eq!(item.name, Some("Test Movie".to_string()));
        assert_eq!(item.genres.unwrap(), vec!["Action", "Adventure"]);
        assert_eq!(
            item.studios.unwrap()[0].name,
            Some("Studio One".to_string())
        );
        assert_eq!(item.people.unwrap()[0].name, Some("Actor 1".to_string()));
        assert_eq!(
            item.taglines.unwrap()[0],
            "Just when you thought it was safe..."
        );
        assert_eq!(item.index_number, Some(1));
        assert_eq!(item.parent_index_number, Some(2));
        assert_eq!(item.child_count, Some(5));
        assert_eq!(
            item.external_urls.unwrap()[0].name,
            Some("IMDb".to_string())
        );
    }
}
