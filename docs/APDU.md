# APDU protocol description

This document aims to provide a description of the APDU protocol supported by the app, explaining what each instruction does, the expected parameters and return values

## General Structure

The general structure of a reqeuest and response is as follows:

### Request / Command

| Field   | Type     | Content                | Note                   |
|:--------|:---------|:-----------------------|------------------------|
| CLA     | byte (1) | Application Identifier | 0xFF, To be determined |
| INS     | byte (1) | Instruction ID         |                        |
| P1      | byte (1) | Parameter 1            |                        |
| P2      | byte (1) | Parameter 2            |                        |
| L       | byte (1) | Bytes in payload       |                        |
| PAYLOAD | byte (L) | Payload                |                        |

### Response

| Field   | Type     | Content     | Note                     |
| ------- | -------- | ----------- | ------------------------ |
| ANSWER  | byte (?) | Answer      | depends on the command   |
| SW1-SW2 | byte (2) | Return code | see list of return codes |

#### Return codes

| Return code | Description             |
| ----------- | ----------------------- |
| 0x6400      | Execution Error         |
| 0x6982      | Empty buffer            |
| 0x6983      | Output buffer too small |
| 0x6986      | Command not allowed     |
| 0x6D00      | INS not supported       |
| 0x6E00      | CLA not supported       |
| 0x6F00      | Unknown                 |
| 0x9000      | Success                 |

---

## Commands definitions

### GetVersion

This command will return the app version

#### Command

| Field | Type     | Content                | Expected |
|-------|----------|------------------------|----------|
| CLA   | byte (1) | Application Identifier |          |
| INS   | byte (1) | Instruction ID         | 0x00     |
| P1    | byte (1) | Parameter 1            | ignored  |
| P2    | byte (1) | Parameter 2            | ignored  |
| L     | byte (1) | Bytes in payload       | 0        |

#### Response

| Field     | Type     | Content          | Note                            |
| --------- | -------- | ---------------- | ------------------------------- |
| TEST      | byte (1) | Test Mode        | 0xFF means test mode is enabled |
| MAJOR     | byte (1) | Version Major    |                                 |
| MINOR     | byte (1) | Version Minor    |                                 |
| PATCH     | byte (1) | Version Patch    |                                 |
| LOCKED    | byte (1) | Device is locked |                                 |
| TARGET ID | byte (4) | Target ID        |                                 |
| SW1-SW2   | byte (2) | Return code      | see list of return codes        |

### GetAddress

This command returns the public key corresponding to the secret key found at the given bip32 path, see below. 

#### Command

| Field   | Type     | Content                   | Expected        |
|---------|----------|---------------------------|-----------------|
| CLA     | byte (1) | Application Identifier    |                 |
| INS     | byte (1) | Instruction ID            | 0x01            |
| P1      | byte (1) | Request User confirmation | No = 0          |
| P2      | byte (1) | ignored                   |                 |
| L       | byte (1) | Bytes in payload          | (depends)       |
| PathN   | byte (1) | Number of path components | ? (typically 4) |
| Path[0] | byte (4) | Derivation Path Data      | 0x80000A55      |
| Path[1] | byte (4) | Derivation Path Data      | 0xA2862AD3      |
| Path[2] | byte (4) | Derivation Path Data      | ?               |
| Path[3] | byte (4) | Derivation Path Data      | ?               |
| Path[4] | byte (4) | Derivation Path Data      | ?               |

#### Response

| Field      | Type      | Content           | Note                     |
| ---------- | --------- | ----------------- | ------------------------ |
| PK_LEN     | byte (1)  | Bytes in PKEY     |                          |
| PKEY       | byte (??) | Public key bytes  |                          |
| SW1-SW2    | byte (2)  | Return code       | see list of return codes |

### Sign

This command will return a signature of the passed payload hashed with SHA256

#### Command

| Field | Type     | Content                | Expected          |
|-------|----------|------------------------|-------------------|
| CLA   | byte (1) | Application Identifier |                   |
| INS   | byte (1) | Instruction ID         | 0x02              |
| P1    | byte (1) | Payload desc           | 0 = init          |
|       |          |                        | 1 = add           |
|       |          |                        | 2 = last          |
| P2    | byte (1) | ignored                |                   |
| L     | byte (1) | Bytes in payload       | (depends)         |

The first packet/chunk includes only the derivation path

All other packets/chunks contain data chunks that are described below

_First Packet_

| Field   | Type     | Content                   | Expected        |
|---------|----------|---------------------------|-----------------|
| PathN   | byte (1) | Number of path components | ? (typically 4) |
| Path[0] | byte (4) | Derivation Path Data      | 0x80000A55      |
| Path[1] | byte (4) | Derivation Path Data      | 0xA2862AD3      |
| Path[2] | byte (4) | Derivation Path Data      | ?               |
| Path[3] | byte (4) | Derivation Path Data      | ?               |
| Path[4] | byte (4) | Derivation Path Data      | ?               |

_Other Chunks/Packets_

| Field | Type     | Content | Expected |
| ----- | -------- | ------- | -------- |
| Data  | bytes... | Message |          |

Data is defined as:

| Field   | Type    | Content      | Expected |
| ------- | ------- | ------------ | -------- |
| Message | bytes.. | Data to sign |          |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SIG_HASH | byte (32) | Signed hash | SHA256 hash used as signature message |
| SIG      | byte (65) | Signature   | RSV encoded signature                 |
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |
