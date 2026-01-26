use crate::client::AuthenticatedClient;
use crate::error::Result;
use crate::models::{BaseItemDto, BaseItemDtoQueryResult};

pub async fn get_views(client: &AuthenticatedClient) -> Result<BaseItemDtoQueryResult> {
    let path = format!("/Users/{}/Views", client.user_id());
    let response = client.get(&path).await?;
    let items = response.json::<BaseItemDtoQueryResult>().await?;
    Ok(items)
}

#[derive(Debug, Default, Clone)]
pub struct ItemsQuery {
    pub parent_id: Option<String>,
    pub recursive: Option<bool>,
    pub include_item_types: Option<Vec<String>>,
    pub sort_by: Option<Vec<String>>,
    pub sort_order: Option<String>,
    pub fields: Option<Vec<String>>,
    pub limit: Option<i32>,
    pub start_index: Option<i32>,
}

impl ItemsQuery {
    pub fn with_parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    pub fn with_recursive(mut self, recursive: bool) -> Self {
        self.recursive = Some(recursive);
        self
    }

    pub fn with_sort_by(mut self, sort_by: impl Into<String>) -> Self {
        self.sort_by = Some(vec![sort_by.into()]);
        self
    }

    pub fn with_limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(parent_id) = &self.parent_id {
            params.push(format!("ParentId={}", parent_id));
        }

        if let Some(recursive) = self.recursive {
            params.push(format!("Recursive={}", recursive));
        }

        if let Some(types) = &self.include_item_types
            && !types.is_empty()
        {
            params.push(format!("IncludeItemTypes={}", types.join(",")));
        }

        if let Some(sort_by) = &self.sort_by
            && !sort_by.is_empty()
        {
            params.push(format!("SortBy={}", sort_by.join(",")));
        }

        if let Some(sort_order) = &self.sort_order {
            params.push(format!("SortOrder={}", sort_order));
        }

        if let Some(fields) = &self.fields
            && !fields.is_empty()
        {
            params.push(format!("Fields={}", fields.join(",")));
        }

        if let Some(limit) = self.limit {
            params.push(format!("Limit={}", limit));
        }

