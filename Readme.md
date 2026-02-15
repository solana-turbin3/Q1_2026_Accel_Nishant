# Turbin3 Q1 2026 Accelerator — Builders Program

Solana programs and integrations built for the **Turbin3 Q1 2026 Accelerator**. Proof of work across SPL Token 2022, decentralized task queues, and Ephemeral Rollups.

---

## Projects

### Whitelist Transfer Hook

`whitelist-transfer-hook/`

SPL Token 2022 **transfer hook** — only whitelisted addresses can transfer; others are rejected on-chain. Admin-managed whitelist, ExtraAccountMetaList, full flow tests.

| Network | Program ID                                     |
| ------- | ---------------------------------------------- |
| Devnet  | `EfvcbUrqid3P54BhoFLrJhAdJxe2vxKhGG9sDRvCsWHh` |

→ [README](./whitelist-transfer-hook/README.md) · `anchor build` · `anchor test`

---

### TukTuk Counter

`tuktuk-counter/`

[Helium TukTuk](https://github.com/helium/tuktuk) integration: counter program with **on-chain task queue** and **cron automation**. Increment via CPI-scheduled tasks or a recurring cron job — no centralized off-chain runner.

| Network | Program ID                                     |
| ------- | ---------------------------------------------- |
| Devnet  | `6wpZxCkv6bZwosC1WHqqhm2sDZfV3oA7fSMU78vkipaR` |

→ [README](./tuktuk-counter/README.md) · `anchor build` · `anchor test` · `anchor run cron`

---

### Magic Block — ER State Account

`magicblock-er/`

**Ephemeral Rollups** state-account example using [Magic Block’s Ephemeral Rollups SDK](https://github.com/magicblock-labs). User account lifecycle (init, update, delegate, undelegate, close), commit updates, and **state updates outside the rollup** via Ephemeral VRF (request randomness → callback).

| Network | Program ID                                     |
| ------- | ---------------------------------------------- |
| Devnet  | `EQkMxVqHWsEPHD44yAicQZ55Av8AGbfLdgsuZPJmUBqm` |

→ `anchor build` · `yarn test`

---

**Prerequisites:** Rust, Solana CLI, Anchor CLI, Node.js, Yarn. Per-project setup and details are in each folder (and linked READMEs where present).

_Turbin3 Q1 2026 Accelerator._
