//! ex02 - Funciones y control de flujo.
//!
//! Lee primero `tasks/ex02.md`. Reemplaza cada `todo!()` con una implementación.

/// La comisión base de una transacción, en lamports.
///
/// Solana cobra 5_000 lamports por firma. Esta es solo la comisión base; las
/// comisiones de prioridad se suman por separado.
pub fn base_fee(signatures: u64) -> u64 {
    todo!()
}

/// Suma todos los montos del slice.
///
/// Un slice vacío suma cero.
pub fn total(amounts: &[u64]) -> u64 {
    todo!()
}

/// Cuenta cuántos montos son estrictamente mayores que `threshold`.
pub fn count_above(amounts: &[u64], threshold: u64) -> usize {
    todo!()
}

/// Nunca transfieras más que el saldo.
///
/// Retorna el monto que realmente se puede transferir.
pub fn clamp_transfer(amount: u64, balance: u64) -> u64 {
    todo!()
}
