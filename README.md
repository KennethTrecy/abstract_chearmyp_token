# Abstract Chearmyp Token
An abstraction of Chearmyp tokens.

## Installation
Add it to the dependencies:
```
[dependencies.abstract_chearmyp_token]
git = "https://github.com/KennethTrecy/abstract_chearmyp_token"
tag = "v0.3.0"
```

You may also activate all the features:
```
[dependencies.abstract_chearmyp_token]
git = "https://github.com/KennethTrecy/abstract_chearmyp_token"
tag = "v0.3.0"
features = ["no_std", "vecdeque_token_queue", "assertable_token_kind"]
```

## Origin
The repository was based from [`filled_toml`] branch of [Feo Template].

### Documentation
Run the following on the console:
```
cargo doc --all-features --open
```

[`filled_toml`]: https://github.com/KennethTrecy/feo_template/tree/filled_toml
[Feo Template]: https://github.com/KennethTrecy/feo_template
