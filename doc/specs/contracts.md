# 📱 EsimRegistry

A minimal, self-custodied **eSIM profile ↔ wallet registry** built on Ethereum.
It binds unique eSIM profiles (`bytes32` identifiers) to wallet addresses, enabling authenticated mapping, updates, and removals — while enforcing **one-to-one profile–wallet relationships**.

---

## ✨ Features

* **Register**: Bind a new eSIM profile to a wallet.
* **Update**: Rebind an existing eSIM profile to a new wallet.
* **Deregister**: Remove the binding between a profile and wallet.
* **Lookup**: Query profile → wallet or wallet → profile mappings.
* **EIP-712 domain separation**: Built-in typed data hashing for signatures and off-chain verification.
* **Strict validation**: Ensures no duplicate registrations, invalid inputs, or double-bindings.

---

## 🏗 Contract Overview

* **Events**

  * `Register(profile, wallet, created)`
  * `Update(profile, wallet, updated)`
  * `Deregister(profile, wallet, removed)`

* **Core Mappings**

  * `profileToWallet`: maps `bytes32 profile → address wallet`
  * `walletToProfile`: maps `address wallet → bytes32 profile`

* **Access Control**

  * Functions are gated by `onlySelf` — meaning calls must come **through the contract itself** (e.g., via [ERC-7702 smart account execution](https://eips.ethereum.org/EIPS/eip-7702)), preventing arbitrary external calls.

---

## 🚀 Usage

### 1. Register a Profile

```solidity
esimRegistry.registerProfile(esimProfile, esimWallet);
```

* `esimProfile`: unique `bytes32` identifier for the eSIM (e.g., keccak256 hash of ICCID).
* `esimWallet`: wallet address to bind.

Emits: `Register(profile, wallet, timestamp)`.

---

### 2. Update a Profile

```solidity
esimRegistry.updateProfile(esimProfile, newWallet);
```

* Clears the old wallet binding.
* Rebinds the profile to `newWallet`.

Emits: `Update(profile, newWallet, timestamp)`.

---

### 3. Deregister a Profile

```solidity
esimRegistry.deregisterProfile(esimProfile);
```

* Removes both mappings.

Emits: `Deregister(profile, wallet, timestamp)`.

---

### 4. Query Bindings

```solidity
address wallet = esimRegistry.getWallet(profile);
bytes32 profile = esimRegistry.getProfile(wallet);
```

---

## 🔒 Validation Rules

* Profile **must not** be `0x0` and **must not** be already registered.
* Wallet **must not** be `0x0` and **must not** be already registered.
* Lookups revert if the profile or wallet is not registered.

---

## ⚡ Technical Notes

* Uses **EIP-712 domain separator** with cached `chainId` for efficient typed data hashing.
* Designed for **7702 controlled execution** (`onlySelf`), ensuring all operations are mediated.
* All timestamps are `uint48` to minimize gas cost while covering centuries of block time.

---

## 🧪 Example Flow

1. `registerProfile(profile1, wallet1)` → binds profile1 ↔ wallet1.
2. `getWallet(profile1)` → returns wallet1.
3. `updateProfile(profile1, wallet2)` → rebinds to wallet2, wallet1 cleared.
4. `deregisterProfile(profile1)` → removes mapping.

---

## 📜 License

MIT License.
