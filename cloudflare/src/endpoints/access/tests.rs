#[cfg(test)]
mod tests {
    use super::super::{
        AccessApplication, AccessPolicy, AccessRule, AccessUser, ApplicationType,
        CreateAccessApplication, CreateAccessApplicationParams, CreateAccessPolicy,
        CreateAccessPolicyParams, CreateAccessServiceToken, CreateServiceTokenParams, CorsHeaders,
        DeleteAccessApplication, DeleteAccessPolicy, DeleteAccessServiceToken, GetAccessApplication,
        GetAccessPolicy, GetAccessServiceToken, GetAccessUser, ListAccessApplications,
        ListAccessPolicies, ListAccessServiceTokens, ListAccessUsers, PolicyDecision,
        RotateAccessServiceToken, ServiceToken, UpdateAccessApplication,
        UpdateAccessApplicationParams, UpdateAccessPolicy, UpdateAccessPolicyParams,
        UpdateAccessServiceToken, UpdateServiceTokenParams,
    };
    use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
    use chrono::{DateTime, Utc};

    #[test]
    fn test_access_application_serialization() {
        let app = AccessApplication {
            id: "app-123".to_string(),
            name: "Test Application".to_string(),
            domain: "test.example.com".to_string(),
            application_type: ApplicationType::SelfHosted,
            session_duration: Some("24h".to_string()),
            auto_redirect_to_identity: Some(true),
            allowed_idps: Some(vec!["google".to_string(), "azure".to_string()]),
            cors_headers: Some(CorsHeaders {
                allowed_origins: Some(vec!["https://example.com".to_string()]),
                allowed_methods: Some(vec!["GET".to_string(), "POST".to_string()]),
                allowed_headers: Some(vec!["Content-Type".to_string()]),
                allow_credentials: Some(true),
                max_age: Some(3600),
            }),
            custom_deny_message: Some("Access denied".to_string()),
            custom_deny_url: Some("https://example.com/denied".to_string()),
            custom_pages: None,
            tags: Some(vec!["production".to_string(), "api".to_string()]),
            logo_url: Some("https://example.com/logo.png".to_string()),
            skip_interstitial: Some(false),
            created_at: Some(
                DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            updated_at: Some(
                DateTime::parse_from_rfc3339("2023-06-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            policies: Some(vec!["policy-1".to_string(), "policy-2".to_string()]),
        };

        let json = serde_json::to_string(&app).unwrap();
        let deserialized: AccessApplication = serde_json::from_str(&json).unwrap();

        assert_eq!(app, deserialized);
        assert_eq!(app.id, "app-123");
        assert_eq!(app.name, "Test Application");
        assert_eq!(app.application_type, ApplicationType::SelfHosted);
    }

    #[test]
    fn test_application_types() {
        let types = vec![
            ApplicationType::SelfHosted,
            ApplicationType::Ssh,
            ApplicationType::Vnc,
            ApplicationType::Biso,
            ApplicationType::AppLauncher,
            ApplicationType::Bookmark,
            ApplicationType::Saas,
        ];

        let json = serde_json::to_string(&types).unwrap();
        let deserialized: Vec<ApplicationType> = serde_json::from_str(&json).unwrap();

        assert_eq!(types, deserialized);

        // Test individual serialization
        assert_eq!(
            serde_json::to_string(&ApplicationType::SelfHosted).unwrap(),
            "\"self_hosted\""
        );
        assert_eq!(
            serde_json::to_string(&ApplicationType::Ssh).unwrap(),
            "\"ssh\""
        );
        assert_eq!(
            serde_json::to_string(&ApplicationType::Saas).unwrap(),
            "\"saas\""
        );
    }

    #[test]
    fn test_access_policy_serialization() {
        let policy = AccessPolicy {
            id: "policy-456".to_string(),
            name: "Allow Admins".to_string(),
            decision: PolicyDecision::Allow,
            include: vec![
                AccessRule::EmailDomain {
                    email_domain: "example.com".to_string(),
                },
                AccessRule::Group {
                    group: "admin-group".to_string(),
                },
            ],
            exclude: Some(vec![AccessRule::Email {
                email: "blocked@example.com".to_string(),
            }]),
            require: Some(vec![AccessRule::Country {
                country: "US".to_string(),
            }]),
            approval_groups: None,
            session_duration: Some("8h".to_string()),
            created_at: Some(
                DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            updated_at: Some(
                DateTime::parse_from_rfc3339("2023-06-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
        };

        let json = serde_json::to_string(&policy).unwrap();
        let deserialized: AccessPolicy = serde_json::from_str(&json).unwrap();

        assert_eq!(policy, deserialized);
        assert_eq!(policy.id, "policy-456");
        assert_eq!(policy.decision, PolicyDecision::Allow);
        assert_eq!(policy.include.len(), 2);
    }

    #[test]
    fn test_access_rules() {
        let rules = vec![
            AccessRule::Email {
                email: "user@example.com".to_string(),
            },
            AccessRule::EmailDomain {
                email_domain: "example.com".to_string(),
            },
            AccessRule::Ip {
                ip: "192.168.1.0/24".to_string(),
            },
            AccessRule::Country {
                country: "US".to_string(),
            },
            AccessRule::Group {
                group: "admin-group".to_string(),
            },
            AccessRule::ServiceToken {
                service_token: "token-123".to_string(),
            },
            AccessRule::Everyone {},
            AccessRule::Certificate {},
            AccessRule::AzureGroup {
                azure_group: "azure-admin".to_string(),
            },
            AccessRule::GitHubOrganization {
                github_organization: "my-org".to_string(),
            },
            AccessRule::GoogleWorkspaceGroup {
                google_workspace_group: "workspace-admin@example.com".to_string(),
            },
        ];

        let json = serde_json::to_string(&rules).unwrap();
        let deserialized: Vec<AccessRule> = serde_json::from_str(&json).unwrap();

        assert_eq!(rules, deserialized);
    }

    #[test]
    fn test_policy_decisions() {
        let decisions = vec![
            PolicyDecision::Allow,
            PolicyDecision::Deny,
            PolicyDecision::NonIdentity,
            PolicyDecision::Bypass,
        ];

        let json = serde_json::to_string(&decisions).unwrap();
        let deserialized: Vec<PolicyDecision> = serde_json::from_str(&json).unwrap();

        assert_eq!(decisions, deserialized);

        // Test individual serialization
        assert_eq!(
            serde_json::to_string(&PolicyDecision::Allow).unwrap(),
            "\"allow\""
        );
        assert_eq!(
            serde_json::to_string(&PolicyDecision::Deny).unwrap(),
            "\"deny\""
        );
    }

    #[test]
    fn test_service_token_serialization() {
        let token = ServiceToken {
            id: "token-789".to_string(),
            name: "API Token".to_string(),
            client_id: "client-123".to_string(),
            client_secret: Some("secret-456".to_string()),
            expires_at: Some(
                DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            created_at: Some(
                DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            updated_at: Some(
                DateTime::parse_from_rfc3339("2023-06-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            last_used_at: None,
            duration: Some("1y".to_string()),
        };

        let json = serde_json::to_string(&token).unwrap();
        let deserialized: ServiceToken = serde_json::from_str(&json).unwrap();

        assert_eq!(token, deserialized);
        assert_eq!(token.id, "token-789");
        assert_eq!(token.client_id, "client-123");
    }

    #[test]
    fn test_cors_headers_serialization() {
        let cors = CorsHeaders {
            allowed_origins: Some(vec![
                "https://example.com".to_string(),
                "https://app.example.com".to_string(),
            ]),
            allowed_methods: Some(vec!["GET".to_string(), "POST".to_string(), "PUT".to_string()]),
            allowed_headers: Some(vec!["Content-Type".to_string(), "Authorization".to_string()]),
            allow_credentials: Some(true),
            max_age: Some(86400),
        };

        let json = serde_json::to_string(&cors).unwrap();
        let deserialized: CorsHeaders = serde_json::from_str(&json).unwrap();

        assert_eq!(cors, deserialized);
        assert_eq!(cors.allowed_origins.as_ref().unwrap().len(), 2);
        assert_eq!(cors.max_age, Some(86400));
    }

    #[test]
    fn test_list_access_applications_endpoint() {
        // Account-level request
        let list_request = ListAccessApplications {
            account_id: "account-123",
            is_account: true,
        };

        assert_eq!(list_request.method(), Method::GET);
        assert_eq!(list_request.path(), "accounts/account-123/access/apps");

        // Zone-level request
        let zone_list_request = ListAccessApplications {
            account_id: "zone-456",
            is_account: false,
        };

        assert_eq!(zone_list_request.path(), "zones/zone-456/access/apps");
    }

    #[test]
    fn test_create_access_application_endpoint() {
        let create_params = CreateAccessApplicationParams {
            name: "Test App".to_string(),
            domain: "test.example.com".to_string(),
            application_type: ApplicationType::SelfHosted,
            session_duration: Some("24h".to_string()),
            auto_redirect_to_identity: Some(true),
            allowed_idps: Some(vec!["google".to_string()]),
            cors_headers: None,
            custom_deny_message: None,
            custom_deny_url: None,
            tags: Some(vec!["production".to_string()]),
            logo_url: None,
            skip_interstitial: Some(false),
        };

        let create_request = CreateAccessApplication {
            account_id: "account-create123",
            params: create_params,
            is_account: true,
        };

        assert_eq!(create_request.method(), Method::POST);
        assert_eq!(
            create_request.path(),
            "accounts/account-create123/access/apps"
        );

        let body = create_request.body();
        assert!(body.is_some());
        if let Some(RequestBody::Json(json)) = body {
            assert!(json.contains("Test App"));
            assert!(json.contains("test.example.com"));
            assert!(json.contains("self_hosted"));
        }
    }

    #[test]
    fn test_access_policy_endpoints() {
        // List policies for application
        let list_app_policies = ListAccessPolicies {
            account_id: "account-123",
            app_id: Some("app-456"),
            is_account: true,
        };

        assert_eq!(list_app_policies.method(), Method::GET);
        assert_eq!(
            list_app_policies.path(),
            "accounts/account-123/access/apps/app-456/policies"
        );

        // List account-level policies
        let list_account_policies = ListAccessPolicies {
            account_id: "account-123",
            app_id: None,
            is_account: true,
        };

        assert_eq!(
            list_account_policies.path(),
            "accounts/account-123/access/policies"
        );

        // Create policy
        let create_params = CreateAccessPolicyParams {
            name: "Test Policy".to_string(),
            decision: PolicyDecision::Allow,
            include: vec![AccessRule::EmailDomain {
                email_domain: "example.com".to_string(),
            }],
            exclude: None,
            require: None,
            approval_groups: None,
            session_duration: None,
        };

        let create_policy = CreateAccessPolicy {
            account_id: "account-456",
            params: create_params,
            app_id: Some("app-789"),
            is_account: true,
        };

        assert_eq!(create_policy.method(), Method::POST);
        assert_eq!(
            create_policy.path(),
            "accounts/account-456/access/apps/app-789/policies"
        );

        let body = create_policy.body();
        assert!(body.is_some());
        if let Some(RequestBody::Json(json)) = body {
            assert!(json.contains("Test Policy"));
            assert!(json.contains("allow"));
            assert!(json.contains("email_domain"));
        }
    }

    #[test]
    fn test_service_token_endpoints() {
        // List tokens
        let list_tokens = ListAccessServiceTokens {
            account_id: "account-tokens",
        };

        assert_eq!(list_tokens.method(), Method::GET);
        assert_eq!(list_tokens.path(), "accounts/account-tokens/access/service_tokens");

        // Create token
        let create_params = CreateServiceTokenParams {
            name: "API Token".to_string(),
            duration: Some("8760h".to_string()), // 1 year
        };

        let create_token = CreateAccessServiceToken {
            account_id: "account-create",
            params: create_params,
        };

        assert_eq!(create_token.method(), Method::POST);
        assert_eq!(
            create_token.path(),
            "accounts/account-create/access/service_tokens"
        );

        let body = create_token.body();
        assert!(body.is_some());
        if let Some(RequestBody::Json(json)) = body {
            assert!(json.contains("API Token"));
            assert!(json.contains("8760h"));
        }

        // Rotate token
        let rotate_token = RotateAccessServiceToken {
            account_id: "account-rotate",
            token_id: "token-rotate123",
        };

        assert_eq!(rotate_token.method(), Method::POST);
        assert_eq!(
            rotate_token.path(),
            "accounts/account-rotate/access/service_tokens/token-rotate123/rotate"
        );
    }

    #[test]
    fn test_access_user_endpoints() {
        // List users
        let list_users = ListAccessUsers {
            account_id: "account-users",
        };

        assert_eq!(list_users.method(), Method::GET);
        assert_eq!(list_users.path(), "accounts/account-users/access/users");

        // Get user
        let get_user = GetAccessUser {
            account_id: "account-getuser",
            user_id: "user-123",
        };

        assert_eq!(get_user.method(), Method::GET);
        assert_eq!(
            get_user.path(),
            "accounts/account-getuser/access/users/user-123"
        );
    }

    #[test]
    fn test_endpoint_paths_consistency() {
        let account_id = "test-account";
        let app_id = "test-app";
        let policy_id = "test-policy";
        let token_id = "test-token";
        let user_id = "test-user";

        // Application paths
        let list_apps = ListAccessApplications {
            account_id,
            is_account: true,
        };
        assert_eq!(list_apps.path(), "accounts/test-account/access/apps");

        let get_app = GetAccessApplication {
            account_id,
            app_id,
            is_account: true,
        };
        assert_eq!(
            get_app.path(),
            "accounts/test-account/access/apps/test-app"
        );

        // Policy paths
        let get_policy = GetAccessPolicy {
            account_id,
            policy_id,
            app_id: Some(app_id),
            is_account: true,
        };
        assert_eq!(
            get_policy.path(),
            "accounts/test-account/access/apps/test-app/policies/test-policy"
        );

        // Service token paths
        let get_token = GetAccessServiceToken {
            account_id,
            token_id,
        };
        assert_eq!(
            get_token.path(),
            "accounts/test-account/access/service_tokens/test-token"
        );

        // User paths
        let get_user = GetAccessUser {
            account_id,
            user_id,
        };
        assert_eq!(
            get_user.path(),
            "accounts/test-account/access/users/test-user"
        );
    }

    #[test]
    fn test_create_params_serialization() {
        let app_params = CreateAccessApplicationParams {
            name: "Serialization Test".to_string(),
            domain: "serialize.example.com".to_string(),
            application_type: ApplicationType::Saas,
            session_duration: Some("12h".to_string()),
            auto_redirect_to_identity: Some(false),
            allowed_idps: None,
            cors_headers: None,
            custom_deny_message: None,
            custom_deny_url: None,
            tags: None,
            logo_url: None,
            skip_interstitial: None,
        };

        let json = serde_json::to_string(&app_params).unwrap();
        let deserialized: CreateAccessApplicationParams = serde_json::from_str(&json).unwrap();

        assert_eq!(app_params, deserialized);
        assert_eq!(app_params.name, "Serialization Test");
        assert_eq!(app_params.application_type, ApplicationType::Saas);

        let policy_params = CreateAccessPolicyParams {
            name: "Test Policy Serialization".to_string(),
            decision: PolicyDecision::Deny,
            include: vec![AccessRule::Everyone {}],
            exclude: None,
            require: None,
            approval_groups: None,
            session_duration: None,
        };

        let policy_json = serde_json::to_string(&policy_params).unwrap();
        let policy_deserialized: CreateAccessPolicyParams =
            serde_json::from_str(&policy_json).unwrap();

        assert_eq!(policy_params, policy_deserialized);
        assert_eq!(policy_params.decision, PolicyDecision::Deny);
    }
}