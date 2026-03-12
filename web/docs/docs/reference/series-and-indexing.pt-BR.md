# Series E Indexacao

Valores de serie representam amostras indexadas no tempo com historico limitado.

## Formas De Series De Mercado

PalmScript expoe series de mercado apenas por formas qualificadas por fonte:

```palmscript
bn.close
bb.1h.close
```

Regras:

- `<alias>.<field>` referencia aquela fonte no intervalo base do script
- `<alias>.<interval>.<field>` referencia aquela fonte no intervalo nomeado
- identificadores de mercado soltos como `close` sao rejeitados
- referencias a intervalos superiores de source exigem uma declaracao
  correspondente `use <alias> <interval>`

## Semantica Da Amostra Atual

Quando uma serie e usada sem indexacao, a expressao se avalia para a amostra
atual daquela serie no passo de execucao atual.

## Indexacao

Indexacao tem a forma:

```palmscript
x[n]
```

Regras:

- `n` deve ser um literal inteiro nao negativo
- indexacao dinamica e rejeitada
- apenas valores de serie podem ser indexados
- `x[0]` referencia a amostra atual
- `x[1]` referencia a amostra anterior
- `x[n]` referencia a amostra de `n` atualizacoes atras no clock de atualizacao
  da propria serie

Se nao houver historico suficiente, a expressao indexada se avalia como `na`.

## Propriedade Do Clock De Atualizacao

Toda serie avanca no seu proprio clock de atualizacao.

Exemplos:

- `bn.close[1]` segue o intervalo base
- `bb.1h.close[1]` segue a source `bb` no clock de uma hora

Series derivadas herdam os clocks de atualizacao de suas entradas. Uma serie
mais lenta nao e recontada em clocks mais rapidos quando nao avancou.

## Amostras Ausentes

Series podem produzir `na` para a amostra atual quando:

- nao ha historico suficiente
- o feed da fonte esta ausente em um passo do clock base vindo da uniao dos
  timestamps base das fontes declaradas
- a serie e um feed de intervalo superior que ainda nao fechou uma vez
- um indicador ainda esta em warmup

## Series De Tempo

`time` e uma serie numerica cuja amostra e o horario de abertura do candle em
milissegundos Unix UTC.

Regras:

- `time` base expoe o horario de abertura do candle do intervalo base
- `time` de intervalo superior expoe o horario de abertura daquele candle de
  intervalo superior
- `time` qualificado por fonte segue as mesmas regras de selecao de source e
  intervalo dos campos de preco e volume
