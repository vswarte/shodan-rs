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
| Realm | API          | Verb   | Endpoint                                              |       Support        |
|:------|:-------------|:-------|:------------------------------------------------------|:--------------------:|
| REST  | Search       | GET    | /shodan/host/{ip}                                     | :white_large_square: |
| REST  | Search       | GET    | /shodan/host/count                                    | :white_large_square: |
| REST  | Search       | GET    | /shodan/host/search                                   | :white_large_square: |
| REST  | Search       | GET    | /shodan/host/search/facets                            | :white_large_square: |
| REST  | Search       | GET    | /shodan/host/search/filters                           | :white_large_square: |
| REST  | Search       | GET    | /shodan/host/search/tokens                            | :white_large_square: |
| REST  | Scanning     | GET    | /shodan/ports                                         |  :white_check_mark:  |
| REST  | Scanning     | GET    | /shodan/protocols                                     |  :white_check_mark:  |
| REST  | Scanning     | POST   | /shodan/scan                                          | :white_large_square: |
| REST  | Scanning     | POST   | /shodan/scan/internet                                 | :white_large_square: |
| REST  | Scanning     | GET    | /shodan/scans                                         | :white_large_square: |
| REST  | Scanning     | GET    | /shodan/scan/{id}                                     | :white_large_square: |
| REST  | Alerts       | POST   | /shodan/alert                                         | :white_large_square: |
| REST  | Alerts       | GET    | /shodan/alert/{id}/info                               | :white_large_square: |
| REST  | Alerts       | GET    | /shodan/alert/{id}/info                               | :white_large_square: |
| REST  | Alerts       | DELETE | /shodan/alert/{id}                                    | :white_large_square: |
| REST  | Alerts       | POST   | /shodan/alert/{id}                                    | :white_large_square: |
| REST  | Alerts       | GET    | /shodan/alert/info                                    | :white_large_square: |
| REST  | Alerts       | GET    | /shodan/alert/triggers                                | :white_large_square: |
| REST  | Alerts       | PUT    | /shodan/alert/{id}/trigger/{trigger}                  | :white_large_square: |
| REST  | Alerts       | DELETE | /shodan/alert/{id}/trigger/{trigger}                  | :white_large_square: |
| REST  | Alerts       | PUT    | /shodan/alert/{id}/trigger/{trigger}/ignore/{service} | :white_large_square: |
| REST  | Alerts       | DELETE | /shodan/alert/{id}/trigger/{trigger}/ignore/{service} | :white_large_square: |
| REST  | Alerts       | PUT    | /shodan/alert/{id}/notifier/{notifier_id}             | :white_large_square: |
| REST  | Alerts       | DELETE | /shodan/alert/{id}/notifier/{notifier_id}             | :white_large_square: |
| REST  | Notifiers    | GET    | /notifier                                             | :white_large_square: |
| REST  | Notifiers    | GET    | /notifier/provider                                    | :white_large_square: |
| REST  | Notifiers    | POST   | /notifier                                             | :white_large_square: |
| REST  | Notifiers    | DELETE | /notifier/{id}                                        | :white_large_square: |
| REST  | Notifiers    | GET    | /notifier/{id}                                        | :white_large_square: |
| REST  | Notifiers    | PUT    | /notifier/{id}                                        | :white_large_square: |
| REST  | Directory    | GET    | /shodan/query                                         |  :white_check_mark:  |
| REST  | Directory    | GET    | /shodan/query/search                                  |  :white_check_mark:  |
| REST  | Directory    | GET    | /shodan/query/tags                                    |  :white_check_mark:  |
| REST  | Bulk         | GET    | /shodan/data                                          | :white_large_square: |
| REST  | Bulk         | GET    | /shodan/data/{dataset}                                | :white_large_square: |
| REST  | Organization | GET    | /org                                                  | :white_large_square: |
| REST  | Organization | PUT    | /org/member/{user}                                    | :white_large_square: |
| REST  | Organization | DELETE | /org/member/{user}                                    | :white_large_square: |
| REST  | Account      | GET    | /account/profile                                      |  :white_check_mark:  |
| REST  | DNS          | GET    | /dns/domain/{domain}                                  |  :white_check_mark:  |
| REST  | DNS          | GET    | /dns/resolve                                          |  :white_check_mark:  |
| REST  | DNS          | GET    | /dns/reverse                                          |  :white_check_mark:  |
| REST  | Utility      | GET    | /tools/httpheaders                                    |  :white_check_mark:  |
| REST  | Utility      | GET    | /tools/myip                                           |  :white_check_mark:  |
| REST  | API Status   | GET    | /api-info                                             |  :white_check_mark:  |