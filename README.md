# Rust Basics — Repositorio de práctica

Ocho ejercicios pequeños, **37 tests** y un objetivo: que te sientas cómodo con
el Rust que necesitas para leer y escribir un programa de Solana.

Este repositorio es **público**. No necesitas una invitación, y no tienes
permisos de escritura en él. **Haz un fork**, resuelve los ejercicios en tu fork
y abre un pull request. El pull request es tu entrega; el instructor califica
sus checks.

Los ejercicios corresponden a las dos presentaciones:

| Presentación | Ejercicios | Tests |
| --- | --- | ---: |
| Rust Basics 1 | ex01 – ex04 | 19 |
| Rust Basics 2 | ex05 – ex08 | 18 |
| **Total** | **ex01 – ex08** | **37** |

## Inicio rápido

1. Haz clic en **Fork** en la parte superior de esta página. Esto crea tu propia
   copia en GitHub.
2. Clona **tu fork**, no el repositorio de la clase:

```bash
git clone https://github.com/<tu-usuario>/Solana-Rust-Bootcamp-ES.git
cd Solana-Rust-Bootcamp-ES
```

3. Guarda el repositorio de la clase como `upstream`, para poder traer
   actualizaciones más adelante:

```bash
git remote add upstream https://github.com/dev3pack/Solana-Rust-Bootcamp-ES.git
```

4. Ejecuta los tests:

```bash
cargo test
```

La primera ejecución compila el crate y todos los tests fallan, porque cada
función termina en `todo!()`. Eso es lo esperado. Elige un ejercicio, lee su
tarea y haz que sus tests pasen.

## Trabaja en tu propia rama

Crea una rama en tu fork. No trabajes en `main`.

```bash
git checkout -b solutions/tu-nombre
```

`main` se queda como el punto de partida. Tu rama contiene tus respuestas. Esto
también mantiene tu trabajo separado del de los demás.

Ejecuta un ejercicio a la vez mientras trabajas:

```bash
cargo test --test ex01
```

Ejecuta todo antes de hacer push:

```bash
cargo test
```

Haz push a **tu fork**:

```bash
git add .
git commit -m "ex01-ex08 solutions"
git push -u origin solutions/tu-nombre
```

Verás advertencias de `unused variable` en las funciones iniciales.
Desaparecen a medida que reemplazas cada `todo!()` con una implementación real.

## Entrega tu trabajo

Después de hacer push de tu rama, abre un pull request:

1. Ve a tu fork en GitHub.
2. Haz clic en **Contribute** → **Open pull request**.
3. Base repository: `dev3pack/Solana-Rust-Bootcamp-ES`.
4. Base branch: `main`.
5. Head repository: tu fork.
6. Compare branch: `solutions/tu-nombre`.
7. Título: `Soluciones — tu nombre`.
8. Haz clic en **Create pull request**.

**No** hagas merge del pull request. Es una entrega, no un cambio al repositorio
de la clase. El instructor califica los checks y luego lo cierra.

Tu fork es público, igual que este repositorio. No pongas contraseñas, claves
privadas ni datos personales en él.

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
7. Cuando pasen los 37 tests, haz push de tu rama y abre el pull request.

Los tests son la especificación. Si un test y la descripción de una tarea no
coinciden, gana el test. Si crees que un test está mal, pregúntale al
instructor; no lo edites.

## Reglas

- **Mantén tus cambios en `src/`.** No edites `tests/`, `tests.sha256` ni
  `.github/`.
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

La nota sale de los checks de tu **pull request**, no de un push directo al
repositorio de la clase.

## El CI y el checksum

Cada pull request al repositorio de la clase ejecuta:

1. `sha256sum -c tests.sha256` — verifica que los archivos de test no cambiaron.
2. `cargo test --locked` — ejecuta los 37 tests.

Los push a tu fork también pueden ejecutar el workflow, si tienes Actions
habilitado ahí. La ejecución que cuenta es la del pull request. Si es tu primer
pull request, GitHub puede pedirle al instructor que apruebe el workflow; eso es
normal.

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
archivo aparte fuera de `tests/` o elimínalo antes de abrir el pull request;
solo cuentan los 37 tests oficiales.

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

Abre un issue en el **repositorio de la clase**:
<https://github.com/dev3pack/Solana-Rust-Bootcamp-ES/issues>

El instructor no ve los issues que abras en tu propio fork. Incluye el número
del ejercicio, el comando exacto que ejecutaste y el error completo
del compilador. No pegues solo la última línea; la primera línea y la flecha
importan.
