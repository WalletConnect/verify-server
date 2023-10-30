# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## v0.31.1 - 2023-10-30
#### Bug Fixes
- update test scam domain - (83aee99) - Darksome
- - -

## v0.31.0 - 2023-10-30
#### Features
- juice up prod to 2 CPU's (#53) - (6ee1e88) - xDarksome
- - -

## v0.30.0 - 2023-10-20
#### Features
- update non-scam domain - (994406d) - Derek
- - -

## v0.29.1 - 2023-10-19
#### Bug Fixes
- CsrfToken::validate_format checking only JWT header (#52) - (ec2ec56) - xDarksome
- - -

## v0.29.0 - 2023-10-02
#### Features
- geo-blocking (#51) - (e942c90) - Xavier Basty
- - -

## v0.28.3 - 2023-09-05
#### Bug Fixes
- sanitize /index.js?token parameter (#49) - (a40075c) - xDarksome
- - -

## v0.28.2 - 2023-09-01
#### Bug Fixes
- make scam use-case look like malicious - (667bbf1) - Darksome
- - -

## v0.28.1 - 2023-09-01
#### Bug Fixes
- data_api_auth_token on cd - (c3e2162) - Darksome
- - -

## v0.28.0 - 2023-09-01
#### Features
- scam guard (#48) - (416202a) - xDarksome
- - -

## v0.27.1 - 2023-08-09
#### Bug Fixes
- adds target origin on post message (#47) - (600b7d2) - Gancho Radkov
- - -

## v0.27.0 - 2023-08-09
#### Bug Fixes
- **(ci)** Terraform CI not kicking off deploy - (7de0851) - Derek
- **(ci)** ci kicking off on .github changes - (0788641) - Derek
- cannot deploy cert - (4a48487) - Derek
#### Features
- sends message to parent page that verify has loaded (#46) - (d7ea55b) - Gancho Radkov
- - -

## v0.26.0 - 2023-08-07
#### Bug Fixes
- **(ci)** Terraform CI not kicking off deploy - (d730f41) - Derek
#### Features
- expose on .org (#45) - (cdee926) - Derek
- - -

## v0.25.1 - 2023-07-18
#### Bug Fixes
- don't use cookies for CSRF protection (#43) - (6276d0b) - xDarksome
- - -

## v0.25.0 - 2023-07-13
#### Features
- make Verify API opt-in & secure attestation with CSRF tokens (#42) - (022f45a) - xDarksome
- propagate tags to ECS tasks - (4087c5f) - Derek
- - -

## v0.24.2 - 2023-06-14
#### Bug Fixes
- remove redundant project actions (#38) - (2f48904) - Xavier Basty
- - -

## v0.24.1 - 2023-06-12
#### Bug Fixes
- **(terraform)** update hashicorp/aws version (#34) - (65ff9d1) - xDarksome
- - -

## v0.24.0 - 2023-06-12
#### Features
- Validate project IDs (#25, #33) - (bf86b33) - xDarksome
- - -

## v0.23.0 - 2023-06-09
#### Features
- add project issues workflow, update project id (#32) - (727ccbc) - Xavier Basty
- - -

## v0.22.0 - 2023-05-22
#### Bug Fixes
- update Grafana version - (d012342) - Darksome
#### Features
- **(infra)** downsize redis - (e7449eb) - Derek
- - -

## v0.21.1 - 2023-05-11
#### Bug Fixes
- don't post /attestation if attestationId is invalid (#28) - (029e130) - xDarksome
- - -

## v0.21.0 - 2023-05-08
#### Features
- Temporary disable Content-Security-Policy (#27) - (d8f3694) - xDarksome
- - -

## v0.20.0 - 2023-05-03
#### Features
- Allow localhost on prod - (f895c61) - xDarksome
- - -

## v0.19.0 - 2023-04-28
#### Features
- Impl proper CORS for {OPTIONS,GET} /attestation/{id} (#16) - (ec75150) - xDarksome
- - -

## v0.18.0 - 2023-04-26
#### Bug Fixes
- Grafana data source uids - (3b8ea46) - Darksome
#### Features
- disambiguate "project not found" 404 and "no verified domains" 404 (#21) - (b489581) - xDarksome
- - -

## v0.17.0 - 2023-04-26
#### Features
- Grafana (#17) - (3ecf2f9) - xDarksome
- - -

## v0.16.0 - 2023-04-26
#### Features
- Operational Readiness: o11y (#15) - (c6cef9b) - xDarksome
- - -

## v0.15.0 - 2023-04-24
#### Features
- Build Content-Security-Policy header based on ProjectData::verified_domains (#14) - (8e16abc) - xDarksome
#### Miscellaneous Chores
- Remove branch name from image tag in deploy.yml - (8031e5c) - xDarksome
- - -

## v0.14.0 - 2023-04-11
#### Features
- Implement project data cache (#12) - (9917d98) - xDarksome
#### Miscellaneous Chores
- Remove 'kick-off-release' from ci_terraform.yml - (632cdac) - xDarksome
- - -

## v0.13.0 - 2023-04-10
#### Features
- Validate project ids using `cerberus` (#10) - (6512709) - xDarksome
#### Miscellaneous Chores
- Skip 'release' workflow for 'chore' commits - (2649f82) - xDarksome
- - -

## v0.12.0 - 2023-04-07
#### Features
- Add hardcoded Content-Security-Policy header (#9) - (92ad690) - xDarksome
#### Miscellaneous Chores
- Add manual deploy workflow (#13) - (cb00962) - xDarksome
- Fix compiler warnings - (0bc9a8b) - xDarksome
- Fix compiler warnings - (3f423aa) - Darksome
- - -

## v0.11.0 - 2023-01-31
#### Bug Fixes
- fmt - (82fb3b3) - Derek
#### Features
- allow cors on get attestation - (61019c6) - Derek
- - -

## v0.10.1 - 2023-01-31
#### Bug Fixes
- re-adds `/attestation` POST req route - (854439d) - Gancho Radkov
- - -

## v0.10.0 - 2023-01-31
#### Features
- adds `content-type: json` to POST req headers - (81446d8) - Gancho Radkov
- - -

## v0.9.0 - 2023-01-31
#### Features
- updates gist url to include `/attestation` for POST req - (51ec9a5) - Gancho Radkov
- - -

## v0.8.0 - 2023-01-31
#### Features
- updates gist url - (d7861f8) - Gancho Radkov
- - -

## v0.7.0 - 2023-01-30
#### Features
- serve enclave - (c2859d4) - Derek
#### Miscellaneous Chores
- pin dependencies - (6f162e2) - Derek
- - -

## v0.6.0 - 2023-01-26
#### Features
- add enclave - (09d9227) - Derek
- - -

## v0.5.0 - 2023-01-25
#### Bug Fixes
- fmt - (d9fd108) - Derek
#### Features
- deploy redis - (0aa64c8) - Derek
- - -

## v0.4.0 - 2023-01-25
#### Features
- remove postgres - (bdb4923) - Derek
- - -

## v0.3.0 - 2023-01-25
#### Bug Fixes
- comment out clopp - (39ffd93) - Derek
- fmt - (5dd9c16) - Derek
#### Features
- tests and clippy - (c25b380) - Derek
- add justfile - (fef5cde) - Derek
- getting/setting attestion works - (169e627) - Derek
- integrate Redis - (2c90652) - Derek
- implement attestation endpoint - (7359e24) - Derek
- - -

## v0.2.0 - 2023-01-23
#### Bug Fixes
- wrong repository targeted - (e8dd6b6) - Derek
- terraform runs release - (4ba96da) - Derek
#### Features
- removes dependency - (976c172) - Derek
- - -

## v0.1.2 - 2023-01-23
#### Bug Fixes
- var files don't exist - (26abfc5) - Derek
- - -

## v0.1.1 - 2023-01-23
#### Bug Fixes
- fmt - (6ba6e24) - Derek
- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).