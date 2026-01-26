use crate::client::AuthenticatedClient;
use crate::error::Result;
use crate::models::BaseItemDtoQueryResult;

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
    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(parent_id) = &self.parent_id {
            params.push(format!("ParentId={}", parent_id));
        }

        if let Some(recursive) = self.recursive {
            params.push(format!("Recursive={}", recursive));
        }

        if let Some(types) = &self.include_item_types {
            if !types.is_empty() {
                params.push(format!("IncludeItemTypes={}", types.join(",")));
            }
        }

        if let Some(sort_by) = &self.sort_by {
            if !sort_by.is_empty() {
                params.push(format!("SortBy={}", sort_by.join(",")));
            }
        }

        if let Some(sort_order) = &self.sort_order {
            params.push(format!("SortOrder={}", sort_order));
        }

        if let Some(fields) = &self.fields {
            if !fields.is_empty() {
                params.push(format!("Fields={}", fields.join(",")));
            }
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
}
