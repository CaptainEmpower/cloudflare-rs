#[cfg(test)]
mod tests {
    use super::super::{
        CreatePageRule, CreatePageRuleParams, DeletePageRule, EditPageRule, GetPageRule,
        ListPageRules, PageRule, PageRuleAction, PageRuleActionValue, PageRuleConstraint,
        PageRuleStatus, PageRuleTarget, UpdatePageRule, UpdatePageRuleParams,
    };
    use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
    use chrono::{DateTime, Utc};

    #[test]
    fn test_page_rule_serialization() {
        let page_rule = PageRule {
            id: "page-rule-123".to_string(),
            actions: vec![
                PageRuleAction::browser_check(true),
                PageRuleAction::cache_level("aggressive"),
                PageRuleAction::browser_cache_ttl(3600),
            ],
            targets: vec![PageRuleTarget::url_matches("*example.com/images/*")],
            status: PageRuleStatus::Active,
            priority: Some(1),
            created_on: Some(
                DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            modified_on: Some(
                DateTime::parse_from_rfc3339("2023-06-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
        };

        let json = serde_json::to_string(&page_rule).unwrap();
        let deserialized: PageRule = serde_json::from_str(&json).unwrap();

        assert_eq!(page_rule, deserialized);
        assert_eq!(page_rule.id, "page-rule-123");
        assert_eq!(page_rule.actions.len(), 3);
        assert_eq!(page_rule.targets.len(), 1);
        assert_eq!(page_rule.status, PageRuleStatus::Active);
        assert_eq!(page_rule.priority, Some(1));
    }

    #[test]
    fn test_page_rule_action_values() {
        // Test string value
        let string_action = PageRuleAction {
            id: "browser_check".to_string(),
            value: Some(PageRuleActionValue::String("on".to_string())),
        };

        // Test number value
        let number_action = PageRuleAction {
            id: "browser_cache_ttl".to_string(),
            value: Some(PageRuleActionValue::Number(3600)),
        };

        // Test boolean value
        let bool_action = PageRuleAction {
            id: "some_setting".to_string(),
            value: Some(PageRuleActionValue::Boolean(true)),
        };

        // Test object value
        let object_action = PageRuleAction {
            id: "forwarding_url".to_string(),
            value: Some(PageRuleActionValue::Object(serde_json::json!({
                "url": "https://redirect.example.com",
                "status_code": 302
            }))),
        };

        let actions = vec![string_action, number_action, bool_action, object_action];
        let json = serde_json::to_string(&actions).unwrap();
        let deserialized: Vec<PageRuleAction> = serde_json::from_str(&json).unwrap();

        assert_eq!(actions, deserialized);
    }

    #[test]
    fn test_page_rule_target_types() {
        // Test matches operator
        let matches_target = PageRuleTarget::url_matches("*example.com/*");
        assert_eq!(matches_target.target, "url");
        assert_eq!(matches_target.constraint.operator, "matches");
        assert_eq!(matches_target.constraint.value, "*example.com/*");

        // Test equals operator
        let equals_target = PageRuleTarget::url_equals("https://example.com/exact");
        assert_eq!(equals_target.constraint.operator, "equals");

        // Test contains operator
        let contains_target = PageRuleTarget::url_contains("/api/");
        assert_eq!(contains_target.constraint.operator, "contains");
        assert_eq!(contains_target.constraint.value, "/api/");
    }

    #[test]
    fn test_page_rule_action_helpers() {
        // Test common actions
        let always_online = PageRuleAction::always_online(true);
        assert_eq!(always_online.id, "always_online");

        let cache_ttl = PageRuleAction::browser_cache_ttl(7200);
        assert_eq!(cache_ttl.id, "browser_cache_ttl");
        if let Some(PageRuleActionValue::Number(ttl)) = cache_ttl.value {
            assert_eq!(ttl, 7200);
        } else {
            panic!("Expected number value");
        }

        let minify = PageRuleAction::minify(true, false, true);
        assert_eq!(minify.id, "minify");

        let ssl = PageRuleAction::ssl("strict");
        assert_eq!(ssl.id, "ssl");

        let forwarding = PageRuleAction::forwarding_url("https://new.example.com", 301);
        assert_eq!(forwarding.id, "forwarding_url");
    }

    #[test]
    fn test_list_page_rules_endpoint() {
        let list_request = ListPageRules {
            zone_id: "zone-abc123",
            status: Some("active"),
            order: Some("priority"),
            direction: Some("asc"),
            match_type: Some("all"),
        };

        assert_eq!(list_request.method(), Method::GET);
        assert_eq!(list_request.path(), "zones/zone-abc123/pagerules");

        let query = list_request.query();
        assert!(query.is_some());
        let query_string = query.unwrap();
        assert!(query_string.contains("status=active"));
        assert!(query_string.contains("order=priority"));
        assert!(query_string.contains("direction=asc"));
        assert!(query_string.contains("match=all"));

        // Test without query parameters
        let list_request_no_params = ListPageRules {
            zone_id: "zone-abc123",
            status: None,
            order: None,
            direction: None,
            match_type: None,
        };

        assert_eq!(list_request_no_params.query(), None);
    }

    #[test]
    fn test_get_page_rule_endpoint() {
        let get_request = GetPageRule {
            zone_id: "zone-def456",
            rule_id: "rule-789",
        };

        assert_eq!(get_request.method(), Method::GET);
        assert_eq!(get_request.path(), "zones/zone-def456/pagerules/rule-789");
    }

    #[test]
    fn test_create_page_rule_endpoint() {
        let create_params = CreatePageRuleParams {
            actions: vec![
                PageRuleAction::browser_check(true),
                PageRuleAction::cache_level("aggressive"),
            ],
            targets: vec![PageRuleTarget::url_matches("*example.com/api/*")],
            status: Some(PageRuleStatus::Active),
            priority: Some(1),
        };

        let create_request = CreatePageRule {
            zone_id: "zone-create123",
            params: create_params,
        };

        assert_eq!(create_request.method(), Method::POST);
        assert_eq!(create_request.path(), "zones/zone-create123/pagerules");

        let body = create_request.body();
        assert!(body.is_some());
        if let Some(RequestBody::Json(json)) = body {
            assert!(json.contains("\"browser_check\""));
            assert!(json.contains("\"cache_level\""));
            assert!(json.contains("*example.com/api/*"));
        }
    }

    #[test]
    fn test_update_page_rule_endpoint() {
        let update_params = CreatePageRuleParams {
            actions: vec![PageRuleAction::ssl("flexible")],
            targets: vec![PageRuleTarget::url_equals("https://secure.example.com")],
            status: Some(PageRuleStatus::Disabled),
            priority: None,
        };

        let update_request = UpdatePageRule {
            zone_id: "zone-update456",
            rule_id: "rule-update789",
            params: update_params,
        };

        assert_eq!(update_request.method(), Method::PUT);
        assert_eq!(
            update_request.path(),
            "zones/zone-update456/pagerules/rule-update789"
        );

        let body = update_request.body();
        assert!(body.is_some());
        if let Some(RequestBody::Json(json)) = body {
            assert!(json.contains("\"ssl\""));
            assert!(json.contains("https://secure.example.com"));
        }
    }

    #[test]
    fn test_edit_page_rule_endpoint() {
        let edit_params = UpdatePageRuleParams {
            actions: None,
            targets: None,
            status: Some(PageRuleStatus::Active),
            priority: Some(5),
        };

        let edit_request = EditPageRule {
            zone_id: "zone-edit789",
            rule_id: "rule-edit123",
            params: edit_params,
        };

        assert_eq!(edit_request.method(), Method::PATCH);
        assert_eq!(
            edit_request.path(),
            "zones/zone-edit789/pagerules/rule-edit123"
        );

        let body = edit_request.body();
        assert!(body.is_some());
        if let Some(RequestBody::Json(json)) = body {
            assert!(json.contains("\"status\""));
            assert!(json.contains("\"priority\":5"));
        }
    }

    #[test]
    fn test_delete_page_rule_endpoint() {
        let delete_request = DeletePageRule {
            zone_id: "zone-delete999",
            rule_id: "rule-delete111",
        };

        assert_eq!(delete_request.method(), Method::DELETE);
        assert_eq!(
            delete_request.path(),
            "zones/zone-delete999/pagerules/rule-delete111"
        );
    }

    #[test]
    fn test_page_rule_status_serialization() {
        let active_status = PageRuleStatus::Active;
        let disabled_status = PageRuleStatus::Disabled;

        let active_json = serde_json::to_string(&active_status).unwrap();
        let disabled_json = serde_json::to_string(&disabled_status).unwrap();

        assert_eq!(active_json, "\"active\"");
        assert_eq!(disabled_json, "\"disabled\"");

        let active_deserialized: PageRuleStatus = serde_json::from_str(&active_json).unwrap();
        let disabled_deserialized: PageRuleStatus = serde_json::from_str(&disabled_json).unwrap();

        assert_eq!(active_deserialized, PageRuleStatus::Active);
        assert_eq!(disabled_deserialized, PageRuleStatus::Disabled);
    }

    #[test]
    fn test_create_page_rule_params_serialization() {
        let params = CreatePageRuleParams {
            actions: vec![
                PageRuleAction::cache_level("basic"),
                PageRuleAction::edge_cache_ttl(86400),
            ],
            targets: vec![PageRuleTarget::url_contains("/cache")],
            status: Some(PageRuleStatus::Active),
            priority: Some(10),
        };

        let json = serde_json::to_string(&params).unwrap();
        let deserialized: CreatePageRuleParams = serde_json::from_str(&json).unwrap();

        assert_eq!(params, deserialized);
        assert_eq!(params.actions.len(), 2);
        assert_eq!(params.targets.len(), 1);
        assert_eq!(params.status, Some(PageRuleStatus::Active));
        assert_eq!(params.priority, Some(10));
    }

    #[test]
    fn test_endpoint_paths_consistency() {
        let zone_id = "test-zone-123";
        let rule_id = "test-rule-456";

        // List endpoint
        let list = ListPageRules {
            zone_id,
            status: None,
            order: None,
            direction: None,
            match_type: None,
        };
        assert_eq!(list.path(), "zones/test-zone-123/pagerules");

        // Get endpoint
        let get = GetPageRule { zone_id, rule_id };
        assert_eq!(get.path(), "zones/test-zone-123/pagerules/test-rule-456");

        // Create endpoint
        let create = CreatePageRule {
            zone_id,
            params: CreatePageRuleParams {
                actions: vec![PageRuleAction::browser_check(true)],
                targets: vec![PageRuleTarget::url_matches("*")],
                status: None,
                priority: None,
            },
        };
        assert_eq!(create.path(), "zones/test-zone-123/pagerules");

        // Delete endpoint
        let delete = DeletePageRule { zone_id, rule_id };
        assert_eq!(delete.path(), "zones/test-zone-123/pagerules/test-rule-456");
    }
}