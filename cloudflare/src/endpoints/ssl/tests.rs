#[cfg(test)]
mod tests {
    use super::super::{
        CertificateRequestType, CertificateValidity, CreateOriginCaCertificate,
        CreateOriginCaCertificateParams, GetOriginCaCertificate, GetZoneSslSettings,
        ListOriginCaCertificates, OriginCaCertificate, RevokeOriginCaCertificate, SslMode,
        UpdateSslSettingsParams, UpdateZoneSslSettings, ZoneSslSettings,
    };
    use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
    use chrono::{DateTime, Utc};

    #[test]
    fn test_origin_ca_certificate_serialization() {
        let certificate = OriginCaCertificate {
            id: "cert-123".to_string(),
            csr: "-----BEGIN CERTIFICATE REQUEST-----\nMIICijCCAXICAQAwRTELMAkGA1UEBhMCQVUxEzARBgNVBAgMClNvbWUtU3RhdGUxITAfBgNVBAoMGEludGVybmV0IFdpZGdpdHMgUHR5IEx0ZA==\n-----END CERTIFICATE REQUEST-----".to_string(),
            hostnames: vec!["example.com".to_string(), "*.example.com".to_string()],
            expires_on: DateTime::parse_from_rfc3339("2024-12-31T23:59:59Z")
                .unwrap()
                .with_timezone(&Utc),
            request_type: "origin-rsa".to_string(),
            certificate_authority: Some("Cloudflare Origin CA".to_string()),
            certificate: "-----BEGIN CERTIFICATE-----\nMIICijCCAXICAQAwRTELMAkGA1UEBhMCQVUxEzARBgNVBAgMClNvbWUtU3RhdGUxITAfBgNVBAoMGEludGVybmV0IFdpZGdpdHMgUHR5IEx0ZA==\n-----END CERTIFICATE-----".to_string(),
            signature: Some("sha256WithRSAEncryption".to_string()),
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
            status: Some("active".to_string()),
        };

        let json = serde_json::to_string(&certificate).unwrap();
        let deserialized: OriginCaCertificate = serde_json::from_str(&json).unwrap();

        assert_eq!(certificate, deserialized);
        assert_eq!(certificate.id, "cert-123");
        assert_eq!(certificate.hostnames.len(), 2);
        assert_eq!(certificate.request_type, "origin-rsa");
    }

    #[test]
    fn test_certificate_request_types() {
        // Test serialization
        let rsa_type = CertificateRequestType::OriginRsa;
        let ecc_type = CertificateRequestType::OriginEcc;
        let keyless_type = CertificateRequestType::KeylessCertificate;

        assert_eq!(rsa_type.to_string(), "origin-rsa");
        assert_eq!(ecc_type.to_string(), "origin-ecc");
        assert_eq!(keyless_type.to_string(), "keyless-certificate");

        // Test JSON serialization
        let rsa_json = serde_json::to_string(&rsa_type).unwrap();
        let ecc_json = serde_json::to_string(&ecc_type).unwrap();
        let keyless_json = serde_json::to_string(&keyless_type).unwrap();

        assert_eq!(rsa_json, "\"origin-rsa\"");
        assert_eq!(ecc_json, "\"origin-ecc\"");
        assert_eq!(keyless_json, "\"keyless-certificate\"");
    }

    #[test]
    fn test_certificate_validity_constants() {
        assert_eq!(CertificateValidity::WEEK, 7);
        assert_eq!(CertificateValidity::MONTH, 30);
        assert_eq!(CertificateValidity::THREE_MONTHS, 90);
        assert_eq!(CertificateValidity::YEAR, 365);
        assert_eq!(CertificateValidity::TWO_YEARS, 730);
        assert_eq!(CertificateValidity::FIFTEEN_YEARS, 5475);
    }

