# Rust Shodan API Client [![Build Status]][actions] [![Latest Version]][crates.io] [![Docs Status]][docs] [![License Status]][license]

[Build Status]: https://img.shields.io/github/actions/workflow/status/vswarte/shodan-rs/rust.yml?branch=main
[actions]: https://github.com/vswarte/shodan-rs/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/shodan_client.svg
[crates.io]: https://crates.io/crates/shodan\_client
[Docs Status]: https://img.shields.io/docsrs/shodan-client/0.1.1
[docs]: https://docs.rs/shodan-client/latest/shodan_client/
[License Status]: https://img.shields.io/github/license/vswarte/shodan-rs
[license]: https://github.com/vswarte/shodan-rs/blob/main/LICENSE-APACHE

A shodan client written in rust (if that wasn't clear yet).

## API support
The official shodan API documentation can be found [here](https://developer.shodan.io/api).
This client does not support the entire API as I only have an account with a `dev` plan.
You can review the support table below.

## Usage

```rust
use shodan_client::*;

let client = ShodanClient::new(String::from("API-KEY-GOES-HERE"));
let account_details = client.account_profile().await.unwrap();
```

### API support

:heavy_check_mark: indicates full support for the endpoint.
:heavy_division_sign: indicates partial support for the endpoint.
Emptyness indicates no support yet for the endpoint.

| Realm | API          | Verb   | Endpoint                                              |        Support        |
|:------|:-------------|:-------|:------------------------------------------------------|:---------------------:|
| REST  | Search       | GET    | /shodan/host/{ip}                                     | :heavy_division_sign: |
| REST  | Search       | GET    | /shodan/host/count                                    |  :heavy_check_mark:   |
| REST  | Search       | GET    | /shodan/host/search                                   | :heavy_division_sign: |
| REST  | Search       | GET    | /shodan/host/search/facets                            |  :heavy_check_mark:   |
| REST  | Search       | GET    | /shodan/host/search/filters                           |  :heavy_check_mark:   |
| REST  | Search       | GET    | /shodan/host/search/tokens                            | :heavy_division_sign: |
| REST  | Scanning     | GET    | /shodan/ports                                         |  :heavy_check_mark:   |
| REST  | Scanning     | GET    | /shodan/protocols                                     |  :heavy_check_mark:   |
| REST  | Scanning     | POST   | /shodan/scan                                          |                       |
| REST  | Scanning     | POST   | /shodan/scan/internet                                 |                       |
| REST  | Scanning     | GET    | /shodan/scans                                         |                       |
| REST  | Scanning     | GET    | /shodan/scan/{id}                                     |                       |
| REST  | Alerts       | POST   | /shodan/alert                                         |                       |
| REST  | Alerts       | GET    | /shodan/alert/{id}/info                               |                       |
| REST  | Alerts       | GET    | /shodan/alert/{id}/info                               |                       |
| REST  | Alerts       | DELETE | /shodan/alert/{id}                                    |                       |
| REST  | Alerts       | POST   | /shodan/alert/{id}                                    |                       |
| REST  | Alerts       | GET    | /shodan/alert/info                                    |                       |
| REST  | Alerts       | GET    | /shodan/alert/triggers                                |                       |
| REST  | Alerts       | PUT    | /shodan/alert/{id}/trigger/{trigger}                  |                       |
| REST  | Alerts       | DELETE | /shodan/alert/{id}/trigger/{trigger}                  |                       |
| REST  | Alerts       | PUT    | /shodan/alert/{id}/trigger/{trigger}/ignore/{service} |                       |
| REST  | Alerts       | DELETE | /shodan/alert/{id}/trigger/{trigger}/ignore/{service} |                       |
| REST  | Alerts       | PUT    | /shodan/alert/{id}/notifier/{notifier_id}             |                       |
| REST  | Alerts       | DELETE | /shodan/alert/{id}/notifier/{notifier_id}             |                       |
| REST  | Notifiers    | GET    | /notifier                                             |                       |
| REST  | Notifiers    | GET    | /notifier/provider                                    |                       |
| REST  | Notifiers    | POST   | /notifier                                             |                       |
| REST  | Notifiers    | DELETE | /notifier/{id}                                        |                       |
| REST  | Notifiers    | GET    | /notifier/{id}                                        |                       |
| REST  | Notifiers    | PUT    | /notifier/{id}                                        |                       |
| REST  | Directory    | GET    | /shodan/query                                         |  :heavy_check_mark:   |
| REST  | Directory    | GET    | /shodan/query/search                                  |  :heavy_check_mark:   |
| REST  | Directory    | GET    | /shodan/query/tags                                    |  :heavy_check_mark:   |
| REST  | Bulk         | GET    | /shodan/data                                          |                       |
| REST  | Bulk         | GET    | /shodan/data/{dataset}                                |                       |
| REST  | Organization | GET    | /org                                                  |                       |
| REST  | Organization | PUT    | /org/member/{user}                                    |                       |
| REST  | Organization | DELETE | /org/member/{user}                                    |                       |
| REST  | Account      | GET    | /account/profile                                      |  :heavy_check_mark:   |
| REST  | DNS          | GET    | /dns/domain/{domain}                                  |  :heavy_check_mark:   |
| REST  | DNS          | GET    | /dns/resolve                                          |  :heavy_check_mark:   |
| REST  | DNS          | GET    | /dns/reverse                                          |  :heavy_check_mark:   |
| REST  | Utility      | GET    | /tools/httpheaders                                    |  :heavy_check_mark:   |
| REST  | Utility      | GET    | /tools/myip                                           |  :heavy_check_mark:   |
| REST  | API Status   | GET    | /api-info                                             |  :heavy_check_mark:   |

## Tests

The client includes a set of tests that call out to the actual shodan API.

### Example invocation
```shell
$ SHODAN_TEST_KEY=<API-KEY-GOES-HERE> cargo test
```
