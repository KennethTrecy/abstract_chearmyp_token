# Abstract Chearmyp Token
An abstraction of Chearmyp tokens.

## Installation
Add it to the dependencies:
```
[dependencies.abstract_chearmyp_token]
git = "https://github.com/KennethTrecy/abstract_chearmyp_token"
tag = "v0.4.1"
```

You may also activate all the features:
```
[dependencies.abstract_chearmyp_token]
git = "https://github.com/KennethTrecy/abstract_chearmyp_token"
tag = "v0.4.1"
features = ["no_std", "vecdeque_token_queue", "assertable_token_kind"]
```

## Origin
Some parts of the repository was based from [`filled_bare_metal`] branch of [Feo Template].

### Documentation
Run the following on the console:
```
cargo doc --all-features --open
```

### Author
Abstract Chearmyp Token was created by Kenneth Trecy Tobias.

[`filled_bare_metal`]: https://github.com/KennethTrecy/feo_template/tree/filled_bare_metal
[Feo Template]: https://github.com/KennethTrecy/feo_template
