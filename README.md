# Shodan-rs

A shodan client written in rust (if that wasn't clear yet). Heavily WIP so docs are TODO :-(

## API support
The official shodan API documentation can be found [here](https://developer.shodan.io/api).
This client does not support the entire API as I only have an account with a `dev` plan.
You can review the support table below.

### API support
| Realm | API          | VERB   | Endpoint                                              |       Support        |
|-------|--------------|--------|-------------------------------------------------------|:--------------------:|
| REST  | Search       | GET    | /shodan/host/ip/                                      | :white_large_square: |
| REST  | Search       | GET    | /shodan/host/count/                                   | :white_large_square: |
| REST  | Search       | GET    | /shodan/host/search/                                  | :white_large_square: |
| REST  | Search       | GET    | /shodan/host/search/facets                            | :white_large_square: |
| REST  | Search       | GET    | /shodan/host/search/filters                           | :white_large_square: |
| REST  | Search       | GET    | /shodan/host/search/tokens                            | :white_large_square: |
| REST  | Scanning     | GET    | /shodan/ports                                         | :white_large_square: |
| REST  | Scanning     | GET    | /shodan/protocols                                     | :white_large_square: |
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
| REST  | Directory    | GET    | /shodan/query                                         | :white_large_square: |
| REST  | Directory    | GET    | /shodan/query/search                                  | :white_large_square: |
| REST  | Directory    | GET    | /shodan/query/tags                                    | :white_large_square: |
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