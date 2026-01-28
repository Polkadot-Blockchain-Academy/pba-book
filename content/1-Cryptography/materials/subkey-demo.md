# Subkey Signature and HDKD (Hierarchical Deterministic Key Derivation) Demo

All the subkey examples also exist in [a jupyter notebook](./signature-demo.ipynb) for reference.
As an alternative, here are subkey examples to compliment/replace using the REPL.

## Key Generation

```sh
subkey generate

Secret phrase: s      desert piano add owner tuition tail melt rally height faint thunder immune
  Network ID:        substrate
  Secret seed:       0x6a0ea68072cfd0ffbabb40801570fa5e9f3a88966eaed9dedaeb0cf140b9cd8d
  Public key (hex):  0x7acdc47530002fbc50f413859093b7df90c27874aee732dca940ea4842751d58
  Account ID:        0x7acdc47530002fbc50f413859093b7df90c27874aee732dca940ea4842751d58
  Public key (SS58): 5Eqipnpt5asTm7sCFWQeJjsNJX5cYVJMid3zjKHjDUGKBJTo
  SS58 Address:      5Eqipnpt5asTm7sCFWQeJjsNJX5cYVJMid3zjKHjDUGKBJTo
```

## Sign

```sh
echo -n 'Hello Polkadot Blockchain Academy' | subkey sign --suri 'desert piano add owner tuition tail melt rally height faint thunder immune'
```

> Note, this changes each execution, this is one viable signature: `f261d56b80e4b53c70dd2ba1de6b9384d85a8f4c6d912fd86acab3439a47992aa85ded04ac55c7525082dcbc815001cd5cc94ec1a907bbd8e3138cfc8a382683`

## Verify

```sh
echo -n 'Hello Polkadot Blockchain Academy' | subkey verify  '0xf261d56b80e4b53c70dd2ba1de6b9384d85a8f4c6d912fd86acab3439a47992aa85ded04ac55c7525082dcbc815001cd5cc94ec1a907bbd8e3138cfc8a382683' \
    '0x7acdc47530002fbc50f413859093b7df90c27874aee732dca940ea4842751d58'
```

> Expect `Signature verifies correctly.`

## Tamper with the Message

> Last char in `Public key (hex)` - AKA URI - is changed:

```sh
echo -n 'Hello Polkadot Blockchain Academy' | subkey verify \
 '0xf261d56b80e4b53c70dd2ba1de6b9384d85a8f4c6d912fd86acab3439a47992aa85ded04ac55c7525082dcbc815001cd5cc94ec1a907bbd8e3138cfc8a382683' \
    '0x7acdc47530002fbc50f413859093b7df90c27874aee732dca940ea4842751d59'
Error: SignatureInvalid
```

## Hard Derivation

```sh
subkey inspect 'desert piano add owner tuition tail melt rally height faint thunder immune//polkadot'

Secret Key URI `desert piano add owner tuition tail melt rally height faint thunder immune//polkadot` is account:
  Network ID:        substrate
 Secret seed:       0x3d764056127d0c1b4934725cb9faecf00ed0996daa84d24a903b906f319e06bf
  Public key (hex):  0xce6ccb0af417ade10062ac9b553d506b67d16c61cd2b6ce85330bc023db7e906
  Account ID:        0xce6ccb0af417ade10062ac9b553d506b67d16c61cd2b6ce85330bc023db7e906
  Public key (SS58): 5GjN3FsnqTCFZD2b1wbvJAWsVaneHbqz9HJoWeQuLFLBnwwj
  SS58 Address:      5GjN3FsnqTCFZD2b1wbvJAWsVaneHbqz9HJoWeQuLFLBnwwj
```

```sh
subkey inspect 'desert piano add owner tuition tail melt rally height faint thunder immune//kusama'