        if let Some(start_index) = self.start_index {
            params.push(format!("StartIndex={}", start_index));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

pub async fn get_items(
    client: &AuthenticatedClient,
    query: &ItemsQuery,
) -> Result<BaseItemDtoQueryResult> {
    let path = format!(
        "/Users/{}/Items{}",
        client.user_id(),
        query.to_query_string()
    );
    let response = client.get(&path).await?;
    let items = response.json::<BaseItemDtoQueryResult>().await?;
    Ok(items)
}

#[derive(Debug, Default, Clone)]
pub struct LatestQuery {
    pub parent_id: Option<String>,
    pub limit: Option<i32>,
    pub include_item_types: Option<Vec<String>>,
    pub group_items: Option<bool>,
    pub fields: Option<Vec<String>>,
}

impl LatestQuery {
    pub fn with_parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    pub fn with_limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_include_item_types(mut self, types: Vec<String>) -> Self {
        self.include_item_types = Some(types);
        self
    }

    pub fn with_group_items(mut self, group: bool) -> Self {
        self.group_items = Some(group);
        self
    }

    pub fn with_fields(mut self, fields: Vec<String>) -> Self {
        self.fields = Some(fields);
        self
    }

    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(parent_id) = &self.parent_id {
            params.push(format!("ParentId={}", parent_id));
        }

        if let Some(limit) = self.limit {
            params.push(format!("Limit={}", limit));
        }

        if let Some(types) = &self.include_item_types
            && !types.is_empty()
        {
            params.push(format!("IncludeItemTypes={}", types.join(",")));
        }

        if let Some(group) = self.group_items {
            params.push(format!("GroupItems={}", group));
        }

        if let Some(fields) = &self.fields
            && !fields.is_empty()
        {
            params.push(format!("Fields={}", fields.join(",")));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

pub async fn get_latest(
    client: &AuthenticatedClient,
    query: &LatestQuery,
) -> Result<Vec<BaseItemDto>> {
    let path = format!(
        "/Users/{}/Items/Latest{}",
        client.user_id(),
        query.to_query_string()
    );
    let response = client.get(&path).await?;
    let items = response.json::<Vec<BaseItemDto>>().await?;
    Ok(items)
}

#[derive(Debug, Clone)]
pub struct ResumeQuery {
    pub limit: Option<i32>,
    pub recursive: bool,
    pub media_types: Option<Vec<String>>,
    pub fields: Option<Vec<String>>,
    pub enable_image_types: Option<Vec<String>>,
}

impl Default for ResumeQuery {
    fn default() -> Self {
        Self {
            limit: None,
            recursive: true,
            media_types: Some(vec!["Video".to_string()]),
            fields: None,
            enable_image_types: None,
        }
    }
}

impl ResumeQuery {
    pub fn with_limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }

    pub fn with_media_types(mut self, types: Vec<String>) -> Self {
        self.media_types = Some(types);
        self
    }

    pub fn with_fields(mut self, fields: Vec<String>) -> Self {
        self.fields = Some(fields);
        self
    }

    pub fn with_enable_image_types(mut self, types: Vec<String>) -> Self {
        self.enable_image_types = Some(types);
        self
    }

    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        params.push(format!("Recursive={}", self.recursive));

        if let Some(limit) = self.limit {
            params.push(format!("Limit={}", limit));
        }

        if let Some(types) = &self.media_types
            && !types.is_empty()
        {
            params.push(format!("MediaTypes={}", types.join(",")));
        }

        if let Some(fields) = &self.fields
            && !fields.is_empty()
        {
            params.push(format!("Fields={}", fields.join(",")));
        }

        if let Some(types) = &self.enable_image_types
            && !types.is_empty()
        {
            params.push(format!("EnableImageTypes={}", types.join(",")));
        }

        format!("?{}", params.join("&"))
    }
}

pub async fn get_resume(
    client: &AuthenticatedClient,
    query: &ResumeQuery,
) -> Result<BaseItemDtoQueryResult> {
    let path = format!(
        "/Users/{}/Items/Resume{}",
        client.user_id(),
        query.to_query_string()
    );
    let response = client.get(&path).await?;
    let items = response.json::<BaseItemDtoQueryResult>().await?;
    Ok(items)
}

#[derive(Debug, Clone)]
pub struct NextUpQuery {
    pub user_id: String,
    pub series_id: Option<String>,
    pub limit: Option<i32>,
    pub fields: Option<Vec<String>>,
    pub enable_image_types: Option<Vec<String>>,
}

impl NextUpQuery {
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            series_id: None,
            limit: None,
            fields: None,
            enable_image_types: None,
        }
    }

    pub fn with_series_id(mut self, series_id: impl Into<String>) -> Self {
        self.series_id = Some(series_id.into());
        self
    }

    pub fn with_limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_fields(mut self, fields: Vec<String>) -> Self {
        self.fields = Some(fields);
        self
    }

    pub fn with_enable_image_types(mut self, types: Vec<String>) -> Self {
        self.enable_image_types = Some(types);
        self
    }

    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        params.push(format!("UserId={}", self.user_id));

        if let Some(series_id) = &self.series_id {
            params.push(format!("SeriesId={}", series_id));
        }

        if let Some(limit) = self.limit {
            params.push(format!("Limit={}", limit));
        }

        if let Some(fields) = &self.fields
            && !fields.is_empty()
        {
            params.push(format!("Fields={}", fields.join(",")));
        }

        if let Some(types) = &self.enable_image_types
            && !types.is_empty()
        {
            params.push(format!("EnableImageTypes={}", types.join(",")));
        }

        format!("?{}", params.join("&"))
    }
}

pub async fn get_next_up(
    client: &AuthenticatedClient,
    query: &NextUpQuery,
) -> Result<BaseItemDtoQueryResult> {
    let path = format!("/Shows/NextUp{}", query.to_query_string());
    let response = client.get(&path).await?;
    let items = response.json::<BaseItemDtoQueryResult>().await?;
    Ok(items)
}

pub async fn get_item(client: &AuthenticatedClient, item_id: &str) -> Result<BaseItemDto> {
    let path = format!("/Users/{}/Items/{}", client.user_id(), item_id);
    let response = client.get(&path).await?;
    let item = response.json::<BaseItemDto>().await?;
    Ok(item)
}

#[derive(Debug, Default, Clone)]
pub struct SeasonsQuery {
    pub user_id: Option<String>,
    pub is_special_season: Option<bool>,
    pub fields: Option<Vec<String>>,
    pub enable_images: Option<bool>,
    pub enable_user_data: Option<bool>,
}

