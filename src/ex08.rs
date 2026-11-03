//! ex08 - Proyecto final: una mini cuenta bancaria.
//!
//! Lee primero `tasks/ex08.md`. Reemplaza cada `todo!()` con una implementación.
//! Nada de `HashMap` ni traits: solo un struct, un historial `Vec<String>` y
//! `Result`.

pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

/// Todo lo que puede salir mal en el mini banco.
#[derive(Debug, PartialEq)]
pub enum BankError {
    ZeroAmount,
    InsufficientFunds { available: u64, required: u64 },
}

/// Una cuenta bancaria con un historial legible para humanos.
#[derive(Debug, Clone, PartialEq)]
pub struct BankAccount {
    pub owner: String,
    pub lamports: u64,
    pub history: Vec<String>,
}

impl BankAccount {
    /// Crea una cuenta con cero lamports y un historial vacío.
    pub fn new(owner: &str) -> Self {
        Self {
            owner: owner.to_string(),
            lamports: 0,
            history: Vec::new(),
        }
    }

    /// Deposita lamports y lo registra en el historial.
    ///
    /// Retorna `BankError::ZeroAmount` para un depósito de cero. Un depósito
    /// exitoso agrega `"deposit <amount>"` al historial.
    pub fn deposit(&mut self, amount: u64) -> Result<(), BankError> {
        if amount == 0 {
            return Err(BankError::ZeroAmount);
        }

        self.lamports += amount;
        self.history.push(format!("deposit {}", amount));
        Ok(())
    }

    /// Retira lamports y lo registra en el historial.
    ///
    /// Retorna `BankError::ZeroAmount` para un retiro de cero. Retorna
    /// `BankError::InsufficientFunds` cuando el saldo es insuficiente. Un retiro
    /// exitoso agrega `"withdraw <amount>"` al historial.
    pub fn withdraw(&mut self, amount: u64) -> Result<(), BankError> {
        if amount == 0 {
            return Err(BankError::ZeroAmount);
        }

        if amount > self.lamports {
            return Err(BankError::InsufficientFunds {
                available: self.lamports,
                required: amount,
            });
        }

        self.lamports -= amount;
        self.history.push(format!("withdraw {}", amount));
        Ok(())
    }

    /// Retorna el saldo en SOL enteros, descartando el residuo.
    pub fn balance_in_sol(&self) -> u64 {
        self.lamports / LAMPORTS_PER_SOL
    }
}