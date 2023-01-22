use crate::search::Facet;
use serde::de::Visitor;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub matches: Vec<Match>,
    pub total: i64,
    pub facets: Option<HashMap<String, Vec<Facet>>>,
}

#[derive(Debug, Deserialize)]
pub struct Match {
    pub hash: i64,
    pub asn: String,
    pub http: Option<Http>,
    pub os: Option<String>,
    pub tags: Option<Vec<String>>,
    pub timestamp: String,
    pub isp: String,
    pub transport: String,
    #[serde(rename = "_shodan")]
    pub shodan: Shodan,
    pub ssl: Option<Ssl>,
    pub cloud: Option<Cloud>,
    pub hostnames: Vec<String>,
    pub location: LocationClass,
    pub ip: Option<i64>,
    pub domains: Vec<String>,
    pub org: String,
    pub data: String,
    pub port: i64,
    pub ip_str: String,
    pub product: Option<String>,
    pub cpe23: Option<Vec<String>>,
    pub cpe: Option<Vec<String>>,
    pub version: Option<String>,
    //pub mysql: Option<Mysql>,
    pub info: Option<String>,
    //pub vulns: Option<HashMap<String, Vuln>>,
    pub ipv6: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Cloud {
    pub region: Option<String>,
    pub service: Option<String>,
    pub provider: String,
}

#[derive(Debug, Deserialize)]
pub struct Http {
    pub status: i64,
    pub robots_hash: Option<i64>,
    pub redirects: Vec<Redirect>,
    pub securitytxt: Option<String>,
    pub title: Option<String>,
    pub sitemap_hash: Option<i64>,
    pub robots: Option<String>,
    pub server: Option<String>,
    pub headers_hash: i64,
    pub host: String,
    pub html: String,
    pub location: String,
    pub components: Option<HashMap<String, Component>>,
    pub html_hash: i64,
    pub sitemap: Option<String>,
    pub securitytxt_hash: Option<String>,
    pub favicon: Option<Favicon>,
    pub waf: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Component {
    pub categories: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Favicon {
    pub hash: i64,
    pub data: String,
    pub location: String,
}

#[derive(Debug, Deserialize)]
pub struct Redirect {
    pub host: String,
    pub data: String,
    pub location: String,
    pub html: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LocationClass {
    pub city: String,
    pub region_code: String,
    pub area_code: Option<String>,
    pub longitude: f64,
    pub latitude: f64,
    pub country_code: String,
    pub country_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Mysql {
    pub authentication_plugin: String,
    pub capabilities: i64,
    pub server_status: String,
    pub thread_id: i64,
    pub version: String,
    pub extended_server_capabilities: i64,
    pub protocol_version: i64,
    pub server_language: i64,
}

#[derive(Debug, Deserialize)]
pub struct Shodan {
    pub region: String,
    pub ptr: Option<bool>,
    pub module: String,
    pub id: String,
    pub options: HashMap<String, String>,
    pub crawler: String,
}

#[derive(Debug, Deserialize)]
pub struct Ssl {
    pub chain_sha256: Vec<String>,
    pub jarm: String,
    pub chain: Vec<String>,
    pub dhparams: Option<Dhparams>,
    pub versions: Vec<String>,
    pub acceptable_cas: Vec<String>,
    pub tlsext: Vec<Tlsext>,
    #[serde(rename = "ja3s")]
    pub ja3_s: String,
    pub cert: Cert,
    pub cipher: Cipher,
    pub trust: Trust,
    pub handshake_states: Vec<String>,
    pub alpn: Vec<String>,
    pub ocsp: Ocsp,
}

#[derive(Debug, Deserialize)]
pub struct Cert {
    pub sig_alg: String,
    pub issued: String,
    pub expires: String,
    pub expired: bool,
    pub version: i64,
    pub extensions: Vec<Extension>,
    pub fingerprint: Fingerprint,
    pub serial: f64,
    pub subject: Issuer,
    pub pubkey: Pubkey,
    pub issuer: Issuer,
}

#[derive(Debug, Deserialize)]
pub struct Extension {
    pub critical: Option<bool>,
    pub data: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Fingerprint {
    pub sha256: String,
    pub sha1: String,
}

#[derive(Debug, Deserialize)]
pub struct Issuer {
    #[serde(rename = "C")]
    pub c: Option<String>,
    #[serde(rename = "CN")]
    pub cn: String,
    #[serde(rename = "O")]
    pub o: Option<String>,
    #[serde(rename = "L")]
    pub l: Option<String>,
    #[serde(rename = "ST")]
    pub st: Option<String>,
    #[serde(rename = "OU")]
    pub ou: Option<String>,
    #[serde(rename = "emailAddress")]
    pub email_address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Pubkey {
    #[serde(rename = "type")]
    pub pubkey_type: String,
    pub bits: i64,
}

#[derive(Debug, Deserialize)]
pub struct Cipher {
    pub version: String,
    pub bits: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Dhparams {
    pub prime: String,
    pub public_key: String,
    pub bits: i64,
    pub generator: i64,
    pub fingerprint: String,
}

#[derive(Debug, Deserialize)]
pub struct Ocsp {
    pub version: Option<String>,
    pub response_status: Option<String>,
    pub responder_id: Option<String>,
    pub cert_status: Option<String>,
    pub produced_at: Option<String>,
    pub signature_algorithm: Option<String>,
    pub next_update: Option<String>,
    pub this_update: Option<String>,
    pub certificate_id: Option<CertificateId>,
}

#[derive(Debug, Deserialize)]
pub struct CertificateId {
    pub hash_algorithm: String,
    pub issuer_name_hash: String,
    pub issuer_name_key: String,
    pub serial_number: String,
}

#[derive(Debug, Deserialize)]
pub struct Tlsext {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Trust {
    pub revoked: bool,
    pub browser: Option<Browser>,
}

#[derive(Debug, Deserialize)]
pub struct Browser {
    pub mozilla: bool,
    pub apple: bool,
    pub microsoft: bool,
}

#[derive(Debug, Deserialize)]
pub struct Vuln {
    pub verified: bool,
    pub references: Vec<String>,
    pub cvss: Option<f64>,
    pub summary: String,
}