    #[test]
    fn test_create_certificate_params_helpers() {
        let csr = "-----BEGIN CERTIFICATE REQUEST-----\ntest\n-----END CERTIFICATE REQUEST-----".to_string();
        let hostnames = vec!["example.com".to_string(), "*.example.com".to_string()];

        // Test RSA certificate creation
        let rsa_params = CreateOriginCaCertificateParams::new_rsa(csr.clone(), hostnames.clone());
        assert_eq!(rsa_params.request_type, Some("origin-rsa".to_string()));
        assert_eq!(rsa_params.requested_validity, Some(CertificateValidity::YEAR));
        assert_eq!(rsa_params.hostnames, hostnames);

        // Test ECC certificate creation
        let ecc_params = CreateOriginCaCertificateParams::new_ecc(csr.clone(), hostnames.clone());
        assert_eq!(ecc_params.request_type, Some("origin-ecc".to_string()));

        // Test keyless certificate creation
        let keyless_params = CreateOriginCaCertificateParams::new_keyless(csr.clone(), hostnames.clone());
        assert_eq!(keyless_params.request_type, Some("keyless-certificate".to_string()));

        // Test custom validity
        let custom_validity = rsa_params.with_validity(CertificateValidity::TWO_YEARS);
        assert_eq!(custom_validity.requested_validity, Some(CertificateValidity::TWO_YEARS));
    }

    #[test]
    fn test_ssl_mode_serialization() {
        let modes = vec![
            SslMode::Off,
            SslMode::Flexible,
            SslMode::Full,
            SslMode::FullStrict,
        ];

        let json = serde_json::to_string(&modes).unwrap();
        let deserialized: Vec<SslMode> = serde_json::from_str(&json).unwrap();

        assert_eq!(modes, deserialized);

        // Test individual mode serialization
        let off_json = serde_json::to_string(&SslMode::Off).unwrap();
        let flexible_json = serde_json::to_string(&SslMode::Flexible).unwrap();
        let full_json = serde_json::to_string(&SslMode::Full).unwrap();
        let strict_json = serde_json::to_string(&SslMode::FullStrict).unwrap();

        assert_eq!(off_json, "\"off\"");
        assert_eq!(flexible_json, "\"flexible\"");
        assert_eq!(full_json, "\"full\"");
        assert_eq!(strict_json, "\"full_strict\"");
    }

