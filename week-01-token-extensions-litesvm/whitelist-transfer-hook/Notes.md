## Notes : Mental Model

#### 1. Why pda instead of vec ?
- When We use vec , so everytime when we add address so it will reallocate the size. Rellocation cost is so expensive so it is not idea. 
- `vec.contain` so it will also very slow - O(n) time complexity
- Maximize account size limit i.e 10MB so it is not make sense here
- When we use `PDA` - no rellocation , less size of every account


#### 2. So Yes PDA then, but pda address also store inside one single account for checking so, Will size not increase ?
- No, it is not storing inside one single account instead it derives account. During whitelisted, we create account using sender's address and word. So we just check , account is exist or not and if exist so we just fetch that account.


#### 3. How `PDA` does make sense ?
- For every approved user, we create a PDA using seeds like `["whitelist", user_pubkey]`. During transfer, we derive the PDA again. If the account exists, the transfer is allowed. If it doesn’t exist, Anchor validation fails automatically. This avoids storing a large vector and gives `O(1)` lookup.

#### 4. Why minting required ?
- Without `minting ` account doesn't exist. It has decimal, authoriy, supply, extension, or related extra functionality.

#### 5. Why `transfer hooks` ?
- Suppose, Bob transfers token to Alice. bob balances decrease and alice increase , in between hook will call and it will check custom condition , if everything goes well then successful transaction happen other wise revert. [Transfer Hooks](https://solana.com/developers/guides/token-extensions/transfer-hook)
---

### Important Stuff : 
1. The transfer hook validates the sender’s address. Only whitelisted senders can transfer tokens. The token is a custom Token-2022 mint created with the transfer hook extension, not an existing token like USDC.”
2. The transfer hook must be implemented as an instruction because Solana programs can only execute logic through instructions. The Token-2022 program CPI-calls this instruction automatically during transfers.