impl SeasonsQuery {
    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn with_is_special_season(mut self, is_special: bool) -> Self {
        self.is_special_season = Some(is_special);
        self
    }

    pub fn with_fields(mut self, fields: Vec<String>) -> Self {
        self.fields = Some(fields);
        self
    }

    pub fn with_enable_images(mut self, enable: bool) -> Self {
        self.enable_images = Some(enable);
        self
    }

    pub fn with_enable_user_data(mut self, enable: bool) -> Self {
        self.enable_user_data = Some(enable);
        self
    }

    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(user_id) = &self.user_id {
            params.push(format!("UserId={}", user_id));
        }

        if let Some(is_special) = self.is_special_season {
            params.push(format!("IsSpecialSeason={}", is_special));
        }

        if let Some(fields) = &self.fields
            && !fields.is_empty()
        {
            params.push(format!("Fields={}", fields.join(",")));
        }

        if let Some(enable) = self.enable_images {
            params.push(format!("EnableImages={}", enable));
        }

        if let Some(enable) = self.enable_user_data {
            params.push(format!("EnableUserData={}", enable));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

pub async fn get_seasons(
    client: &AuthenticatedClient,
    series_id: &str,
    query: &SeasonsQuery,
) -> Result<BaseItemDtoQueryResult> {
    let path = format!(
        "/Shows/{}/Seasons{}",
        series_id,
        query.to_query_string()
    );
    let response = client.get(&path).await?;
    let items = response.json::<BaseItemDtoQueryResult>().await?;
    Ok(items)
}

#[derive(Debug, Default, Clone)]
pub struct EpisodesQuery {
    pub season_id: Option<String>,
    pub season: Option<i32>,
    pub user_id: Option<String>,
    pub fields: Option<Vec<String>>,
    pub enable_images: Option<bool>,
    pub enable_user_data: Option<bool>,
    pub start_index: Option<i32>,
    pub limit: Option<i32>,
}

impl EpisodesQuery {
    pub fn with_season_id(mut self, season_id: impl Into<String>) -> Self {
        self.season_id = Some(season_id.into());
        self
    }

    pub fn with_season(mut self, season: i32) -> Self {
        self.season = Some(season);
        self
    }

    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn with_fields(mut self, fields: Vec<String>) -> Self {
        self.fields = Some(fields);
        self
    }

    pub fn with_limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_start_index(mut self, start_index: i32) -> Self {
        self.start_index = Some(start_index);
        self
    }

    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(season_id) = &self.season_id {
            params.push(format!("SeasonId={}", season_id));
        }

        if let Some(season) = self.season {
            params.push(format!("Season={}", season));
        }

        if let Some(user_id) = &self.user_id {
            params.push(format!("UserId={}", user_id));
        }

        if let Some(fields) = &self.fields
            && !fields.is_empty()
        {
            params.push(format!("Fields={}", fields.join(",")));
        }

        if let Some(enable) = self.enable_images {
            params.push(format!("EnableImages={}", enable));
        }

        if let Some(enable) = self.enable_user_data {
            params.push(format!("EnableUserData={}", enable));
        }

        if let Some(limit) = self.limit {
            params.push(format!("Limit={}", limit));
        }

        if let Some(start_index) = self.start_index {
            params.push(format!("StartIndex={}", start_index));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

pub async fn get_episodes(
    client: &AuthenticatedClient,
    series_id: &str,
    query: &EpisodesQuery,
) -> Result<BaseItemDtoQueryResult> {
    let path = format!(
        "/Shows/{}/Episodes{}",
        series_id,
        query.to_query_string()
    );
    let response = client.get(&path).await?;
    let items = response.json::<BaseItemDtoQueryResult>().await?;
    Ok(items)
}

#[derive(Debug, Default, Clone)]
pub struct SimilarQuery {
    pub user_id: Option<String>,
    pub limit: Option<i32>,
    pub fields: Option<Vec<String>>,
}

impl SimilarQuery {
    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn with_limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_fields(mut self, fields: Vec<String>) -> Self {
        self.fields = Some(fields);
        self
    }

    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(user_id) = &self.user_id {
            params.push(format!("UserId={}", user_id));
        }

        if let Some(limit) = self.limit {
            params.push(format!("Limit={}", limit));
        }

        if let Some(fields) = &self.fields
            && !fields.is_empty()
        {
            params.push(format!("Fields={}", fields.join(",")));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

pub async fn get_similar(
    client: &AuthenticatedClient,
    item_id: &str,
    query: &SimilarQuery,
) -> Result<BaseItemDtoQueryResult> {
    let path = format!(
        "/Items/{}/Similar{}",
        item_id,
        query.to_query_string()
    );
    let response = client.get(&path).await?;
    let items = response.json::<BaseItemDtoQueryResult>().await?;
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_items_query_to_string() {
        let query = ItemsQuery {
            parent_id: Some("id-123".to_string()),
            recursive: Some(true),
            limit: Some(10),
            ..Default::default()
        };

        let qs = query.to_query_string();
        assert!(qs.contains("ParentId=id-123"));
        assert!(qs.contains("Recursive=true"));
        assert!(qs.contains("Limit=10"));
        assert!(qs.starts_with('?'));
    }

    #[test]
    fn test_items_query_empty() {
        let query = ItemsQuery::default();
        assert_eq!(query.to_query_string(), "");
    }

    #[test]
    fn test_items_query_arrays() {
        let query = ItemsQuery {
            include_item_types: Some(vec!["Movie".to_string(), "Series".to_string()]),
            ..Default::default()
        };

        let qs = query.to_query_string();
        assert!(qs.contains("IncludeItemTypes=Movie,Series"));
    }

    #[test]
    fn test_latest_query_with_limit() {
        let query = LatestQuery::default().with_limit(16);
        let qs = query.to_query_string();
        assert!(qs.contains("Limit=16"));
        assert!(qs.starts_with('?'));
    }

    #[test]
    fn test_latest_query_with_parent_id() {
        let query = LatestQuery::default()
            .with_parent_id("parent-123")
            .with_limit(10);
        let qs = query.to_query_string();
        assert!(qs.contains("ParentId=parent-123"));
        assert!(qs.contains("Limit=10"));
    }

    #[test]
    fn test_latest_query_empty() {
        let query = LatestQuery::default();
        assert_eq!(query.to_query_string(), "");
    }

    #[test]
    fn test_resume_query_default() {
        let query = ResumeQuery::default();
        let qs = query.to_query_string();
        assert!(qs.contains("Recursive=true"));
        assert!(qs.contains("MediaTypes=Video"));
    }

    #[test]
    fn test_resume_query_with_limit() {
        let query = ResumeQuery::default().with_limit(20);
        let qs = query.to_query_string();
        assert!(qs.contains("Limit=20"));
        assert!(qs.contains("Recursive=true"));
    }

    #[test]
    fn test_next_up_query_with_user_id() {
        let query = NextUpQuery::new("user-123");
        let qs = query.to_query_string();
        assert!(qs.contains("UserId=user-123"));
    }

    #[test]
    fn test_next_up_query_with_series_id() {
        let query = NextUpQuery::new("user-123")
            .with_series_id("series-456")
            .with_limit(5);
        let qs = query.to_query_string();
        assert!(qs.contains("UserId=user-123"));
        assert!(qs.contains("SeriesId=series-456"));
        assert!(qs.contains("Limit=5"));
    }

    #[test]
    fn test_seasons_query_to_string() {
        let query = SeasonsQuery::default()
            .with_user_id("user-123")
            .with_is_special_season(false)
            .with_fields(vec!["Overview".to_string()]);

        let qs = query.to_query_string();
        assert!(qs.contains("UserId=user-123"));
        assert!(qs.contains("IsSpecialSeason=false"));
        assert!(qs.contains("Fields=Overview"));
    }

    #[test]
    fn test_episodes_query_to_string() {
        let query = EpisodesQuery::default()
            .with_season_id("season-1")
            .with_limit(5)
            .with_start_index(0);

        let qs = query.to_query_string();
        assert!(qs.contains("SeasonId=season-1"));
        assert!(qs.contains("Limit=5"));
        assert!(qs.contains("StartIndex=0"));
    }

    #[test]
    fn test_similar_query_to_string() {
        let query = SimilarQuery::default()
            .with_limit(10)
            .with_user_id("user-1");

        let qs = query.to_query_string();
        assert!(qs.contains("Limit=10"));
        assert!(qs.contains("UserId=user-1"));
    }
}
