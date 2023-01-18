# Shodan-rs

A shodan client written in rust (if that wasn't clear yet). This library is heavily in flux and is missing a lot of
components. I suggest waiting until the API is less in flux and there is a crate published.

## API support
The official shodan API documentation can be found [here](https://developer.shodan.io/api).
This client does not support the entire API as I only have an account with a `dev` plan.
You can review the support table below.

## Usage
First off you'll want to create a `ShodanClient`. This requires an API key from [shodan.io](https://shodan.io).
Once you're acquired your key you can spawn a client like so:
```rust
let shodan_client = ShodanClient::new(String::from("API-KEY-GOES-HERE"));
```

You can then make calls to the shodan API. As an example we can fetch our shodan account details:
```rust
let account_details_response = shodan_client.get_account_profile().unwrap();
if let ShodanClientResponse::Response(account_details) = account_details_response {
    println!("Account Details: {:?}", account_details);
}
```

Note: the `unwrap()` here glosses over a possible `reqwest::Error` that might occur if something odd happens with the 
transport itself (ex: SSL certificate errors, a general lack of connectivity, etc). You should be handling this properly if
you don't want to your program to panic. I have omitted this for the sake of brevity.

The `Result<T, E>` coming from any call to the shodan API will always wrap enum `ShodanClientResponse` which can be
either of variant `ShodanClientResponse::Response` or `ShodanClientResponse::Error`, the latter of which indicates
an error response from the shodan API itself (ex: rate limiting, plan constraits or owning insufficient credits for a
call).

### API support

:heavy_check_mark: indicates full support for the endpoint.
:heavy_division_sign: indicates partial support for the endpoint.
Emptyness indicates no support yet for the endpoint.

The client currently only supports the REST API although I do want to add support for the streaming API eventually.

| Realm | API          | Verb   | Endpoint                                              |        Support        |
|:------|:-------------|:-------|:------------------------------------------------------|:---------------------:|
| REST  | Search       | GET    | /shodan/host/{ip}                                     | :heavy_division_sign: |
| REST  | Search       | GET    | /shodan/host/count                                    |                       |
| REST  | Search       | GET    | /shodan/host/search                                   |                       |
| REST  | Search       | GET    | /shodan/host/search/facets                            |                       |
| REST  | Search       | GET    | /shodan/host/search/filters                           |                       |
| REST  | Search       | GET    | /shodan/host/search/tokens                            |                       |
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

The client includes a set of tests that call out to the actual shodan API (for the time being). Once it's all proven to
work it makes more sense mocking the responses so that testing becomes infinitely less flaky. Until then running the
tests requires a shodan API key as well. You can supply said key to the testing suite by setting the `SHODAN_TEST_KEY`
env var. Some endpoints in the test suite do use API credits. Running the tests is not free in that sense.

### Example invocation
```shell
$ SHODAN_TEST_KEY=<API-KEY-GOES-HERE> cargo test
```