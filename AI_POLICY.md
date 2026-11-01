# Política de IA

La IA está permitida en este curso. Copiar una respuesta no.

La regla es una sola oración:

> **Pídele a la IA que te explique el error, no que lo resuelva.**

Si no puedes explicar tu solución en una llamada de dos minutos, no cuenta.
No se trata de atraparte. Se trata de asegurar que la habilidad se quede en tu
cabeza cuando termine la cohorte.

## Qué está permitido

- Preguntar por qué el compilador rechazó una línea y qué significa el código de
  error.
- Pedir un ejemplo más pequeño que demuestre un concepto.
- Preguntar la diferencia entre `&` y `&mut`, o entre `String` y `&str`.
- Pedirle que revise tu explicación de una solución.
- Pedir una pista cuando estés atascado, y luego cerrar el chat y escribir el
  código tú mismo.

Ejemplo de un buen prompt:

```text
Este es mi código y este es el error completo del compilador. Explícame qué se
movió, qué línea lo causó y cuáles son mis opciones. No me des la función
corregida.
```

## Qué no está permitido

- Pegar un ejercicio y pedir la solución completa.
- Copiar en `src/` código que no entiendes.
- Pedirle a la IA que reescriba los tests para que pasen.
- Usar IA para escribir tu explicación de dos minutos.
- Ocultar el uso de IA cuando un mentor te pregunta cómo resolviste algo.

## Cómo usar bien una pista

1. Lee el error del compilador de arriba hacia abajo.
2. Escribe, con tus propias palabras, lo que crees que significa el error.
3. Pídele a la IA que confirme o corrija esa explicación.
4. Cierra el chat.
5. Escribe la corrección tú mismo, sin el chat abierto.
6. Ejecuta el test.
7. Explica la corrección en voz alta en una o dos oraciones.

Si el paso 5 es imposible sin el chat, todavía no lo has aprendido. Vuelve a la
tarea y al Rust Book.

## Una explicación de dos minutos

Un mentor puede pedirte que expliques cualquier función que entregaste. Una
explicación aprobada cubre:

- qué recibe la función y qué retorna;
- por qué los parámetros usan `&`, `&mut` o un valor con ownership;
- qué pasa en el camino de error;
- por qué el test fallaría si se eliminara una línea.

Si puedes hacer eso, el código es tuyo. Si no puedes, la nota no cuenta, aunque
los tests estén en verde.

## Citar el uso de IA

Si la IA te ayudó a entender un concepto, dilo en la descripción del pull
request:

```text
Usé un chat de IA para entender E0502. Escribí la corrección yo mismo.
```

Eso es honesto y no tiene penalización. Afirmar que escribiste algo que copiaste
sí tiene penalización.

## El objetivo

El compilador es tu primer auditor. La IA puede ser un segundo tutor, pero no
puede ser tus manos. El objetivo de este repositorio no son 37 marcas verdes. El
objetivo es poder abrir un programa real de Anchor, leerlo y modificarlo sin
adivinar.
