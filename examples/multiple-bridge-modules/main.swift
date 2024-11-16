func print_bank_account_amount(bank_account: BankAccount) {
    print("Bank Account contains $\(bank_account.amount())")
}

func print_user_name(user: User) {
    print("User Name \(user.name().toString())")
}

print_bank_account_amount(bank_account: make_bank_account())
print_user_name(user: make_user())