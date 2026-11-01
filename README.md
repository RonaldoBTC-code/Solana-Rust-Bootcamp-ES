# Rust Basics — Repositorio de práctica

Ocho ejercicios pequeños, **37 tests** y un objetivo: que te sientas cómodo con
el Rust que necesitas para leer y escribir un programa de Solana.

Los ejercicios corresponden a las dos presentaciones:

| Presentación | Ejercicios | Tests |
| --- | --- | ---: |
| Rust Basics 1 | ex01 – ex04 | 19 |
| Rust Basics 2 | ex05 – ex08 | 18 |
| **Total** | **ex01 – ex08** | **37** |

## Inicio rápido

```bash
git clone https://github.com/dev3pack/Solana-Rust-Bootcamp-ES.git
cd Solana-Rust-Bootcamp-ES
cargo test
```

La primera ejecución compila el crate y todos los tests fallan, porque cada
función termina en `todo!()`. Eso es lo esperado. Elige un ejercicio, lee su
tarea y haz que sus tests pasen.

### Trabaja en tu propia rama

`main` es el punto de partida limpio. No subas tus soluciones ahí. Crea una
rama con tu nombre una sola vez y luego sigue haciendo push a ella:

```bash
git checkout -b solutions/tu-nombre
```

Así tu trabajo queda separado del de los demás estudiantes, y el CI muestra tu
propio resultado en lugar del de otra persona.

Ejecuta un ejercicio a la vez mientras trabajas:

```bash
cargo test --test ex01
```

Ejecuta todo antes de hacer push:

```bash
cargo test
```

También verás advertencias de `unused variable` en las funciones iniciales.
Desaparecen a medida que reemplazas cada `todo!()` con una implementación real.

## Qué hay en el repositorio

```
src/
  lib.rs        la raíz del crate; declara los ocho módulos
  ex01.rs       variables y tipos
  ex02.rs       funciones y control de flujo
  ex03.rs       ownership (propiedad)
  ex04.rs       borrowing (préstamos)
  ex05.rs       structs e impl
  ex06.rs       enums y match
  ex07.rs       Result y ?
  ex08.rs       proyecto final: mini banco
tests/
  ex01.rs ... ex08.rs   los 37 tests; no los edites
tasks/
  ex01.md ... ex08.md   qué construir en cada ejercicio
.github/workflows/tests.yml
tests.sha256             checksums que usa el CI para detectar tests editados
AI_POLICY.md
```

## Cómo trabajar

1. Lee `tasks/exNN.md`.
2. Abre `src/exNN.rs`.
3. Reemplaza un `todo!()` a la vez.
4. Ejecuta `cargo test --test exNN`.
5. Lee el error del compilador de arriba hacia abajo: código de error, flecha, ayuda.
6. Cuando el ejercicio esté en verde, pasa al siguiente.
7. Ejecuta `cargo test` y haz push a tu rama cuando pasen los 37 tests.

Los tests son la especificación. Si un test y la descripción de una tarea no
coinciden, gana el test. Si crees que un test está mal, pregúntale al
instructor; no lo edites.

## Reglas

- **No edites nada en `tests/`.** El CI ejecuta `sha256sum -c tests.sha256`
  antes de los tests. Un archivo de test modificado hace fallar el build aunque
  el código pase.
- **No borres ni renombres tests.** El evaluador cuenta los tests aprobados de 37.
- **Nada de `HashMap` ni traits en ex08.** El proyecto final usa un
  `Vec<String>` para el historial a propósito. Mantenlo simple.
- **Nada de panics para fallos esperados.** Usa `Result` en ex07 y ex08. Un
  `todo!()`, un `unwrap()` sobre la entrada del usuario o un `panic!()` no es
  manejo de errores.
- **La IA está permitida como tutor, no como máquina de soluciones.** Lee
  `AI_POLICY.md`. Si no puedes explicar tu código en una llamada de dos minutos,
  no cuenta.

## Calificación

Cada test vale un punto. La nota automática es:

```
puntos = número de tests aprobados
```

Cada parte se califica sobre los tests de esa parte: 19 para Rust Basics 1 y 18
para Rust Basics 2. Un mentor también revisa una muestra de soluciones en busca
de llamadas innecesarias a `unwrap()` y `clone()`, y puede pedirte que expliques
una solución. Pasar los tests con código que no puedes explicar no cuenta como
aprobado.

## El CI y el checksum

Cada push a tu rama, y cada pull request, ejecuta:

1. `sha256sum -c tests.sha256` — verifica que los archivos de test no cambiaron.
2. `cargo test --locked` — ejecuta los 37 tests.

El CI está en rojo al inicio, porque `todo!()` hace panic. Se pone en verde
cuando pasan los 37 tests. Para verificar el checksum tú mismo en macOS:

```bash
shasum -a 256 -c tests.sha256
```

Si el paso del checksum falla, restaura los tests:

```bash
git checkout -- tests/
```

Luego vuelve a ejecutar los tests. Si agregaste un test propio, ponlo en un
archivo aparte fuera de `tests/` o elimínalo antes de hacer push; el checksum
solo cubre los archivos oficiales, y los archivos extra no forman parte de la
nota.

## Errores comunes del compilador

| Error | Qué significa | Lo primero que puedes intentar |
| --- | --- | --- |
| `E0382` | Usaste un valor después de que fue movido | Préstalo con `&`, o clónalo si de verdad necesitas dos dueños |
| `E0502` | Una lectura y una escritura están vivas al mismo tiempo | Termina la lectura antes de escribir |
| `E0308` | Tipos que no coinciden | Compara la firma de la función con el valor que retornas |
| `E0004` | A un `match` le falta un caso | Agrega el brazo que falta, o usa `_` solo cuando un comodín sea correcto |
| `E0499` | Más de un préstamo mutable | Mantén vivo un solo `&mut` a la vez |
| `E0596` | No se puede prestar como mutable | Declara la variable con `let mut` |

`todo!()` no es un error de compilación. Compila y hace panic en tiempo de
ejecución con "not yet implemented". Si ves ese panic, la función todavía es un
esqueleto.

## Recursos

- The Rust Book, capítulos 3 a 9: <https://doc.rust-lang.org/book/>
- El libro de Rust en español: <https://book.rustlang-es.org/>
- Rustlings, ejercicios cortos: <https://github.com/rust-lang/rustlings>
- Biblioteca estándar de Rust: <https://doc.rust-lang.org/std/>
- Documentación de Solana: <https://solana.com/docs>
- Documentación de Anchor: <https://www.anchor-lang.com/docs>

## Preguntas

Abre un issue en este repositorio o pregunta en el canal de la cohorte. Incluye
el número del ejercicio, el comando exacto que ejecutaste y el error completo
del compilador. No pegues solo la última línea; la primera línea y la flecha
importan.