    #[test]
    fn test_zone_ssl_settings_serialization() {
        let ssl_settings = ZoneSslSettings {
            value: SslMode::FullStrict,
            modified_on: Some(
                DateTime::parse_from_rfc3339("2023-06-01T12:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            editable: Some(true),
        };

        let json = serde_json::to_string(&ssl_settings).unwrap();
        let deserialized: ZoneSslSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(ssl_settings, deserialized);
        assert_eq!(ssl_settings.value, SslMode::FullStrict);
        assert_eq!(ssl_settings.editable, Some(true));
    }

    #[test]
    fn test_list_origin_ca_certificates_endpoint() {
        // Test without zone filter
        let list_request = ListOriginCaCertificates { zone_id: None };

        assert_eq!(list_request.method(), Method::GET);
        assert_eq!(list_request.path(), "certificates");
        assert_eq!(list_request.query(), None);

        // Test with zone filter
        let list_request_with_zone = ListOriginCaCertificates {
            zone_id: Some("zone-abc123".to_string()),
        };

        assert_eq!(list_request_with_zone.method(), Method::GET);
        assert_eq!(list_request_with_zone.path(), "certificates");

        let query = list_request_with_zone.query();
        assert!(query.is_some());
        assert_eq!(query.unwrap(), "zone_id=zone-abc123");
    }

    #[test]
    fn test_get_origin_ca_certificate_endpoint() {
        let get_request = GetOriginCaCertificate {
            certificate_id: "cert-def456",
        };

        assert_eq!(get_request.method(), Method::GET);
        assert_eq!(get_request.path(), "certificates/cert-def456");
    }

    #[test]
    fn test_create_origin_ca_certificate_endpoint() {
        let create_params = CreateOriginCaCertificateParams {
            csr: "-----BEGIN CERTIFICATE REQUEST-----\ntest\n-----END CERTIFICATE REQUEST-----".to_string(),
            hostnames: vec!["test.example.com".to_string()],
            request_type: Some("origin-rsa".to_string()),
            requested_validity: Some(365),
        };

        let create_request = CreateOriginCaCertificate { params: create_params };

        assert_eq!(create_request.method(), Method::POST);
        assert_eq!(create_request.path(), "certificates");

        let body = create_request.body();
        assert!(body.is_some());
        if let Some(RequestBody::Json(json)) = body {
            assert!(json.contains("BEGIN CERTIFICATE REQUEST"));
            assert!(json.contains("test.example.com"));
            assert!(json.contains("origin-rsa"));
            assert!(json.contains("\"requested_validity\":365"));
        }
    }

    #[test]
    fn test_revoke_origin_ca_certificate_endpoint() {
        let revoke_request = RevokeOriginCaCertificate {
            certificate_id: "cert-revoke123",
        };

        assert_eq!(revoke_request.method(), Method::DELETE);
        assert_eq!(revoke_request.path(), "certificates/cert-revoke123");
    }

    #[test]
    fn test_get_zone_ssl_settings_endpoint() {
        let get_ssl_request = GetZoneSslSettings {
            zone_id: "zone-ssl123",
        };

        assert_eq!(get_ssl_request.method(), Method::GET);
        assert_eq!(get_ssl_request.path(), "zones/zone-ssl123/settings/ssl");
    }

    #[test]
    fn test_update_zone_ssl_settings_endpoint() {
        let update_params = UpdateSslSettingsParams {
            value: SslMode::FullStrict,
        };

        let update_ssl_request = UpdateZoneSslSettings {
            zone_id: "zone-update-ssl456",
            params: update_params,
        };

        assert_eq!(update_ssl_request.method(), Method::PATCH);
        assert_eq!(
            update_ssl_request.path(),
            "zones/zone-update-ssl456/settings/ssl"
        );

        let body = update_ssl_request.body();
        assert!(body.is_some());
        if let Some(RequestBody::Json(json)) = body {
            assert!(json.contains("\"value\":\"full_strict\""));
        }
    }

    #[test]
    fn test_create_certificate_params_serialization() {
        let params = CreateOriginCaCertificateParams {
            csr: "test-csr".to_string(),
            hostnames: vec!["example.com".to_string(), "*.example.com".to_string()],
            request_type: Some("origin-ecc".to_string()),
            requested_validity: Some(CertificateValidity::TWO_YEARS),
        };

        let json = serde_json::to_string(&params).unwrap();
        let deserialized: CreateOriginCaCertificateParams = serde_json::from_str(&json).unwrap();

        assert_eq!(params, deserialized);
        assert_eq!(params.hostnames.len(), 2);
        assert_eq!(params.request_type, Some("origin-ecc".to_string()));
        assert_eq!(params.requested_validity, Some(730));
    }

    #[test]
    fn test_endpoint_paths_consistency() {
        let cert_id = "test-cert-123";
        let zone_id = "test-zone-456";

        // Certificate endpoints
        let list_certs = ListOriginCaCertificates { zone_id: None };
        assert_eq!(list_certs.path(), "certificates");

        let get_cert = GetOriginCaCertificate {
            certificate_id: cert_id,
        };
        assert_eq!(get_cert.path(), "certificates/test-cert-123");

        let create_cert = CreateOriginCaCertificate {
            params: CreateOriginCaCertificateParams::new_rsa(
                "test-csr".to_string(),
                vec!["test.com".to_string()],
            ),
        };
        assert_eq!(create_cert.path(), "certificates");

        let revoke_cert = RevokeOriginCaCertificate {
            certificate_id: cert_id,
        };
        assert_eq!(revoke_cert.path(), "certificates/test-cert-123");

        // SSL settings endpoints
        let get_ssl = GetZoneSslSettings { zone_id };
        assert_eq!(get_ssl.path(), "zones/test-zone-456/settings/ssl");

        let update_ssl = UpdateZoneSslSettings {
            zone_id,
            params: UpdateSslSettingsParams {
                value: SslMode::Full,
            },
        };
        assert_eq!(update_ssl.path(), "zones/test-zone-456/settings/ssl");
    }
}