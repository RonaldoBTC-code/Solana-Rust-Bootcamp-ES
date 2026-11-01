//! ex05 - Structs e `impl`.
//!
//! Lee primero `tasks/ex05.md`. Reemplaza cada `todo!()` con una implementación.

/// Una cuenta de Solana simplificada.
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub address: String,
    pub lamports: u64,
    pub owner: String,
}

impl Account {
    /// Crea una cuenta con cero lamports.
    ///
    /// El dueño de una cuenta nueva es el programa `"system"`.
    pub fn new(address: String) -> Self {
        todo!()
    }

    /// Agrega lamports a la cuenta.
    pub fn deposit(&mut self, amount: u64) {
        todo!()
    }

    /// Retira lamports de la cuenta.
    ///
    /// Retorna `true` cuando el retiro fue exitoso. Retorna `false` y deja el
    /// saldo sin cambios cuando no hay suficientes lamports.
    pub fn withdraw(&mut self, amount: u64) -> bool {
        todo!()
    }

    /// Retorna el saldo en SOL enteros, descartando el residuo.
    ///
    /// Esto es solo para mostrar. Nunca uses un número de punto flotante para
    /// dinero.
    pub fn balance_in_sol(&self) -> u64 {
        todo!()
    }
}
