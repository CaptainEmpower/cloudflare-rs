use crate::framework::response::ApiResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A Cloudflare Page Rule
/// https://developers.cloudflare.com/api/resources/page_rules/
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PageRule {
    /// Rule identifier tag
    pub id: String,
    /// The set of actions to take when the rule is matched
    pub actions: Vec<PageRuleAction>,
    /// Targets to evaluate on each request
    pub targets: Vec<PageRuleTarget>,
    /// Status of the rule (active/disabled)
    pub status: PageRuleStatus,
    /// Priority of the rule (lower numbers are higher priority)
    pub priority: Option<u32>,
    /// When the rule was created
    pub created_on: Option<DateTime<Utc>>,
    /// When the rule was last modified
    pub modified_on: Option<DateTime<Utc>>,
}

/// Page Rule Action
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PageRuleAction {
    /// Action identifier
    pub id: String,
    /// Action value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<PageRuleActionValue>,
}

/// Page Rule Action Value (supports multiple types)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum PageRuleActionValue {
    String(String),
    Number(u32),
    Boolean(bool),
    Object(serde_json::Value),
}

/// Page Rule Target
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PageRuleTarget {
    /// The target type (usually "url")
    pub target: String,
    /// The constraint for this target
    pub constraint: PageRuleConstraint,
}

/// Page Rule Constraint
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PageRuleConstraint {
    /// Constraint operator
    pub operator: String,
    /// Constraint value
    pub value: String,
}

/// Page Rule Status
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PageRuleStatus {
    Active,
    Disabled,
}

/// Parameters for creating a new page rule
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CreatePageRuleParams {
    /// The set of actions to take when the rule is matched
    pub actions: Vec<PageRuleAction>,
    /// Targets to evaluate on each request
    pub targets: Vec<PageRuleTarget>,
    /// Status of the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PageRuleStatus>,
    /// Priority of the rule (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,
}

/// Parameters for updating a page rule
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UpdatePageRuleParams {
    /// The set of actions to take when the rule is matched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<PageRuleAction>>,
    /// Targets to evaluate on each request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<PageRuleTarget>>,
    /// Status of the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PageRuleStatus>,
    /// Priority of the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,
}

/// Common Page Rule Actions
impl PageRuleAction {
    /// Always Online action
    pub fn always_online(enabled: bool) -> Self {
        Self {
            id: "always_online".to_string(),
            value: Some(PageRuleActionValue::String(if enabled { "on" } else { "off" }.to_string())),
        }
    }

    /// Browser Cache TTL action
    pub fn browser_cache_ttl(seconds: u32) -> Self {
        Self {
            id: "browser_cache_ttl".to_string(),
            value: Some(PageRuleActionValue::Number(seconds)),
        }
    }

    /// Browser Check action
    pub fn browser_check(enabled: bool) -> Self {
        Self {
            id: "browser_check".to_string(),
            value: Some(PageRuleActionValue::String(if enabled { "on" } else { "off" }.to_string())),
        }
    }

    /// Cache Level action
    pub fn cache_level(level: &str) -> Self {
        Self {
            id: "cache_level".to_string(),
            value: Some(PageRuleActionValue::String(level.to_string())),
        }
    }

    /// Custom Cache Key action
    pub fn custom_cache_key(config: serde_json::Value) -> Self {
        Self {
            id: "cache_key_fields".to_string(),
            value: Some(PageRuleActionValue::Object(config)),
        }
    }

    /// Disable Apps action
    pub fn disable_apps() -> Self {
        Self {
            id: "disable_apps".to_string(),
            value: None,
        }
    }

    /// Disable Performance action
    pub fn disable_performance() -> Self {
        Self {
            id: "disable_performance".to_string(),
            value: None,
        }
    }

    /// Disable Railgun action
    pub fn disable_railgun() -> Self {
        Self {
            id: "disable_railgun".to_string(),
            value: None,
        }
    }

    /// Disable Security action
    pub fn disable_security() -> Self {
        Self {
            id: "disable_security".to_string(),
            value: None,
        }
    }

