//! ex06 - Enums y `match`.
//!
//! Lee primero `tasks/ex06.md`. Reemplaza cada `todo!()` con una implementación.

/// Los datos que lleva una instrucción simplificada.
///
/// Este enum modela los *datos* de la instrucción. Las cuentas las pasa la
/// transacción por separado, igual que una instrucción de Anchor recibe un
/// `Context`.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Transfer {
        from: String,
        to: String,
        lamports: u64,
    },
    CreateAccount {
        address: String,
    },
    Close(String),
}

/// Retorna el tipo de instrucción: `"transfer"`, `"create"` o `"close"`.
pub fn kind(ix: &Instruction) -> &'static str {
    match ix {
        Instruction::Transfer { .. } => "transfer",
        Instruction::CreateAccount { .. } => "create",
        Instruction::Close(_) => "close",
    }
}

/// Retorna cuántos lamports mueve la instrucción.
///
/// Solo `Transfer` mueve lamports. Todas las demás variantes retornan `0`.
pub fn lamports_moved(ix: &Instruction) -> u64 {
    match ix {
        Instruction::Transfer { lamports, .. } => *lamports,
        Instruction::CreateAccount { .. } | Instruction::Close(_) => 0,
    }
}

/// Retorna la cuenta que es el sujeto principal de la instrucción.
///
/// Para `Transfer` retorna el remitente. Para `CreateAccount` retorna la nueva
/// dirección. Para `Close` retorna la dirección que se está cerrando.
pub fn account_touched(ix: &Instruction) -> &str {
    match ix {
        Instruction::Transfer { from, .. } => from.as_str(),
        Instruction::CreateAccount { address } => address.as_str(),
        Instruction::Close(address) => address.as_str(),
    }
}
