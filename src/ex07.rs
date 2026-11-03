//! ex07 - `Result` y el operador `?`.
//!
//! Lee primero `tasks/ex07.md`. Reemplaza cada `todo!()` con una implementación.

/// Todo lo que puede salir mal en una transferencia.
#[derive(Debug, PartialEq)]
pub enum TransferError {
    ZeroAmount,
    SameAccount,
    InsufficientFunds { available: u64, required: u64 },
}

/// Verifica las reglas que no dependen del saldo.
///
/// Retorna, en este orden:
/// 1. `ZeroAmount` cuando `amount == 0`
/// 2. `SameAccount` cuando `from == to`
/// 3. `Ok(())` en cualquier otro caso
pub fn validate(from: &str, to: &str, amount: u64) -> Result<(), TransferError> {
    if amount == 0 {
        return Err(TransferError::ZeroAmount);
    }
    if from == to {
        return Err(TransferError::SameAccount);
    }
    Ok(())
}

/// Valida una transferencia y retorna el saldo restante del remitente.
///
/// Llama primero a `validate` y propaga su error con `?`. Luego retorna
/// `InsufficientFunds` cuando `amount > balance`. Si todo sale bien, retorna
/// `Ok(balance - amount)`.
pub fn transfer(
    balance: u64,
    from: &str,
    to: &str,
    amount: u64,
) -> Result<u64, TransferError> {
    validate(from, to, amount)?;

    if amount > balance {
        return Err(TransferError::InsufficientFunds {
            available: balance,
            required: amount,
        });
    }

    Ok(balance - amount)
}

/// Paga a través de `transfer` y propaga cualquier error con `?`.
///
/// Esta función existe para mostrar la forma de una función que usa `?`.
pub fn pay(balance: u64, from: &str, to: &str, amount: u64) -> Result<u64, TransferError> {
    let new_balance = transfer(balance, from, to, amount)?;
    Ok(new_balance)
}
