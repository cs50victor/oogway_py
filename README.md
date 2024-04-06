# Python SDK for ngrok

[![PyPI][pypi-badge]][pypi-url]
[![Supported Versions][ver-badge]][ver-url]
[![MIT licensed][mit-badge]][mit-url]

[pypi-badge]: https://img.shields.io/pypi/v/ngrok
[pypi-url]: https://pypi.org/project/ngrok
[ver-badge]: https://img.shields.io/pypi/pyversions/ngrok.svg
[ver-url]: https://pypi.org/project/ngrok
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/ngrok/ngrok-rust/blob/main/LICENSE-MIT

`oogway-py` is the official Python SDK for `Oogway AI` that requires no binaries. Quickly allow users to gain knowledge from Master Oogway in few lines of code.

[ngrok](https://ngrok.com) is a globally distributed gateway that provides secure connectivity for applications and services running in any environment.

# Installation

The `ngrok-python` SDK can be installed from [PyPI](https://pypi.org/project/ngrok) via `pip`:

```shell
pip install ngrok
```

# Quickstart

1. [Install `ngrok-python`](#installation)
2. Export your [authtoken from the ngrok dashboard](https://dashboard.ngrok.com/get-started/your-authtoken) as `NGROK_AUTHTOKEN` in your terminal
3. Add the following code to your application to establish connectivity via the [forward method](https://github.com/ngrok/ngrok-python/blob/main/examples/ngrok-forward-minimal.py) through port `9000` on `localhost`:

    ```python
    # import ngrok python sdk
    import ngrok
    
    # Establish connectivity
    listener = ngrok.forward(9000, authtoken_from_env=True)
    
    # Output ngrok url to console
    print(f"Ingress established at {listener.url()}")
    ```

That's it! Your application should now be available through the url output in your terminal. 

> **Note**
> You can find more examples in [the examples directory](https://github.com/ngrok/ngrok-python/tree/main/examples).

# Demo

https://github.com/cs50victor/oogway_py/assets/52110451/f3eb8247-0d4f-48f7-983c-2740ed3ce5fb


