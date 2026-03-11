# Semantica De Evaluacion

Esta pagina define como se evalúan en runtime las expresiones y sentencias de
PalmScript.

## Modelo De Ejecucion

PalmScript compila un script una sola vez y lo evalua una vez por cada paso del
reloj base.

En cada paso:

1. el runtime materializa las muestras actuales de series de mercado para ese
   paso
2. los feeds de intervalos mas lentos avanzan solo si sus velas han cerrado
   por completo para ese paso
3. el programa bytecode se ejecuta
4. las salidas `plot`, `export` y `trigger` se recopilan para ese paso

Distintos feeds de modo mercado pueden construir las entradas del paso de forma
distinta, pero la evaluacion de expresiones es la misma una vez que el paso
comienza.

## Categorias De Expresiones

Las expresiones evalúan a la muestra actual de un valor escalar o de serie.

Para una expresion de serie:

- el resultado de la expresion en un paso es una unica muestra actual
- la indexacion apunta a muestras anteriores sobre el propio reloj de
  actualizacion de esa expresion

## Precedencia De Operadores

PalmScript evalua los operadores en este orden, de menor a mayor precedencia:

1. `or`
2. `and`
3. `==`, `!=`, `<`, `<=`, `>`, `>=`
4. `+`, `-`
5. `*`, `/`
6. `-` unario, `!` unario
7. postfijos de llamada, indexacion y calificacion

Los operadores del mismo nivel de precedencia se asocian de izquierda a
derecha.

## Aritmetica

Los operadores aritmeticos son `+`, `-`, `*` y `/`.

Reglas:

- ambos operandos deben ser numericos, series numericas o `na`
- si cualquiera de los operandos es `na`, el resultado es `na`
- si cualquiera de los operandos es `series<float>`, el resultado es
  `series<float>`
- en caso contrario el resultado es `float`

## Comparaciones

Los operadores de comparacion son `==`, `!=`, `<`, `<=`, `>` y `>=`.

Reglas:

- `<`, `<=`, `>`, y `>=` requieren operandos numericos
- `==` y `!=` se definen para cualquier operando que no sea `na`
- la igualdad entre tipos mezclados compara como desigual
- si cualquiera de los operandos es `na`, el resultado es `na`
- si cualquiera de los operandos es una serie, el resultado es una serie
  booleana
- en caso contrario el resultado es `bool`

## Operadores Unarios

PalmScript soporta:

- `-` unario para operandos numericos
- `!` unario para operandos booleanos

Reglas:

- los operadores unarios propagan `na`
- `-` unario sobre `series<float>` produce `series<float>`
- `!` unario sobre `series<bool>` produce `series<bool>`

## Operadores Logicos

`and` y `or` requieren `bool`, `series<bool>` o `na`.

Usan logica determinista de tres valores:

### `and`

| Izquierda | Derecha | Resultado |
| --- | --- | --- |
| `false` | `false` | `false` |
| `false` | `true` | `false` |
| `false` | `na` | `false` |
| `true` | `false` | `false` |
| `true` | `true` | `true` |
| `true` | `na` | `na` |
| `na` | `false` | `false` |
| `na` | `true` | `na` |
| `na` | `na` | `na` |

### `or`

| Izquierda | Derecha | Resultado |
| --- | --- | --- |
| `true` | `true` | `true` |
| `true` | `false` | `true` |
| `true` | `na` | `true` |
| `false` | `true` | `true` |
| `false` | `false` | `false` |
| `false` | `na` | `na` |
| `na` | `true` | `true` |
| `na` | `false` | `na` |
| `na` | `na` | `na` |

PalmScript evalua ambos operandos antes de aplicar el operador logico. El
lenguaje no garantiza short-circuit evaluation, de modo que las expresiones
logicas se analizan y ejecutan de forma eager dentro de las reglas normales del
lenguaje.

## Semantica De `if`

`if` es una forma de sentencia, no una forma de expresion.

Reglas:

- la condicion debe evaluar a `bool`, `series<bool>` o `na`
- `na` en una condicion `if` se trata como falso para la seleccion de rama
- exactamente una rama se ejecuta en cada paso
- ambas ramas deben estar presentes sintacticamente porque `else` es
  obligatorio

## Evaluacion De Funciones

Las funciones definidas por el usuario tienen cuerpo de expresion y se compilan
mediante especializacion, no mediante despacho dinamico en runtime.

Reglas:

- la cantidad de argumentos debe coincidir con la cantidad de parametros
  declarados
- las funciones se especializan por tipo de argumento y reloj de actualizacion
- los grafos de funciones recursivos o ciclicos se rechazan en tiempo de
  compilacion
- el cuerpo de una funcion no puede llamar a `plot`

## Regla Sin Lookahead

PalmScript no debe exponer velas de intervalos superiores parcialmente
formadas.

Consecuencias:

- una serie de intervalo superior cambia solo despues de que esa vela cierre
  por completo
- la indexacion sobre series de intervalos superiores recorre el historial de
  muestras completamente cerradas de ese intervalo superior
- los intervalos suplementarios conscientes de fuente siguen la misma regla

## Semantica De Builtins Helper

Las formulas de builtins helper, los contratos de indicadores, las reglas de
ventana y el comportamiento de `na` se definen en [Builtins](builtins.md) y en
la seccion [Indicadores](indicators.md).

Reglas:

- los builtins helper siguen el reloj de actualizacion de sus entradas de serie
- las salidas helper participan en `if`, indexacion y llamadas builtin
  posteriores mediante las mismas reglas de valores y `na` definidas en esta
  pagina
- `if` sigue tratando `na` como falso para la seleccion de rama, incluso cuando
  la condicion proviene de un helper como `crossover(...)`

## Determinismo

La evaluacion de expresiones es determinista.

Durante la ejecucion de una estrategia, la semantica del lenguaje depende solo
de:

- el programa compilado
- los feeds de entrada preparados
- los limites de VM configurados

No depende de la hora del sistema, acceso al filesystem, aleatoriedad ni acceso
a la red.
