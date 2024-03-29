Tip:

- In releases, you will find precompiled test apps. If you are just curious, you can run `install_s.sh` for Nano S or `install_sp.sh` for Nano SP and avoid building.

## Download and install

*Once the app is approved by Ledger, it will be available in their app store (Ledger Live).
You can get development builds generated by our CI from the release tab. THESE ARE UNVETTED DEVELOPMENT RELEASES*

Download a release from here (https://github.com/LedgerHQ/app-starknet/releases). You only need `installer_s.sh` (Nano S) or `installer_sp.sh` (Nano SP)

If the file is not executable, run
```sh
chmod +x ./installer_s.sh
```

then run:

```sh
./installer_s.sh load
```

# Development

## Preconditions

- Be sure you checkout submodules too:

    ```
    git submodule update --init --recursive
    ```

- Install Docker CE
    - Instructions can be found here: https://docs.docker.com/install/

- We only officially support Ubuntu. Install the following packages:

   ```
   sudo apt update && apt-get -y install build-essential git wget cmake \
  libssl-dev libgmp-dev autoconf libtool
   ```

- Install `node > v14.0`. We typically recommend using `n` or `nvm`

- For running unit tests you need to install a valid `rust` toolchain.
  We normally use stable 1.53 for this project

- You will need python 3 and then run
    - `make deps`

- This project requires Ledger firmware 2.0.0 
    - The current repository keeps track of Ledger's SDK but it is possible to override it by changing the git submodule.

## How to build ?

To build the app simply run:

    ``` sh
    make build
    ```

## Running tests

- Running rust tests (x64)

    If you just wish to run the rust unit and integration tests, just run:
    ```bash
    make rust_test
    ```
    ** Requires a rust toolchain available **

- Running device emulation+integration tests!!

   ```bash
    Use Zemu! Explained below!
    ```

- Running everything
  
  If you don't want to bother typing all those make commands by hand, you can skip them all!
  
  The only command you have to type is:
  ```sh
  make test_all
  ```
  
  This will initially run unit and integration tests (needs `rust` installed!), then install Zemu for you,
  clean the app's build files in case there's anything, proceed to build both application types
  and finally run the Zemu test suite.

## How to test with Zemu?

> What is Zemu?? Glad you asked!!
>
> Zemu is Zondax's testing+emulation framework for Ledger apps.
>
> Npm Package here: https://www.npmjs.com/package/@zondax/zemu
>
> Repo here: https://github.com/Zondax/zemu

Let's go! First install everything:

```bash
make zemu_install
```

Then you can run our Typescript based tests:

```bash
make zemu_test
```

To run a single specific test:

> At the moment, the recommendation is to run from the IDE. Remember to run `make build` if you change the app.

``` sh
cd zemu
yarn test -t 'test name'
```

This will run just the test maching the name provided

## Using a real device

### How to prepare your DEVELOPMENT! device:

>  You can use an emulated device for development. This is only required if you are using a physical device
>
>    **Please do not use a Ledger device with funds for development purposes.**
>
>    **Have a separate and marked device that is used ONLY for development and testing**

   There are a few additional steps that increase reproducibility and simplify development:

**1 - Ensure your device works in your OS**
- In Linux hosts it might be necessary to adjust udev rules, etc.

  Refer to Ledger documentation: https://support.ledger.com/hc/en-us/articles/115005165269-Fix-connection-issues

**2 - Set a test mnemonic**

Many of our integration tests expect the device to be configured with a known test mnemonic.

- Plug your device while pressing the right button

- Your device will show "Recovery" in the screen

- Double click

- Run `make dev_init`. This will take about 2 minutes. The device will be initialized to:

   ```
   PIN: 5555
   Mnemonic: equip will roof matter pink blind book anxiety banner elbow sun young
   ```

### 3. Add a development certificate

- Plug your device while pressing the right button

- Your device will show "Recovery" in the screen

- Click both buttons at the same time

- Enter your pin if necessary

- Run `make dev_ca`. The device will receive a development certificate to avoid constant manual confirmations.

## Building the Ledger App

### Loading into your development device

The Makefile will build the firmware in a docker container and leave the binary in the correct directory.

- Build

   ```sh
   make                # Builds the app
   ```

- Upload to a device

   The following command will upload the application to the ledger:

   _Warning: The application will be deleted before uploading._
   ```sh
   make load          # Loads the built app to the device
   ```

## APDU Specifications

- [APDU Protocol](docs/APDU.md)
