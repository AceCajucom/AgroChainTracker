# AgroChain Tracker

An immutable, low-cost tracking protocol for agricultural crop batches built using Soroban smart contracts on the Stellar Network.

## Problem & Solution
Smallholder organic coffee farmers in Benguet, Philippines, lose up to 30% of their market income due to intermediate fraud where downstream supply chain entities modify origin and certification data. AgroChain Tracker allows farmers to write immutable crop records to the blockchain at production time, enabling end consumers to instantly audit origin truth data via QR codes.

## Timeline
* **Phase 1:** Smart Contract Core Setup & Unit Testing (Current)
* **Phase 2:** QR Code Dispatcher Frontend Engine Integration
* **Phase 3:** Pilot Deployment on Testnet

## Stellar Features Used
* Soroban Smart Contracts (Decentralized state logging)
* Stellar Custom Tokens (Representing digital batch proofs)
* Trustlines (Frictionless distribution flow)

## Prerequisites
* Rust toolchain (v1.75+)
* Soroban CLI installed

## How to Build
```bash
soroban contract build

## Deployed Contract

| Field | Value |
|-------|-------|
| Contract ID | `CASTEUZN2B65Q7MAH27CEZ6RXAY6L73KNJVBUY35HZOF56PJWFNARQBO` |
| Network | testnet |
| Explorer | [View on stellar.expert](https://stellar.expert/explorer/testnet/contract/CASTEUZN2B65Q7MAH27CEZ6RXAY6L73KNJVBUY35HZOF56PJWFNARQBO) |
| Deploy Tx | [View transaction](https://stellar.expert/explorer/testnet/tx/5191d8a7bd56d9104967bc79ac7ff718d1194f170f0c905542fc3cbf64039f2f) |
| Deployed | 2026-06-26 08:07:11 UTC |
| Wallet | freighter (`GDV4…DG5L`) |

![Screenshot 2026-06-26 160823_2.png](./assets/Screenshot%202026-06-26%20160823_2.png)
