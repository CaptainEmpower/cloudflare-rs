use crate::framework::auth::Credentials;
use crate::framework::client::ClientConfig;
use crate::framework::endpoint::{EndpointSpec, MultipartPart, RequestBody};
use crate::framework::response::{
    ApiErrors, ApiFailure, ApiResponse, ApiSuccess, ResponseConverter,
};
use crate::framework::{auth::AuthClient, Environment};
use reqwest::blocking::RequestBuilder;
use std::borrow::Cow;
use std::net::SocketAddr;

/// Synchronous Cloudflare API client.
// TODO: Rename to BlockingClient?
pub struct HttpApiClient {
    environment: Environment,
    credentials: Credentials,
    http_client: reqwest::blocking::Client,
}

impl HttpApiClient {
    // TODO: Rename to is_custom?
    #[cfg(feature = "mockito")]
    pub fn is_mock(&self) -> bool {
        matches!(self.environment, Environment::Custom(_))
    }
}

impl HttpApiClient {
    pub fn new(
        credentials: Credentials,
        config: ClientConfig,
        environment: Environment,
    ) -> Result<HttpApiClient, crate::framework::Error> {
        let mut builder = reqwest::blocking::Client::builder()
            .timeout(config.http_timeout)
            .default_headers(config.default_headers);

        if let Some(address) = config.resolve_ip {
            let url = url::Url::from(&environment);
            builder = builder.resolve(
                url.host_str()
                    .expect("Environment url should have a hostname"),
                SocketAddr::new(address, 443),
            );
        }
        let http_client = builder.build()?;

        Ok(HttpApiClient {
            environment,
            credentials,
            http_client,
        })
    }

    //noinspection ALL
    // TODO: This should probably just implement request for the Reqwest client itself :)
    /// Synchronously send a request to the Cloudflare API.
    pub fn request<Endpoint>(&self, endpoint: &Endpoint) -> ApiResponse<Endpoint::ResponseType>
    where
        Endpoint: EndpointSpec + Send + Sync,
        Endpoint::ResponseType: ResponseConverter<Endpoint::JsonResponse>,
    {
        // Build the request
        let mut request = self
            .http_client
            .request(endpoint.method(), endpoint.url(&self.environment));

        if let Some(body) = endpoint.body() {
            match body {
                RequestBody::Json(json) => {
                    request = request.body(json);
                }
                RequestBody::Raw(bytes) => {
                    request = request.body(bytes);
                }
                RequestBody::MultiPart(multipart) => {
                    let mut form = reqwest::blocking::multipart::Form::new();
                    for (name, part) in multipart.parts() {
                        match part {
                            MultipartPart::Text(text) => {
                                form = form.text(name, text);
                            }
                            MultipartPart::Bytes(bytes) => {
                                form = form
                                    .part(name, reqwest::blocking::multipart::Part::bytes(bytes));
                            }
                        }
                    }
                    request = request.multipart(form);
                }
            }
            // Reqwest::RequestBuilder::multipart sets the content type for us.
            match endpoint.content_type() {
                None | Some(Cow::Borrowed("multipart/form-data")) => {}
                Some(content_type) => {
                    request = request.header(reqwest::header::CONTENT_TYPE, content_type.as_ref());
                }
            }
        }

        request = request.auth(&self.credentials);
        let response = request.send()?;

        // The condition is necessary, even if a warning is present.
        // The constant is overridden in some cases.
        if Endpoint::IS_RAW_BODY {
            let content_type = response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|ct| ct.to_str().ok())
                .unwrap_or("");
            assert_eq!(content_type, "application/octet-stream");

            map_api_response_raw::<Endpoint>(response)
        } else {
            map_api_response_json::<Endpoint>(response)
        }
    }
}

impl AuthClient for RequestBuilder {
    fn auth(mut self, credentials: &Credentials) -> Self {
        for (k, v) in credentials.headers() {
            self = self.header(k, v);
        }
        self
    }
}

// If the response is 2XX and parses, return Success.
// If the response is 2XX and doesn't parse, return Invalid.
// If the response isn't 2XX, return Failure, with API errors if they were included.
fn map_api_response_raw<Endpoint>(
    resp: reqwest::blocking::Response,
) -> Result<Endpoint::ResponseType, ApiFailure>
where
    Endpoint: EndpointSpec,
    Endpoint::ResponseType: ResponseConverter<Endpoint::JsonResponse>,
{
    let status = resp.status();
    if status.is_success() {
        let bytes = resp.bytes().map_err(ApiFailure::Invalid)?.to_vec();
        Ok(Endpoint::ResponseType::from_raw(bytes))
    } else {
        let parsed: Result<ApiErrors, reqwest::Error> = resp.json();
        let errors = parsed.unwrap_or_default();
        Err(ApiFailure::Error(status, errors))
    }
}

fn map_api_response_json<Endpoint>(
    resp: reqwest::blocking::Response,
) -> Result<Endpoint::ResponseType, ApiFailure>
where
    Endpoint: EndpointSpec,
    Endpoint::ResponseType: ResponseConverter<Endpoint::JsonResponse>,
{
    let status = resp.status();
    if status.is_success() {
        let parsed: Result<ApiSuccess<Endpoint::JsonResponse>, reqwest::Error> = resp.json();
        match parsed {
            Ok(success) => Ok(Endpoint::ResponseType::from_json(success)),
            Err(e) => Err(ApiFailure::Invalid(e)),
        }
    } else {
        let parsed: Result<ApiErrors, reqwest::Error> = resp.json();
        let errors = parsed.unwrap_or_default();
        Err(ApiFailure::Error(status, errors))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::endpoint::{MultipartBody, MultipartPart};
    use crate::framework::response::ApiSuccess;
    use crate::framework::Environment;
    use serde_json::json;

    //region Endpoint that sends a multipart request.
    #[derive(Debug)]
    struct DummyMultipartEndpoint;

    impl EndpointSpec for DummyMultipartEndpoint {
        type JsonResponse = ();
        type ResponseType = ApiSuccess<Self::JsonResponse>;

        fn method(&self) -> reqwest::Method {
            reqwest::Method::POST
        }

        fn path(&self) -> String {
            "/dummy/multipart".into()
        }

        fn body(&self) -> Option<RequestBody> {
            Some(RequestBody::MultiPart(&DummyMultipart))
        }
    }

    struct DummyMultipart;

    impl MultipartBody for DummyMultipart {
        fn parts(&self) -> Vec<(String, MultipartPart)> {
            vec![("key".into(), MultipartPart::Text("value".into()))]
        }
    }
    //endregion

    #[cfg(feature = "mockito")]
    fn create_test_client(url: String) -> HttpApiClient {
        let environment = Environment::Custom(url);
        let credentials = Credentials::UserAuthToken {
            token: "test_token".to_string(),
        };
        let config = ClientConfig::default();
        HttpApiClient::new(credentials, config, environment).unwrap()
    }

    /// Test that the blocking client can successfully send a multipart request.
    #[cfg(feature = "mockito")]
    #[test]
    fn test_multipart_body_success() {
        let body = json!({
            "result": null,
            "result_info": null,
            "success": true,
            "errors": [],
            "messages": []
        });

        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/dummy/multipart")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body.to_string())
            .create();

        let client = create_test_client(server.url());
        let result = client.request(&DummyMultipartEndpoint);

        mock.assert();
        assert!(result.is_ok());
    }
}
