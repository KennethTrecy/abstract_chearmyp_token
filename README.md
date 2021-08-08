# Abstract Chearmyp Token
An abstraction of Chearmyp language represented as queue of tokens.

## Installation
Add it to the dependencies:
```
abstract_chearmyp_token = { git = "http://chearmyp.local/abstract_chearmyp_token", tag = "v0.1.0" }
```

You may also activate all the features:
```
[dependencies.abstract_chearmyp_token]
git = "http://chearmyp.local/abstract_chearmyp_token"
tag = "v0.1.0"
features = ["no_std", "vecdeque_token_queue", "assertable_token_kind"]
```

### Documentation
Run the following on the console:
```
cargo doc --all-features --open
```
