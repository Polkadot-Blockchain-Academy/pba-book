# Subkey Signature and HDKD (Hierarchical Deterministic Key Derivation) Demo

Here are subkey examples for reference on use.
Compliments the formal documentation [found here](https://github.com/paritytech/polkadot-sdk/tree/master/substrate/bin/utils/subkey#subkey).

## Key Generation

```sh
subkey generate

Secret phrase:       desert piano add owner tuition tail melt rally height faint thunder immune
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
subkey inspect 'desert piano add owner tuition tail melt rally height faint thunder immune//polkadot' --network polkadot

Secret Key URI `desert piano add owner tuition tail melt rally height faint thunder immune//polkadot` is account:
  Network ID:        polkadot
 Secret seed:       0x3d764056127d0c1b4934725cb9faecf00ed0996daa84d24a903b906f319e06bf
  Public key (hex):  0xce6ccb0af417ade10062ac9b553d506b67d16c61cd2b6ce85330bc023db7e906
  Account ID:        0xce6ccb0af417ade10062ac9b553d506b67d16c61cd2b6ce85330bc023db7e906
  Public key (SS58): 15ffBb8rhETizk36yaevSKM2MCnHyuQ8Dn3HfwQFtLMhy9io
  SS58 Address:      15ffBb8rhETizk36yaevSKM2MCnHyuQ8Dn3HfwQFtLMhy9io
```

```sh
subkey inspect 'desert piano add owner tuition tail melt rally height faint thunder immune//kusama' --network kusama

Secret Key URI `desert piano add owner tuition tail melt rally height faint thunder immune//kusama` is account:
  Network ID:        kusama
 Secret seed:       0xabd92064a63df86174acfd29ab3204897974f0a39f5d61efdd30099aa5f90bd9
  Public key (hex):  0xf62e5d444f89e704bb9b412adc472f990e9a9f40725ac6ff3abee1c9b7625a63
  Account ID:        0xf62e5d444f89e704bb9b412adc472f990e9a9f40725ac6ff3abee1c9b7625a63
  Public key (SS58): J9753RnTdZJct5RmFQ6gFVdKSyrEjzYwvYUBufMX33PB7az
  SS58 Address:      J9753RnTdZJct5RmFQ6gFVdKSyrEjzYwvYUBufMX33PB7az
```

## Soft Derivation from Secret

```sh
subkey inspect 'desert piano add owner tuition tail melt rally height faint thunder immune//polkadot/0' --network polkadot

Secret Key URI `desert piano add owner tuition tail melt rally height faint thunder immune//polkadot/0` is account:
  Network ID:        polkadot
 Secret seed:       n/a
  Public key (hex):  0x4e8dfdd8a386ae37b8731dba5480d5cc65739023ea24f1a09d88be1bd9dff86b
  Account ID:        0x4e8dfdd8a386ae37b8731dba5480d5cc65739023ea24f1a09d88be1bd9dff86b
  Public key (SS58): 12mzv68gS8Zu2iEdt4Ktkt48JZSKyFSkAVjvtgYhoa42NLNa
  SS58 Address:      12mzv68gS8Zu2iEdt4Ktkt48JZSKyFSkAVjvtgYhoa42NLNa
```

```sh
subkey inspect 'desert piano add owner tuition tail melt rally height faint thunder immune//polkadot/1' --network polkadot

Secret Key URI `desert piano add owner tuition tail melt rally height faint thunder immune//polkadot/1` is account:
  Network ID:        polkadot
 Secret seed:       n/a
  Public key (hex):  0x2e8b3090b17b12ea63029f03d852af71570e8e526690cc271491318a45785e33
  Account ID:        0x2e8b3090b17b12ea63029f03d852af71570e8e526690cc271491318a45785e33
  Public key (SS58): 1242YwUZGBQ84btGSGdSX4swf1ibfSaCDR1sr1ejC9KQ1NbJ
  SS58 Address:      1242YwUZGBQ84btGSGdSX4swf1ibfSaCDR1sr1ejC9KQ1NbJ
```

## Soft Derivation from Public

Note: We use addresses here because Subkey does not derive paths from a raw public key (AFAIK).

```sh
subkey inspect 12mzv68gS8Zu2iEdt4Ktkt48JZSKyFSkAVjvtgYhoa42NLNa/0

Public Key URI `12mzv68gS8Zu2iEdt4Ktkt48JZSKyFSkAVjvtgYhoa42NLNa/0` is account:
  Network ID/Version: polkadot
  Public key (hex):   0x40f22875159420aca51178d1baf2912c18dcb83737dd7bd39dc6743da326dd1c
  Account ID:         0x40f22875159420aca51178d1baf2912c18dcb83737dd7bd39dc6743da326dd1c
  Public key (SS58):  12UA12xuDnEkEsEDrR4T4Cf3S1Hyi2C7B6hJW8LTkcsZy8BX
  SS58 Address:       12UA12xuDnEkEsEDrR4T4Cf3S1Hyi2C7B6hJW8LTkcsZy8BX
```

```sh
subkey inspect 12mzv68gS8Zu2iEdt4Ktkt48JZSKyFSkAVjvtgYhoa42NLNa/1

Public Key URI `12mzv68gS8Zu2iEdt4Ktkt48JZSKyFSkAVjvtgYhoa42NLNa/1` is account:
  Network ID/Version: polkadot
  Public key (hex):   0xc62ec5cd7d83e1f41462d455bb47b6bad9ed5a14741a920ead8366c63746391b
  Account ID:         0xc62ec5cd7d83e1f41462d455bb47b6bad9ed5a14741a920ead8366c63746391b
  Public key (SS58):  15UrNnNSMpX49F3mWcCX7y4kMGcvnQxCabLMT3d8U5abpwr3
  SS58 Address:       15UrNnNSMpX49F3mWcCX7y4kMGcvnQxCabLMT3d8U5abpwr3
```
