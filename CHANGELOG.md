# Changelog

This changelog documents the enhancements made to this fork compared to the original [cloudflare/cloudflare-rs](https://github.com/cloudflare/cloudflare-rs) repository.

## Enhanced Fork Features

This fork extends the official cloudflare-rs crate with comprehensive support for additional Cloudflare services that were missing or incomplete in the upstream version.

### 🆕 New Services Added

#### [v0.14.1+] Cloudflare D1 Database Management
**Added**: Complete D1 database management functionality
- **CRUD Operations**: Create, Read, Update, Delete databases
- **SQL Operations**: Execute queries, raw SQL commands with parameter binding
- **Data Import/Export**: Bulk import and export functionality
- **Query Results**: Comprehensive result handling with metadata
- **Testing**: Full test coverage for all operations

**Endpoints Added**:
- `POST /accounts/{account_id}/d1/database` - Create database
- `GET /accounts/{account_id}/d1/database` - List databases  
- `GET /accounts/{account_id}/d1/database/{database_id}` - Get database
- `PATCH /accounts/{account_id}/d1/database/{database_id}` - Update database
- `DELETE /accounts/{account_id}/d1/database/{database_id}` - Delete database
- `POST /accounts/{account_id}/d1/database/{database_id}/query` - Execute SQL
- `POST /accounts/{account_id}/d1/database/{database_id}/raw` - Raw SQL queries
- `POST /accounts/{account_id}/d1/database/{database_id}/export` - Export data
- `POST /accounts/{account_id}/d1/database/{database_id}/import` - Import data

#### [v0.14.1+] Durable Objects Namespace Management
**Added**: Complete Durable Objects namespace operations
- **Namespace Listing**: Paginated namespace discovery with filtering
- **Object Enumeration**: List objects within namespaces with cursor-based pagination
- **Metadata Access**: Comprehensive namespace and object metadata

**Endpoints Added**:
- `GET /accounts/{account_id}/workers/durable_objects/namespaces` - List DO namespaces
- `GET /accounts/{account_id}/workers/durable_objects/namespaces/{namespace_id}/objects` - List objects

#### [v0.14.1+] Enhanced R2 Bucket Operations
**Enhanced**: Extended R2 bucket management to match complete Cloudflare API
- **Complete CRUD**: All bucket lifecycle operations
- **Bucket Settings**: Location constraints, storage classes, lifecycle policies
- **Metadata Management**: Custom metadata and tagging support

**Enhanced Endpoints**:
- `POST /accounts/{account_id}/r2/buckets` - Create bucket (fixed from PUT)
- `GET /accounts/{account_id}/r2/buckets` - List buckets
- `GET /accounts/{account_id}/r2/buckets/{bucket_name}` - Get bucket details *(new)*
- `PUT /accounts/{account_id}/r2/buckets/{bucket_name}` - Update bucket *(new)*
- `DELETE /accounts/{account_id}/r2/buckets/{bucket_name}` - Delete bucket

#### [v0.14.1+] Workers Script Upload & Management
**Added**: Comprehensive Workers script deployment
- **Dual API Support**: Both legacy multipart and modern JSON APIs
- **Complete Bindings**: 17 different binding types (KV, D1, R2, Queues, etc.)
- **Module Support**: ES modules and WebAssembly module handling
- **Metadata Management**: Script metadata and configuration

**Endpoints Added**:
- `PUT /accounts/{account_id}/workers/scripts/{script_name}` - Upload script (multipart)
- `PUT /accounts/{account_id}/workers/scripts/{script_name}` - Upload script (JSON API)
- `GET /accounts/{account_id}/workers/scripts/{script_name}` - Get script
- `GET /accounts/{account_id}/workers/scripts` - List scripts

**Binding Types Supported**:
- KV Namespace, D1 Database, R2 Bucket, Queue, Service, Secret Text/JSON
- Environment Variables, WebAssembly, AI, Analytics Engine, Browser
- Constellation, Dispatch Namespace, Hyperdrive, mTLS Certificate, Rate Limit

#### [v0.14.1+] Cloudflare Queues Management
**Added**: Complete queue management and operations
- **Queue Lifecycle**: Full CRUD operations for queues
- **Consumer Management**: Worker and HTTP pull consumer support
- **Queue Operations**: Message purging and status monitoring
- **Settings Management**: Delivery delays, retention periods, visibility timeouts

**Endpoints Added**:
- `POST /accounts/{account_id}/queues` - Create queue
- `GET /accounts/{account_id}/queues` - List queues with pagination
- `GET /accounts/{account_id}/queues/{queue_id}` - Get queue details
- `PATCH /accounts/{account_id}/queues/{queue_id}` - Update queue
- `DELETE /accounts/{account_id}/queues/{queue_id}` - Delete queue
- `PUT /accounts/{account_id}/queues/{queue_id}/consumers/{consumer_id}` - Create consumer
- `POST /accounts/{account_id}/queues/{queue_id}/consumers/{consumer_id}` - Update consumer
- `GET /accounts/{account_id}/queues/{queue_id}/consumers/{consumer_id}` - Get consumer
- `DELETE /accounts/{account_id}/queues/{queue_id}/consumers/{consumer_id}` - Delete consumer
- `DELETE /accounts/{account_id}/queues/{queue_id}/messages` - Purge queue
- `GET /accounts/{account_id}/queues/{queue_id}/purge/status` - Get purge status

#### [v0.14.1+] Page Rules Management
**Added**: Complete Page Rules configuration and management
- **URL Pattern Matching**: Wildcard, exact, and contains operators
- **Action Management**: 20+ action types (cache, security, redirects, etc.)
- **Priority Control**: Rule ordering and status management
- **Filter Support**: Status, order, direction, and match type filtering

**Endpoints Added**:
- `GET /zones/{zone_id}/pagerules` - List page rules with filtering
- `GET /zones/{zone_id}/pagerules/{rule_id}` - Get page rule details
- `POST /zones/{zone_id}/pagerules` - Create page rule
- `PUT /zones/{zone_id}/pagerules/{rule_id}` - Update page rule (full)
- `PATCH /zones/{zone_id}/pagerules/{rule_id}` - Edit page rule (partial)
- `DELETE /zones/{zone_id}/pagerules/{rule_id}` - Delete page rule

#### [v0.14.1+] SSL Certificate Management
**Added**: Origin CA Certificate management and SSL settings
- **Certificate Lifecycle**: Create, list, retrieve, and revoke certificates
- **Certificate Types**: RSA, ECC, and Keyless certificate support
- **Validity Control**: Customizable certificate validity periods
- **Zone SSL Settings**: SSL mode configuration (Off, Flexible, Full, Strict)

**Endpoints Added**:
- `GET /certificates` - List Origin CA certificates
- `GET /certificates/{certificate_id}` - Get certificate details
- `POST /certificates` - Create Origin CA certificate
- `DELETE /certificates/{certificate_id}` - Revoke certificate
- `GET /zones/{zone_id}/settings/ssl` - Get SSL settings
- `PATCH /zones/{zone_id}/settings/ssl` - Update SSL settings

#### [v0.14.1+] Cloudflare Access (Zero Trust) Management
**Added**: Complete Zero Trust Access management
- **Application Management**: Self-hosted, SSH, VNC, SaaS, and Bookmark applications
- **Policy Management**: Allow, Deny, Bypass, and Non-Identity policies
- **User Management**: User sessions, authentication history, and access control
- **Service Tokens**: API authentication and token rotation
- **Rule Engine**: 11 rule types including email, IP, country, groups, and identity providers

**Endpoints Added**:
- `GET /accounts/{account_id}/access/apps` - List Access applications
- `GET /accounts/{account_id}/access/apps/{app_id}` - Get application
- `POST /accounts/{account_id}/access/apps` - Create application
- `PUT /accounts/{account_id}/access/apps/{app_id}` - Update application
- `DELETE /accounts/{account_id}/access/apps/{app_id}` - Delete application
- `GET /accounts/{account_id}/access/policies` - List policies
- `GET /accounts/{account_id}/access/policies/{policy_id}` - Get policy
- `POST /accounts/{account_id}/access/policies` - Create policy
- `PUT /accounts/{account_id}/access/policies/{policy_id}` - Update policy
- `DELETE /accounts/{account_id}/access/policies/{policy_id}` - Delete policy
- `GET /accounts/{account_id}/access/users` - List Access users
- `GET /accounts/{account_id}/access/users/{user_id}` - Get user details
- `POST /accounts/{account_id}/access/users/{user_id}/revoke_sessions` - Revoke sessions
- `GET /accounts/{account_id}/access/service_tokens` - List service tokens
- `POST /accounts/{account_id}/access/service_tokens` - Create service token
- `PUT /accounts/{account_id}/access/service_tokens/{token_id}` - Update service token
- `DELETE /accounts/{account_id}/access/service_tokens/{token_id}` - Delete service token
- `POST /accounts/{account_id}/access/service_tokens/{token_id}/rotate` - Rotate token

### 🛠️ Implementation Details

#### Architecture Patterns
- **EndpointSpec Compliance**: All new endpoints follow the established `EndpointSpec` trait pattern
- **Type Safety**: Comprehensive Rust type definitions with proper serialization/deserialization
- **Error Handling**: Proper `ApiResult` and error handling throughout
- **Testing**: Extensive test coverage (66 total tests, 15 new for Queues alone)

#### API Compliance
- **Official API Alignment**: All implementations based on official Cloudflare API documentation
- **Wrangler SDK Patterns**: Queue implementation specifically follows Wrangler SDK patterns for consistency
- **Request/Response Formats**: Proper JSON and multipart form-data support

#### Quality Assurance
- **Comprehensive Testing**: Unit tests for all data structures and endpoints
- **Serialization Testing**: JSON serialization/deserialization validation
- **Endpoint Testing**: HTTP method, path, and query parameter verification
- **Integration Ready**: All endpoints ready for real API usage

### 📦 Dependencies
No additional dependencies were introduced. All enhancements use existing crate dependencies:
- `serde` for serialization
- `chrono` for date/time handling  
- `reqwest` for HTTP client functionality

### 🎯 Use Cases Enabled

This fork enables comprehensive Cloudflare service management:

1. **Full-Stack Applications**: D1 databases + Workers + R2 storage + Queues
2. **Serverless Architectures**: Complete Workers ecosystem with all binding types
3. **Data Pipeline Management**: Queue-based message processing with D1 persistence
4. **Storage Solutions**: Enhanced R2 bucket management with metadata
5. **Microservices**: Durable Objects with proper namespace management

### 🔄 Compatibility
- **Upstream Compatible**: All original cloudflare-rs functionality preserved
- **Drop-in Replacement**: Can replace the official crate without breaking changes
- **Version Aligned**: Based on cloudflare-rs v0.14.1

### 📋 Commit History
All enhancements follow Single Responsibility Principle (SRP) with focused commits:

```
e28ea87 feat: expose new API modules in main endpoints module
bbe8ac8 feat: add module definitions for new API endpoints
2311541 test: add comprehensive test coverage for new APIs
df36533 feat: implement Cloudflare Access service token management
b20f382 feat: implement Cloudflare Access user management
80183df feat: implement Cloudflare Access policy management
eb021c5 feat: implement Cloudflare Access application management
d447eda feat: add Cloudflare Access data structures and types
ab1e47e feat: implement SSL Certificate management endpoints
16288af feat: add SSL Certificate data structures and types
90ce927 feat: implement Page Rules management endpoints
c21643a feat: add Page Rules data structures and types
5c04b3d docs: add comprehensive CHANGELOG documenting fork enhancements
467abe9 test: add multipart support test for blocking API client
4e80f0b feat: expose Cloudflare Queues module in public API
908e6b1 test: add comprehensive test coverage for Cloudflare Queues  
0da88db feat: implement Cloudflare Queues operational endpoints
1f68ca5 feat: implement Cloudflare Queues consumer management
92a0e30 feat: implement Cloudflare Queues management endpoints
17327a2 feat: add Cloudflare Queues data structures and types
7ac163d feat(cloudflare): implement real SDK calls for Workers and R2 resources
2ff4ce7 feat: implement comprehensive Workers script upload and management
ae950e0 feat: enhance R2 bucket operations to match Cloudflare API specification
a0ee995 feat: add Durable Objects namespace management endpoints
bafa417 feat: add complete D1 database management endpoints
```

---

**Note**: This fork maintains compatibility with the original cloudflare-rs while providing significant additional functionality for modern Cloudflare service usage.