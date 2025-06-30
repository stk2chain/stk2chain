# STK2ETH JavaCard Applet Specifications

<img src="../Consumer_eUICC.excalidraw.svg" />

## Trusted Execution Environment
|Component| Function|
|---------|-------------------------|
|`eUICCs` |  Hardware root of trust.|
|`Java Card VM` | Isolated execution environment.|

We leverages the eUICCs as a hardware root of trust, operating within the isolated execution environment of the Java Card VM, effectively creating a Trusted Execution Environment (TEE).

This secure, isolated compute environment guarantees:
  - Confidentiality: 
      - Private keys cannot be read externally.
      - No seed phrases.
      - Keys are generated and stored exclusively within the eUICC Secure Element, ensuring they never leave.
  - Integrity: 
      - Code execution is tamper-proof.
      - Ensured by the combination of the Attested Java Card VM, eUICC and ERC-4337 wallet.
  - Attestation: 
      - Proof of execution in a trusted environment.
      - Transactions are signed exclusively within the Java Card VM.

<br>

This design enables a non-custodial, optimally secure, seedless ERC-4337 mobile wallet by using eSIM Profiles as Trusted Execution Environments (TEE).
It’s highly inspired by Daimo’s and Taisys' models, combining and extending them by leveraging eUICC secure enclaves operating within the isolated execution environment of the Java Card VM for key generation, signing and attestation.


## Private Key Generation and Storage
No seed phrases. Keys are generated and stored `exclusively` within the eUICC Secure Element, ensuring they never leave.


### Must 
1. Keys are generated `exclusively` within the Java Card VM (isolated execution environment).
2. Keys are stored `exclusively` within the eUICC Secure Element ( hardware root of trust )
3. No seed phrases - keys never leave the Secure Element.
4. Keys cannot be read external of applet's Java Card VM (isolated execution environment)
5. Java Card VM Default exclusive:
      - Access Policy is by default exclusive to the applet signature
      - 2 applets by default cannot share the same signature
      - All is erased when applet is erased

### Key Derivation
Uses `SECP236r1`(P-256) with future support for Ed25519. Follows `m/44'/60'/0'/0/x` derivation path (BIP-32,BIP-44 compliant).

- secp256k1 : [BTC/ETH]
- secp256r1: Rest of world  (our default)

| Curve			| Blockchain Use		|		Non-Blockchain Use		|	Security Level	|	Java Card Support |
|-----------|-------------------|-------------------------|-----------------|-------------------|
|secp256k1	|	Bitcoin, Ethereum	|		Rare outside blockchain|			128-bit		|	❌ No official support
|secp256r1 (P-256)	|XRP, Hedera				|TLS, Passkeys, Apple, Google, FIDO2|	128-bit			| ✅ Yes
|Ed25519			|Monero, Solana, NEAR, Polkadot|		SSH, Signal, Tor, WebAuthn|		128-bit			| ✅ Yes
|Ed448			|Limited					|FIDO2, Post-Quantum Research|		224-bit			| ✅ Yes


<!-- R1: [ DeepSeek, P-256, Yamaha, PlayStation ] #M1 -->

#### Summary

| Step		|	Action |
|---------|--------|
|🔹 No Seed Phrase	|Keys stored inside Secure Enclave, generated securely|
|🔹 BIP-32 secp256r1	|Follow m/44'/60'/0'/0/x derivation|
|🔹 Ethereum Address|	Compute from secp256r1 public key → keccak256 → EIP-55|
|🔹 MetaMask Import|	Document path for future R1 support|
|🔹 Security		|Private keys never leave enclave|




## UserOp Signing
Transactions are signed `exclusively` within the Java Card VM (isolated execution environment) using the Generated Key.


## Text-Based User Interface (STK Menu)
STK Menu implements a Text-Based User Interface for:

1. Send ETH
2. Swap
3. Withdraw Cash
4. Buy Airtime
5. My Account
6. Check Balance


```JSON
{

"MainScreen": {
			"text": "M-ETH Main Menu",
			"screen_type": "Menu",
			"default_next_screen": "DefaultNoneScreen",
			"menu_items": {
				"SendETHOption": {
					"option": "1",
					"display_name": "Send ETH",
					"next_screen": "SendETHScreen"
				},
				"SwapOption": {
					"option": "2",
					"display_name": "Swap",
					"next_screen": "SwapScreen"
				},
				"WithdrawOption": {
					"option": "3",
					"display_name": "Withdraw Cash",
					"next_screen": "WithdrawScreen"
				},
				"AirtimeOption": {
					"option": "4",
					"display_name": "Buy Airtime",
					"next_screen": "AirtimeScreen"
				}
				"AccountOption": {
					"option": "5",
					"display_name": "My Account",
					"next_screen": "AccountScreen"
				},
				"BalanceOption": {
					"option": "6",
					"display_name": "Check Balance",
					"next_screen": "BalanceScreen"
				},
			}
		}
}
```
## USSD Submission
On `processToolkit(...)` execution, the `eSIM Toolkit(eSTK)` Applet builds and sends a `SEND USSD` Proactive Command from the `eUICC` to the `Mobile Equipment (ME/phone)` encoded in `BER-TLV` format.