Secret Key URI `desert piano add owner tuition tail melt rally height faint thunder immune//kusama` is account:
  Network ID:        substrate
 Secret seed:       0xabd92064a63df86174acfd29ab3204897974f0a39f5d61efdd30099aa5f90bd9
  Public key (hex):  0xf62e5d444f89e704bb9b412adc472f990e9a9f40725ac6ff3abee1c9b7625a63
  Account ID:        0xf62e5d444f89e704bb9b412adc472f990e9a9f40725ac6ff3abee1c9b7625a63
  Public key (SS58): 5HdVQj5uqGYNsEFyzYb3nJ8dArhcS5BNVYhioFPQ3EptS9Yo
  SS58 Address:      5HdVQj5uqGYNsEFyzYb3nJ8dArhcS5BNVYhioFPQ3EptS9Yo
```

## Soft Derivation from Secret

```sh
subkey inspect 'desert piano add owner tuition tail melt rally height faint thunder immune//polkadot/0'

Secret Key URI `desert piano add owner tuition tail melt rally height faint thunder immune//polkadot/0` is account:
  Network ID:        substrate
 Secret seed:       n/a
  Public key (hex):  0x4e8dfdd8a386ae37b8731dba5480d5cc65739023ea24f1a09d88be1bd9dff86b
  Account ID:        0x4e8dfdd8a386ae37b8731dba5480d5cc65739023ea24f1a09d88be1bd9dff86b
  Public key (SS58): 5DqhmkscaMJRbBE7vRGtcjDySwSgGwtc611SjPZMFV2WBw51
  SS58 Address:      5DqhmkscaMJRbBE7vRGtcjDySwSgGwtc611SjPZMFV2WBw51
```

```sh
subkey inspect 'desert piano add owner tuition tail melt rally height faint thunder immune//polkadot/1'

Secret Key URI `desert piano add owner tuition tail melt rally height faint thunder immune//polkadot/1` is account:
  Network ID:        substrate
 Secret seed:       n/a
  Public key (hex):  0x2e8b3090b17b12ea63029f03d852af71570e8e526690cc271491318a45785e33
  Account ID:        0x2e8b3090b17b12ea63029f03d852af71570e8e526690cc271491318a45785e33
  Public key (SS58): 5D7jQcDVQQ8ed4skUdaSNv3noPiwy9248vHPgifNe4Hspqa4
  SS58 Address:      5D7jQcDVQQ8ed4skUdaSNv3noPiwy9248vHPgifNe4Hspqa4
```

## Soft Derivation from Public

We use addresses here because Subkey does not derive paths from a raw public key (AFAIK).

```sh
subkey inspect 5Eqipnpt5asTm7sCFWQeJjsNJX5cYVJMid3zjKHjDUGKBJTo/0

Public Key URI `5Eqipnpt5asTm7sCFWQeJjsNJX5cYVJMid3zjKHjDUGKBJTo/0` is account:
  Network ID/Version: substrate
  Public key (hex):   0xc4f4c8ee96476b56222b7fd10849f09893d4f1f7f41128d0109e3c86bd10d338
  Account ID:         0xc4f4c8ee96476b56222b7fd10849f09893d4f1f7f41128d0109e3c86bd10d338
  Public key (SS58):  5GWwxZsVJCwVuTsJP99X6Y6txonmMK3s73C4JB3DoY6UEwXq
  SS58 Address:       5GWwxZsVJCwVuTsJP99X6Y6txonmMK3s73C4JB3DoY6UEwXq
```

```sh
Public Key URI `5Eqipnpt5asTm7sCFWQeJjsNJX5cYVJMid3zjKHjDUGKBJTo/1` is account:
  Network ID/Version: substrate
  Public key (hex):   0x96450b4deeaf79aaae878c96d83b31b90375467439914c2b3a360842eba0c476
  Account ID:         0x96450b4deeaf79aaae878c96d83b31b90375467439914c2b3a360842eba0c476
  Public key (SS58):  5FTjYsLzGV2u7pudhPUpyV9A6u3ZiBKBAcuRJdeS4ZgyTYoS
  SS58 Address:       5FTjYsLzGV2u7pudhPUpyV9A6u3ZiBKBAcuRJdeS4ZgyTYoS
```
