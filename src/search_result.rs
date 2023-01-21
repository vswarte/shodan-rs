use serde::{Deserialize};
use std::collections::HashMap;
use serde::de::Visitor;
use crate::search::Facet;

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    matches: Vec<Match>,
    total: i64,
    pub facets: Option<HashMap<String, Vec<Facet>>>,
}

#[derive(Debug, Deserialize)]
pub struct Match {
    hash: i64,
    asn: String,
    http: Option<Http>,
    os: Option<String>,
    tags: Option<Vec<String>>,
    timestamp: String,
    isp: String,
    transport: String,
    #[serde(rename = "_shodan")]
    shodan: Shodan,
    ssl: Option<Ssl>,
    cloud: Option<Cloud>,
    hostnames: Vec<String>,
    location: LocationClass,
    ip: Option<i64>,
    domains: Vec<String>,
    org: String,
    data: String,
    port: i64,
    ip_str: String,
    product: Option<String>,
    cpe23: Option<Vec<String>>,
    cpe: Option<Vec<String>>,
    version: Option<String>,
    // mysql: Option<Mysql>,
    info: Option<String>,
    // vulns: Option<HashMap<String, Vuln>>,
    ipv6: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Cloud {
    region: Option<String>,
    service: Option<String>,
    provider: String,
}

#[derive(Debug, Deserialize)]
pub struct Http {
    status: i64,
    robots_hash: Option<i64>,
    redirects: Vec<Redirect>,
    securitytxt: Option<String>,
    title: Option<String>,
    sitemap_hash: Option<i64>,
    robots: Option<String>,
    server: Option<String>,
    headers_hash: i64,
    host: String,
    html: String,
    location: String,
    components: Option<HashMap<String, Component>>,
    html_hash: i64,
    sitemap: Option<String>,
    securitytxt_hash: Option<String>,
    favicon: Option<Favicon>,
    waf: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Component {
    categories: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Favicon {
    hash: i64,
    data: String,
    location: String,
}

#[derive(Debug, Deserialize)]
pub struct Redirect {
    host: String,
    data: String,
    location: String,
    html: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LocationClass {
    city: String,
    region_code: String,
    area_code: Option<String>,
    longitude: f64,
    latitude: f64,
    country_code: String,
    country_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Mysql {
    authentication_plugin: String,
    capabilities: i64,
    server_status: String,
    thread_id: i64,
    version: String,
    extended_server_capabilities: i64,
    protocol_version: i64,
    server_language: i64,
}

#[derive(Debug, Deserialize)]
pub struct Shodan {
    region: String,
    ptr: Option<bool>,
    module: String,
    id: String,
    options: HashMap<String, String>,
    crawler: String,
}

#[derive(Debug, Deserialize)]
pub struct Ssl {
    chain_sha256: Vec<String>,
    jarm: String,
    chain: Vec<String>,
    dhparams: Option<Dhparams>,
    versions: Vec<String>,
    acceptable_cas: Vec<String>,
    tlsext: Vec<Tlsext>,
    #[serde(rename = "ja3s")]
    ja3_s: String,
    cert: Cert,
    cipher: Cipher,
    trust: Trust,
    handshake_states: Vec<String>,
    alpn: Vec<String>,
    ocsp: Ocsp,
}

#[derive(Debug, Deserialize)]
pub struct Cert {
    sig_alg: String,
    issued: String,
    expires: String,
    expired: bool,
    version: i64,
    extensions: Vec<Extension>,
    fingerprint: Fingerprint,
    serial: f64,
    subject: Issuer,
    pubkey: Pubkey,
    issuer: Issuer,
}

#[derive(Debug, Deserialize)]
pub struct Extension {
    critical: Option<bool>,
    data: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Fingerprint {
    sha256: String,
    sha1: String,
}

#[derive(Debug, Deserialize)]
pub struct Issuer {
    #[serde(rename = "C")]
    c: Option<String>,
    #[serde(rename = "CN")]
    cn: String,
    #[serde(rename = "O")]
    o: Option<String>,
    #[serde(rename = "L")]
    l: Option<String>,
    #[serde(rename = "ST")]
    st: Option<String>,
    #[serde(rename = "OU")]
    ou: Option<String>,
    #[serde(rename = "emailAddress")]
    email_address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Pubkey {
    #[serde(rename = "type")]
    pubkey_type: String,
    bits: i64,
}

#[derive(Debug, Deserialize)]
pub struct Cipher {
    version: String,
    bits: i64,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Dhparams {
    prime: String,
    public_key: String,
    bits: i64,
    generator: i64,
    fingerprint: String,
}

#[derive(Debug, Deserialize)]
pub struct Ocsp {
    version: Option<String>,
    response_status: Option<String>,
    responder_id: Option<String>,
    cert_status: Option<String>,
    produced_at: Option<String>,
    signature_algorithm: Option<String>,
    next_update: Option<String>,
    this_update: Option<String>,
    certificate_id: Option<CertificateId>,
}

#[derive(Debug, Deserialize)]
pub struct CertificateId {
    hash_algorithm: String,
    issuer_name_hash: String,
    issuer_name_key: String,
    serial_number: String,
}

#[derive(Debug, Deserialize)]
pub struct Tlsext {
    id: i64,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Trust {
    revoked: bool,
    browser: Option<Browser>,
}

#[derive(Debug, Deserialize)]
pub struct Browser {
    mozilla: bool,
    apple: bool,
    microsoft: bool,
}

#[derive(Debug, Deserialize)]
pub struct Vuln {
    verified: bool,
    references: Vec<String>,
    cvss: Option<f64>,
    summary: String,
}