    /// Edge Cache TTL action
    pub fn edge_cache_ttl(seconds: u32) -> Self {
        Self {
            id: "edge_cache_ttl".to_string(),
            value: Some(PageRuleActionValue::Number(seconds)),
        }
    }

    /// Email Obfuscation action
    pub fn email_obfuscation(enabled: bool) -> Self {
        Self {
            id: "email_obfuscation".to_string(),
            value: Some(PageRuleActionValue::String(if enabled { "on" } else { "off" }.to_string())),
        }
    }

    /// Forwarding URL action
    pub fn forwarding_url(url: &str, status_code: u32) -> Self {
        Self {
            id: "forwarding_url".to_string(),
            value: Some(PageRuleActionValue::Object(serde_json::json!({
                "url": url,
                "status_code": status_code
            }))),
        }
    }

    /// IP Geolocation action
    pub fn ip_geolocation(enabled: bool) -> Self {
        Self {
            id: "ip_geolocation".to_string(),
            value: Some(PageRuleActionValue::String(if enabled { "on" } else { "off" }.to_string())),
        }
    }

    /// Minify action
    pub fn minify(html: bool, css: bool, js: bool) -> Self {
        Self {
            id: "minify".to_string(),
            value: Some(PageRuleActionValue::Object(serde_json::json!({
                "html": if html { "on" } else { "off" },
                "css": if css { "on" } else { "off" },
                "js": if js { "on" } else { "off" }
            }))),
        }
    }

    /// Opportunistic Encryption action
    pub fn opportunistic_encryption(enabled: bool) -> Self {
        Self {
            id: "opportunistic_encryption".to_string(),
            value: Some(PageRuleActionValue::String(if enabled { "on" } else { "off" }.to_string())),
        }
    }

    /// Rocket Loader action
    pub fn rocket_loader(enabled: bool) -> Self {
        Self {
            id: "rocket_loader".to_string(),
            value: Some(PageRuleActionValue::String(if enabled { "on" } else { "off" }.to_string())),
        }
    }

    /// Security Level action
    pub fn security_level(level: &str) -> Self {
        Self {
            id: "security_level".to_string(),
            value: Some(PageRuleActionValue::String(level.to_string())),
        }
    }

    /// Server Side Excludes action
    pub fn server_side_excludes(enabled: bool) -> Self {
        Self {
            id: "server_side_excludes".to_string(),
            value: Some(PageRuleActionValue::String(if enabled { "on" } else { "off" }.to_string())),
        }
    }

    /// SSL mode action
    pub fn ssl(mode: &str) -> Self {
        Self {
            id: "ssl".to_string(),
            value: Some(PageRuleActionValue::String(mode.to_string())),
        }
    }

    /// True Client IP Header action
    pub fn true_client_ip_header(enabled: bool) -> Self {
        Self {
            id: "true_client_ip_header".to_string(),
            value: Some(PageRuleActionValue::String(if enabled { "on" } else { "off" }.to_string())),
        }
    }

    /// WAF action
    pub fn waf(enabled: bool) -> Self {
        Self {
            id: "waf".to_string(),
            value: Some(PageRuleActionValue::String(if enabled { "on" } else { "off" }.to_string())),
        }
    }
}

/// Common Page Rule Targets
impl PageRuleTarget {
    /// URL target with matches operator
    pub fn url_matches(pattern: &str) -> Self {
        Self {
            target: "url".to_string(),
            constraint: PageRuleConstraint {
                operator: "matches".to_string(),
                value: pattern.to_string(),
            },
        }
    }

    /// URL target with equals operator
    pub fn url_equals(url: &str) -> Self {
        Self {
            target: "url".to_string(),
            constraint: PageRuleConstraint {
                operator: "equals".to_string(),
                value: url.to_string(),
            },
        }
    }

    /// URL target with contains operator
    pub fn url_contains(substring: &str) -> Self {
        Self {
            target: "url".to_string(),
            constraint: PageRuleConstraint {
                operator: "contains".to_string(),
                value: substring.to_string(),
            },
        }
    }
}

// ApiResult trait implementations
impl ApiResult for PageRule {}
impl ApiResult for Vec<PageRule> {}