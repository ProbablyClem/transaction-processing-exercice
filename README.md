# transaction-processing-exercice
Made by Clement Guiton

## Toy transaction processing engine

The engine implements all the required cases : 
- Deposit
- Withdrawal
- Dispute
- Resolve
- Chargeback

## Usage
The program takes the path of the input file as a argument, and outputs the result in the console.

## Tests
The code is tested all the way through, and the tests can be run with the following command:
```bash
cargo test
```

Tests includes serialization and deserialization of the transactions, file reading, and all the cases of the transaction processing engine.

## Safety
Since this is a CLI tool, handling financial transactions, we panic on error.
We'd rather fail fast and loud than let a bug go unnoticed, and cause corrutped data.
Cases that were documented in the requirements are skipped, for example a dispute on a non-existing transaction.  
Otherwise we assert business logic constraits with expects throughout the code  [Tiger style](https://github.com/tigerbeetle/tigerbeetle/blob/main/docs/TIGER_STYLE.md).

Some cases are also handled gracefully, a deposit on a frozen account will be ignored, but we keep it in a separate queue for further processing in case of unlocking.