```Java
public class METHApplet extends Applet implements ToolkitInterface, ToolkitConstants {

    public void processToolkit(byte event) throws ToolkitException {
        
        // USSD CALL via Proactive Command
        // Type of proactive command : SEND USSD  => 0x12 
        // public static final byte PRO_CMD_SEND_USSD = (byte)0x12;

        // Initialize proactive handler
        proHandler = ProactiveHandlerSystem.getTheHandler();

        // Build Proactive Command  
        // @type: SEND USSD (0x12)           = 0x12           
        // @qualifier: the command qualifier
        // @dstDevice: Network = 0x83
        // The source device is always the SIM card.
        // D0 : Proactive Command Tag
        proHandler.init(PRO_CMD_SEND_USSD, (byte)0x00, DEV_ID_NETWORK);

		...

        // Add USSD String TLV
        // Format per 3GPP TS 31.111: DCS + USSD string
        byte dcs = (byte) 0x0F;  // 7-bit default alphabet
        byte[] fullUssd = new byte[ussdString.length + 1];
        fullUssd[0] = dcs;
        Util.arrayCopyNonAtomic(ussdString, (short) 0, fullUssd, (short) 1, (short) ussdString.length);

        // Append Text String TLV
        proHandler.appendTLV(TAG_TEXT_STRING, fullUssd, (short) 0, (short) fullUssd.length);

        // Send to ME (phone)
        proHandler.send();
        
		...
    }


 }
```

### Proactive Commands
Proactive commands in the eSIM Toolkit (STK / USAT) architecture are instructions initiated by the SIM/eSIM (UICC) to the Mobile Equipment (ME) — usually the phone or device — in order to perform actions on behalf of the eSIM, such as displaying text, opening channels, sending messages, USSD etc.

## Technical Specifications

- [ETSI TS 131 111 V18.6.0 (2024-07)](https://www.etsi.org/deliver/etsi_ts/131100_131199/131111/18.06.00_60/ts_131111v180600p.pdf) : 3GPP TS 31.111 (USAT/STK) - Universal Subscriber Identity Module (USIM) Application Toolkit (USAT) Technincal Specification
<!-- ETSI TS 131 111: 6 Proactive UICC -->
<!-- **6.4.12 SEND USSD -->
<!-- ***6.4.12.2 Application Mode -->
<!-- **6.5 Common elements in proactive UICC commands -->
<!-- **6.6 Structure of proactive UICC commands -->
<!-- **6.6.11 SEND USSD -->
<!-- 8.6 Command details -->
<!-- 8.17 USSD string -->

- [ETSI TS 123 038 V16.0.0 (2020-07)](https://www.etsi.org/deliver/etsi_ts/123000_123099/123038/16.00.00_60/ts_123038v160000p.pdf) : 3GPP TS 23.038  - 
Alphabets and language-specific information Technical Specification
<!-- GSM 03.38: GSM 7-bit default alphabet Technical Specification -->
<!-- **6.1.2 Character packing -->
<!-- **6.1.2.3 USSD packing -->

- [ETSI TS 102 223 V17.6.0 (2025-04)](https://www.etsi.org/deliver/etsi_ts/102200_102299/102223/17.06.00_60/ts_102223v170600p.pdf) - Card Application Toolkit (CAT) Technical Specification
<!-- 4.2 Proactive UICC -->
<!-- 4.11 Bearer Independent Protocol  -->
<!-- 5.2 Structure and coding of TERMINAL PROFILE -->
<!-- 6 Proactive UICC -->
<!-- **6.4 Proactive UICC commands and procedures -->
<!-- **6.5 Common elements in proactive UICC commands -->
<!-- **6.6 Structure of proactive UICC commands -->
<!-- 9 Tag values -->
<!-- **9.2 BER-TLV tags in UICC to terminal direction -->
<!-- **9.3 COMPREHENSION-TLV tags in both directions -->
<!-- Annex C (normative): Structure of CAT communications -->
<!-- Annex B (informative): Example of DISPLAY TEXT proactive UICC command -->
<!-- 8.6 Command details -->

- [ETSI TS 101 220 V18.2.0 (2024-11)](https://www.etsi.org/deliver/etsi_ts/101200_101299/101220/18.02.00_60/ts_101220v180200p.pdf) - ETSI numbering system
for telecommunication application providers Technical Specification
<!-- 7 Tag-Length-Value (TLV) data objects -->
<!-- **7.1.1 COMPREHENSION-TLV tag coding -->
<!-- **7.2 Assigned TLV tag values -->
<!-- ***Table 7.17 Card application toolkit templates BER-TLV tag -->

- [ETSI TS 131 115 V18.0.0 (2024-05)](https://www.etsi.org/deliver/etsi_ts/131100_131199/131115/18.00.00_60/ts_131115v180000p.pdf) : 3GPP TS 31.115 - Secured packet structure for (Universal) Subscriber Identity Module (U)SIM Toolkit applications Technical Specification 
<!-- 6 Implementation for USSD -->
<!-- Annex A (normative): USSD String format -->
<!-- **6.1 Structure of the Command Packet contained in a Single
USSD Message -->
<!-- **6.2 Structure of the Command Packet contained in concatenated USSD Messages -->

<!--- [ETSI TS 131 102 V18.6.2 (2024-11)](https://www.etsi.org/deliver/etsi_ts/131100_131199/131102/18.06.02_60/ts_131102v180602p.pdf) : 3GPP TS 31.102 Characteristics of the Universal Subscriber Identity Module (USIM) application Technical Specification-->



