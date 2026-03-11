# Glosario

## Intervalo Base

El intervalo de ejecucion declarado por `interval <...>`.

## Intervalo Declarado

Un intervalo introducido explicitamente mediante `use <alias> <...>` para una
fuente con nombre.

## Script Consciente De Fuentes

Un script que declara al menos una `source`.

## Template De Fuente

Un constructor builtin de exchange o venue, como `binance.spot` o
`hyperliquid.perps`.

## Modo Mercado

Ejecucion sobre feeds historicos respaldados por datos de mercado.

## Sin Lookahead

La garantia de que una muestra de un intervalo superior solo se vuelve visible
despues de que esa vela cierra por completo.

## Serie De Salida

Un resultado nombrado por paso emitido por `export` o `trigger`.

## Union De Timestamps Base

La linea temporal de ejecucion construida a partir de la union de los tiempos
de apertura de velas del intervalo base para todas las fuentes declaradas.
