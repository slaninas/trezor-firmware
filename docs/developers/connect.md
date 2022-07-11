# Trezor Connect

> This page was migrated from the old wiki, the content might be outdated.

Trezor Connect is a JavaScript platform made by SatoshiLabs for an easy integration
of Trezor device with third-party services (websites or
applications).

Trezor Connect can be used as:

-   Authentification tool, as used in [Sign in with Trezor](../developers/connect/login-with-trezor.md)
-   Cipherkey tool, as used in [Password manager](https://trezor.io/passwords/)
-   Cryptocurrency wallet tool, as used in [Trezor Suite](https://suite.trezor.io/) or
    [MyEtherWallet](https://www.myetherwallet.com/)

Trezor connect can be implemented in:

-   Google Chrome Extension
-   Websites (Google Chrome/Chromium/Mozilla Firefox)

## How Trezor Connect works

*Full documentation of Trezor Connect and its methods is on this GitHub
[page]*

After implementing Trezor Connect, a small file containing a declaration
of methods is downloaded. Once the Trezor Connect [method] is used,
the connection to the trezor.io external webpage is established and the
Trezor Connect library is going to be downloaded and injected as an
invisible iframe into your application. Trezor Connect is,
therefore it is provable that it is not saving any
information about the device or account. Trezor Bridge has whitelisted
domains set to "\*.trezor.io" and "localhost" and it is ignoring
messages coming from other domains. This ensures that Connect is not
providing any data without the user's consent. Trezor Connect works as a
tunnel for messages sent from your application to Trezor device via
transport layer (Trezor Bridge/WebUSB).

  [page]: https://github.com/trezor/trezor-suite/tree/develop/packages/connect
  [method]: https://github.com/trezor/trezor-suite/blob/develop/docs/packages/connect/methods.md
