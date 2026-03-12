# Series E Indexacion

Los valores de series representan muestras indexadas en el tiempo con historial
acotado.

## Formas De Series De Mercado

PalmScript expone las series de mercado solo mediante formas calificadas por
fuente:

```palmscript
bn.close
bb.1h.close
```

Reglas:

- `<alias>.<field>` se refiere a esa fuente en el intervalo base del script
- `<alias>.<interval>.<field>` se refiere a esa fuente en el intervalo nombrado
- identificadores de mercado desnudos como `close` se rechazan
- las referencias a intervalos superiores requieren una declaracion
  `use <alias> <interval>` correspondiente

## Semantica De La Muestra Actual

Cuando una serie se usa sin indexacion, la expresion evalua a la muestra actual
de esa serie en el paso de ejecucion actual.

## Indexacion

La indexacion tiene la forma:

```palmscript
x[n]
```

Reglas:

- `n` debe ser un literal entero no negativo
- la indexacion dinamica se rechaza
- solo los valores de series pueden indexarse
- `x[0]` se refiere a la muestra actual
- `x[1]` se refiere a la muestra anterior
- `x[n]` se refiere a la muestra de hace `n` actualizaciones en el propio reloj
  de esa serie

Si no existe suficiente historial, la expresion indexada evalua a `na`.

## Propiedad Del Reloj De Actualizacion

Cada serie avanza sobre su propio reloj de actualizacion.

Ejemplos:

- `bn.close[1]` sigue el intervalo base
- `bb.1h.close[1]` sigue a la fuente `bb` en el reloj de una hora

Las series derivadas heredan los relojes de actualizacion de sus entradas. Una
serie lenta no se vuelve a contar sobre relojes mas rapidos cuando no avanzo.

## Muestras Faltantes

Las series pueden producir `na` para la muestra actual cuando:

- no hay suficiente historial
- falta el feed de una fuente en un paso del reloj base construido a partir de
  la union de los timestamps base de las fuentes declaradas
- la serie pertenece a un feed de intervalo superior que aun no ha cerrado ni
  una vez
- un indicador sigue en fase de warmup

## Series De Tiempo

`time` es una serie numerica cuya muestra es la hora de apertura de la vela en
Unix milliseconds UTC.

Reglas:

- `time` base expone la hora de apertura de la vela del intervalo base
- `time` en un intervalo superior expone la hora de apertura de esa vela de
  intervalo superior
- `time` calificado por fuente sigue las mismas reglas de seleccion de fuente e
  intervalo que los campos de precio y volumen
