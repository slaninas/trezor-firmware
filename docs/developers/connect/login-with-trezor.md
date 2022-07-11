# Login with Trezor

Trezor Connect is a platform for easy integration of **Login with
Trezor** into websites or applications.

Note: more technical and less obsolete documentation of Trezor Connect
API can be found [on GitHub].

Add Trezor Login to Your Website or App

![](Developers_guide_connect-login.png)

Trezor is known as the most secure bitcoin hardware wallet. Trezor
Connect expands its application to the most secure user authentication
device. It allows a **password-less login immune to keyloggers or
phishing** that provides a simple fluid interface for users with basic
computer skills.

Within the first 4 months since official release, the device was
embraced by users in over 70 countries and greatly reviewed by security
experts as [the most sophisticated authenticator device in the world]

## How it works

After you implement Trezor Connect into your web page, the following
button will appear:

![](Developers_guide_connect-button.png)

When user clicks on the button, the following dialog windows will popup:

![](Developers_guide_connect-display.png)

and Trezor will show the following confirmation screen:

![](Developers_guide_connect-screen.png)

After user confirms the action, the device will return a structure with
signed login information. Your backend service just needs to check the
signature against user's public key.

## How to implement

The implementation is fairly simple and straightforward. For full
description see [our repository][on github]

## Implementation and security considerations

Trezor Connect leverages the ability of Trezor device to hold securely
secret keys and use them to sign messages without actually ever exposing
these secrets. Thus when using login with Trezor, no user password is
needed and no reusable secret can be ever captured by keylogger or other
malware.

To gain the maximum security from Trezor Connect the site should make
the implementation correctly without any unintented backdoors. There are
the following basic scenarios:

**Sign in with Trezor only** (use case A)

This is the most secure option: When user Sign in with Trezor for the
first time, the site offers him to create new account with his
site-specific Trezor identity.

There is no other way how to login and site operator should decide
carefully if he allows any other (usually much weaker) form of
authentication and Trezor identity override.

**Classic sign in along with Trezor option** (use case B)

This case occurs when Trezor is only one of more ways of accessing user
account. Usually user already has an account with username-password
authentication and he pairs Trezor with his account later.

With this option, user doesn't have to expose his orginal credentials
each time he authenticates. Nevertheless, he can still easily recover
access to the account using username & password if his Trezor identity
gets lost.

Site can also give user an option of disabling the original credentials,
thus effectively creating the first use case.

**Trezor identity recovery**

Users are instructed to keep their recovery seed safe and offline. Thus
in the event of lost device, user will still be able to login with
Trezor after he recovers the seed on a new device. He may also use
recovery and login through software, although the login through software
means is not supported by Trezor Connect at the moment.

Nevertheless, there will be also rare cases of users who will loose both
device and recovery seed. Therefore the site operator could carefully
consider reasonable and solid ways of Trezor identity override to enable
the user to access his account even without his Trezor.

  [on github]: https://github.com/trezor/trezor-suite/tree/develop/packages/connect
  [the most sophisticated authenticator device in the world]: https://www.coindesk.com/whats-next-bitcoin-wallet-security

