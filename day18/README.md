# 🚀 Day 18 - Learning Rust

## Key improvements

1. `HashMap<uszie, Account>` vs `Vec<Account>`

   ✅ Faster O(1) lookup using account IDs as keys

   ✅ No need to scan entire list with iter().find(...)

2. `serder` + JSON I/O

   🔄 Accounts persist between runs!

   File is stored as accounts.json

3. Input validation

   No more crashes from unwrap() on parse

   User is prompted again if input is bad

## New output

```bash
🏦 Banking System:
1. Create Account
2. View Balance
3. Deposit
4. Withdraw
5. Save & Exit
Enter an option: 1
Account holder name: thien
Initial deposit: 1000
✅ Account created with ID: 1

🏦 Banking System:
1. Create Account
2. View Balance
3. Deposit
4. Withdraw
5. Save & Exit
Enter an option: 2
Enter Account ID: 1
thien (ID: 1) | Balance: 1000.00

🏦 Banking System:
1. Create Account
2. View Balance
3. Deposit
4. Withdraw
5. Save & Exit
Enter an option: 3
Enter Account ID: 1000
Amount to deposit: 2
❌ Account not found

🏦 Banking System:
1. Create Account
2. View Balance
3. Deposit
4. Withdraw
5. Save & Exit
Enter an option: 1
Account holder name: 3
Initial deposit: 1000
✅ Account created with ID: 2
```

## Old output

```bash
🏦 Banking System:
1. Create Account
2. View Balance
3. Deposit
4. Withdraw
5. Exit
Enter an option: 4
Enter Account ID: 1
Amount to withdraw: 600
Withdrawal successful. new balance: 500.00

🏦 Banking System:
1. Create Account
2. View Balance
3. Deposit
4. Withdraw
5. Exit
Enter an option: 2
Enter Account ID: 1
thien | balance: 500

🏦 Banking System:
1. Create Account
2. View Balance
3. Deposit
4. Withdraw
5. Exit
Enter an option: 5
Goodbye
```
