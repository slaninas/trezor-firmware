# Trezor CTL

are tools written in the Python programming language that can be used
with the Trezor device using the command
line client.

## Install python-trezor

Python-trezor requires Python 3.5. The easiest way to install
python-trezor is with pip.

1\. Update repositories, install dependencies:

`sudo apt update && sudo apt install -y python3-pip python3-dev libusb-1.0-0-dev libudev-dev`

2\. Install setuptool and wheel

`pip3 install setuptools wheel`

3\. Install python-trezor

`pip3 install trezor`

4\. Update your $PATH by running

`source .profile`

5\. Udev rules need to be set up on your system; this can be done in two
ways:

a) by installing Trezor Bridge via https://suite.trezor.io/web/bridge/

or

b) by setting up [Udev rules](https://wiki.trezor.io/Udev_rules)

You can now use the Python tools for changing settings in the Trezor
device, signing transactions, retrieving account info and addresses and
much more!

## python-trezor version update

You can check the latest python-trezor version on this
[page](https://pypi.org/project/trezor/). To update your Python-trezor
version run:

`pip3 install -U trezor`

## Commands, options and examples

### Get address

-   *Command :* **trezorctl get-address**

<!-- -->

-   *Description:* This command returns address for a specific BIP32
    path and script type.

<!-- -->

-   *Required option:*

-n or --address BIP32 path eg."m/49'/0'/0'/0/0"

-   *Options:*

-c or --coin

Supported coin types: Bitcoin, Testnet, Bcash, Bgold, Bitcore, Bprivate,
Crown, Dash, Decred, Decred Testnet, DigiByte, Dogecoin, Feathercoin,
Flashcoin, Fujicoin, Groestlcoin, Groestlcoin Testnet, Koto, Litecoin,
Litecoin Testnet, Monacoin, MonetaryUnit, Pesetacoin, SmartCash,
Terracoin, Vertcoin, Viacoin, Zcash, Zcash Testnet, Zcoin, Zencash

-t or --script-type

You can specify which script type of address to use to get xpub

`address/segwit/p2shsegwit`

-d or --show-display

This option will display a specific address on the Trezor display.

-   *Example :*

`trezorctl get-address -n "m/49'/0'/0'/0/0" -t "p2shsegwit" -d`

### Wiping the Trezor device

-   *Command :* **trezorctl wipe-device**

<!-- -->

-   *Description :* This command resets the Trezor device to factory
    defaults and removes all private data.

If the device is not in bootloader mode, it will erase the user data. If
the device is not in bootloader mode but the option --bootloader is
called, it will tell the user to enter bootloader mode. If the device is
in bootloader mode and the option --bootloader is called, it will erase
both the user data and firmware. If the device is in bootloader mode and
the option --bootloader is not called, it will warn the user that the
device is in bootloader mode.

-   *Options :*

--help : Show command description.

-b or --bootloader : Wipe the device in the bootloader mode. This also
erases the firmware.

-   *Example:*

`trezorctl wipe-device`

### Firmware update

-   *Command:* **trezorctl firmware-update**

<!-- -->

-   *Description:* This command downloads and installs the selected
    firmware on the Trezor device, **the device has to be in bootloader
    mode.**

<!-- -->

-   *Options:*

--help : Show command description.

-f or --filename : This option lets the user set the file from which the
firmware should be installed.

You can install a specific version of the firmware (.bin file), download
it from this GitHub
[page](https://github.com/trezor/webwallet-data/tree/master/firmware)

-   *Example:*

`trezorctl firmware-update -f trezor-2.1.0.bin`

-u or --url

You directly type the url of the firmware file you want to install.

-   *Example:*

`trezorctl firmware-update -u https://github.com/trezor/webwallet-data/raw/master/firmware/2/trezor-2.1.0.bin`

-v or --version

You can specify the firmware version you want to install.

-   *Example:*

`trezorctl firmware-update -v 2.1.0`

--fingerprint

You can enter the fingerprint of the installed firmware when updating
the firmware. It will check if the fingerprint of the version matches
the entered fingerprint. If the version is not defined, the fingerprint
is compared to the latest firmware fingerprint.

-   *Example:*

For firmware 2.1.0:

`trezorctl firmware-update --fingerprint bb5b0308807b45d41d1e2ab66a468152997ad69a01099789d8a35e464cde999f`

### Backup device

-   *Command:* **trezorctl backup-device**

<!-- -->

-   *Description:* This command shows the recovery
    seed of the Trezor
    device two times. The device has to be
    initialized before using this command. Showing the recovery seed on
    the device screen allows the user to properly back up the device.

### Change PIN

-   *Command:* **trezorctl change-pin**

<!-- -->

-   *Description:* This command allows the user to change or remove the
    PIN protection of the Trezor device.

<!-- -->

-   *Options:*

-r or --remove: This option allows the user to remove PIN protection
from the device.

### Clear session

-   *Command:* **trezorctl clear-session**

<!-- -->

-   *Description:* This command clears the device session (remove cached
    PIN, passphrase, etc.).

### Disable passphrase

-   *Command:* **trezorctl disable-passphrase**

<!-- -->

-   *Description:* This command disables passphrase protection. It has
    to be confirmed by the user on the device.

### Enable passphrase

-   *Command:* **trezorctl enable-passphrase**

<!-- -->

-   *Description:* This command enables passphrase protection. It has to
    be confirmed by the user on the device.

### Initialize device

-   *Command:* **trezorctl reset-device**

<!-- -->

-   *Description:* This command initializes the device and generates a
    new seed after the new firmware is installed. It is the same process
    as the initialization of the device and creating a new wallet on
    trezor.io/start. Using this command, the user can set the length of
    the recovery seed on both devices,
    enable PIN or Passphrase
    protection, set the Trezor label, set a custom U2F
    counter or skip the backup process.

<!-- -->

-   *Options:*

-b or --backup_type

This option lets you choose between a traditional single BIP-39
Recovery seed or Shamir Backup.

Example:

initiates the devices using Shamir backup with 20-word shares. Changing
the -t option to 256 will create a wallet using 33-word shares.

-t or --strength

This option lets the user choose the version of the recovery
seed. The options are 128 bits (12 words),
192 bits (18 words) and 256 bits (24 words). The default is set to 256
bits.

-   *Example:*

This command creates a new wallet with an 18 word recovery seed.

`trezorctl reset-device -t 192`

-r or --passphrase-protection

This option enables passphrase protection after a new wallet is created.

-p or --pin-protection

This option lets the user set a PIN before a new
wallet is created.

-l or --label

This option lets the user set a label for the device before a new wallet
is created.

-   *Example:*

`trezorctl reset-device -l "MyTrezor"`

-u or --u2f-counter

This option lets the user set a custom U2F counter.

-s or --skip-backup

This option lets the user skip the recovery seed backup process -
showing the seed two times on the device screen. Using this option the
Trezor screen shows the "Needs backup" sign after a new wallet is
created.

### Safety-checks

-   *Command:* **trezorctl set safety-checks \[OPTIONS\]
    \[strict\|prompt\]**

<!-- -->

-   *Description:* Set safety check level. Set to "strict" to get the
    full Trezor security. Set to "prompt" if you want to be able to
    allow potentially unsafe actions, such as mismatching coin keys or
    extreme fees.

This is a power-user feature. Use with caution.

-   ''Example:

`trezorctl set safety-checks prompt`

### Get features

-   *Command:* **trezorctl get-features**

<!-- -->

-   *Description:* This command retrieves the device features and
    settings.

<!-- -->

-   *Example of the get-features response:*

Features (90 bytes) {

`   device_id: '3B5328CA18BAE5F0553BCD3A',`  
`   flags: 0,`  
`   initialized: True,`  
`   language: 'english',`  
`   major_version: 2,`  
`   minor_version: 1,`  
`   model: 'T',`  
`   needs_backup: False,`  
`   no_backup: False,`  
`   passphrase_cached: False,`  
`   passphrase_protection: True,`  
`   patch_version: 0,`  
`   pin_cached: True,`  
`   pin_protection: True,`  
`   revision: 8 bytes b'3f0e3a33',`  
`   unfinished_backup: False,`  
`   vendor: 'trezor.io',`

}

### Get public node

-   *Command:* **trezorctl get-public-node**

<!-- -->

-   *Description:* This command retrieves an extended public key that
    can be used to create a watch-only wallet.

<!-- -->

-   *Required option:*

-n or --address

-   *Example:*

`trezorctl get-public-node -n "m/44'/0'/0'/0/0"`

-   *Options:*

-c or --coin

Supported coin types: Bitcoin, Testnet, Bcash, Bgold, Bitcore, Bprivate,
Crown, Dash, Decred, Decred Testnet, DigiByte, Dogecoin, Feathercoin,
Flashcoin, Fujicoin, Groestlcoin, Groestlcoin Testnet, Koto, Litecoin,
Litecoin Testnet, Monacoin, MonetaryUnit, Pesetacoin, SmartCash,
Terracoin, Vertcoin, Viacoin, Zcash, Zcash Testnet, Zcoin, Zencash

-d or --show-display

This option will display the node public key on the Trezor display.

-t or --script-type

You can specify which script type of address to use to get an xpub

`address/segwit/p2shsegwit`

### List

-   *Command:* **trezorctl list**

<!-- -->

-   *Description:* This command lists all the connected Trezor devices.

### Ping

-   *Command:* **trezorctl ping**

<!-- -->

-   *Description:* This command sends a ping message to the device, to
    check if the communication between computer and device works.

<!-- -->

-   *Example:*

`trezorctl ping "ahoy!"`

-   *Options:*

-b or --button-protection

This option requires the user to confirm on the device that it received
the ping message.

-p or --pin-protection

This option requires the user to enter PIN protection before sending the
ping message to the device.

-r or --passphrase-protection

This option requires the user to enter the passphrase before sending the
ping message.

### Recovery device

-   *Command:* **trezorctl recovery-device**

<!-- -->

-   *Description:* This command will start the safe recovery workflow of
    the wallet using the recovery seed.

<!-- -->

-   *Options:*

-w or --words

This option lets the user choose the number of recovery seed words (12,
18 or 24 words).

-e or --expand

This option lets the user enter only the first four letters of each
mnemonic word (which is sufficient for a successful recovery).

-p or --pin-protection

This option lets the user set the PIN before starting the recovery
process.

-   *Example*

`trezorctl recovery-device -p`

-r or --passphrase-protection

This option lets the user enable passphrase protection before starting
the recovery process.

-l or --label

This option lets the user name the Trezor device before starting the
recovery process.

-t or --type

*scrambled* or *matrix*

This option lets the user choose between the standard (scrambled) and
advanced (matrix) recovery process.

-   *Example*

`trezorctl recovery-device -t matrix`

-d or --dry-run

This option lets the user perform dry run recovery - check if the
entered recovery seed matches the seed in the device. The result will
show if the recovery seed is valid and if it matches the seed in the
device. This option can be used with initialized as well as with
uninitialized device.

### Self test

-   *Command:* **trezorctl self-test**

<!-- -->

-   *Description:* This command works only for Trezor One and in
    bootloader mode. It performs USB, RNG, CPU and FLASH tests. The
    results will show on the device screen.

### Auto-lock-delay

-   *Commands:* **trezorctl set auto-lock-delay**

<!-- -->

-   *Description:* This command delays the default auto-lock for a
    number of seconds.

<!-- -->

-   *Example*

`trezorctl set auto-lock-delay 86400`

### Set homescreen

-   *Command:* **trezorctl set-homescreen**

<!-- -->

-   *Description:* This command lets the user change the home screen on
    the Trezor device.

<!-- -->

-   *Options:*

-f or --filename

For Trezor Model T: The picture has to be in [TOIF (Trezor optimized
image format)](../misc/toif.md) format with a size of 144x144 pixels.

For Trezor One: The picture has to be 128x64 pixels.

For more information about how to create a home screen for Trezor One,
please visit this [home screen
editor](https://trezor.github.io/homescreen-editor/).

-   *Example*

`trezorctl set-homescreen -f mynewhs.png`

### Set label

-   *Command:* **trezorctl set-label**

<!-- -->

-   *Description:* This command lets the user set a new label or change
    the label of the Trezor device.

<!-- -->

-   *Required option:*

-l or --label

-   *Example:*

renames your device to Trezor Wiki.

### Set display rotation

-   *Command:* **trezorctl set-display-rotatinon**

<!-- -->

-   *Description:* This command lets the user rotate the screen
    orientation in desirable direction.

<!-- -->

-   *Required options:* west or north or east or south

<!-- -->

-   *Example:*

### Sign message

-   *Command:* **trezorctl sign-message**

<!-- -->

-   *Description:* This command lets the user sign a message using the
    address of a given path.

<!-- -->

-   *Required option:*

-n or --address

It is necessary to specify BIP32 path with
sign-message command., eg. "m/44'/0'/0'/0/0"

-   *Example:*

`trezorctl sign-message -n "m/44'/0'/0'/0/0" "ahoy"`

-   *Options:*

-c or --coin

It is possible to specify which coin to use to sign a message. The
default is "Bitcoin".

-   *Example:*

`trezorctl sign-message -n "m/44'/0'/0'/0/0" -c "Litecoin" "ahoy"`

-t or --script-type

It is possible to specify which script type of address to use to sign a
message address/segwit/p2shsegwit

### Verify message

-   *Command:* **trezorctl verify-message**

<!-- -->

-   *Description:* This command lets the user verify a signed message
    using an address. After the command is executed, the device will ask
    the user to confirm the address and to verify the message.

<!-- -->

-   *Options:*

-c or --coin

-   *Usage:* trezorctl verify-message \[OPTIONS\] ADDRESS SIGNATURE
    MESSAGE

<!-- -->

-   *Example:*

`trezorctl verify-message 1aWUN65K8ecrc4yi4CnWWB76SaeB8CxHt H2cYhELO9G/NES63uARBidkkgV2ARD8nhCNWHjf+rcTmJreVi+OEfZzL7WvXn02ky62yDQmFFSgxzvaijYPA2ec= ahoy`

### Set passphrase source

-   *Command:* `trezorctl set passphrase enabled --force-on-device`

<!-- -->

-   *Description:* This command works only for Trezor Model T. It forces
    the passphrase entry always on the device. This is useful for using
    Trezor with wallets that can't specify where the passphrase should
    be entered on.
