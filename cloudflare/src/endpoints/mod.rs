/*!
Implementations of the Endpoint trait for individual Cloudflare API endpoints, e.g. DNS or Workers.
If you want to add a new Cloudflare API to this crate, simply add a new submodule of this `endpoints`
module.
 */
pub mod access;
pub mod account;
pub mod ai;
pub mod argo_tunnel;
pub mod cfd_tunnel;
pub mod d1;
pub mod dns;
pub mod load_balancing;
pub mod page_rules;
pub mod queues;
pub mod r2;
pub mod ssl;
pub mod workers;
pub mod workerskv;
pub mod zones;
