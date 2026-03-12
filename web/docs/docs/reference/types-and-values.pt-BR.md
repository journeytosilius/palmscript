# Tipos E Valores

PalmScript opera sobre numeros escalares, booleanos escalares, literais enum
tipados, series desses valores, `na` e `void`.

## Tipos Concretos

A implementacao distingue estes tipos concretos:

- `float`
- `bool`
- `ma_type`
- `tif`
- `trigger_ref`
- `position_side`
- `exit_kind`
- `series<float>`
- `series<bool>`
- `void`

`void` e o tipo de resultado de expressoes como `plot(...)` que nao produzem um
valor reutilizavel.

## Valores Primitivos

Valores de PalmScript assumem as seguintes formas em runtime:

- valores numericos sao `f64`
- valores booleanos sao `true` ou `false`
- valores `ma_type.<variant>` sao literais enum tipados
- valores `tif.<variant>` sao literais enum tipados
- valores `trigger_ref.<variant>` sao literais enum tipados
- valores `position_side.<variant>` sao literais enum tipados
- valores `exit_kind.<variant>` sao literais enum tipados
- `na` e o sentinela de valor ausente
- `void` nao e um literal gravavel pelo usuario

Superficie enum tipada atual:

- `ma_type.sma`
- `ma_type.ema`
- `ma_type.wma`
- `ma_type.dema`
- `ma_type.tema`
- `ma_type.trima`
- `ma_type.kama`
- `ma_type.mama`
- `ma_type.t3`
- `tif.gtc`
- `tif.ioc`
- `tif.fok`
- `tif.gtd`
- `trigger_ref.last`
- `trigger_ref.mark`
- `trigger_ref.index`
- `position_side.long`
- `position_side.short`
- `exit_kind.protect`
- `exit_kind.target`
- `exit_kind.signal`
- `exit_kind.reversal`

Todas as variantes atuais de `ma_type` sao executaveis por meio dos builtins de
media movel estilo TA-Lib; veja [TA-Lib Surface](ta-lib.md). Valores `tif`,
`trigger_ref`, `position_side` e `exit_kind` existem atualmente para
parametrizar declaracoes de ordem de backtest e o estado de posicao / saida
dirigido por backtest.

## Tipos De Serie

Valores de serie sao fluxos indexados no tempo.

Um tipo de serie:

- avanca em um clock de atualizacao
- mantem historico limitado
- expoe sua amostra atual quando usado em expressoes
- pode produzir `na` em uma dada amostra

Campos de mercado sao valores de serie. Builtins de indicador, helper de sinal
e memoria de evento tambem podem retornar valores de serie.

Alguns builtins tambem podem retornar tuplas de tamanho fixo de valores de
serie. Na implementacao atual, resultados em tupla sao suportados apenas como
resultados builtin imediatos e precisam ser desestruturados com `let (...) = ...`.

Exemplo:

```palm
let (line, signal, hist) = macd(spot.close, 12, 26, 9)
plot(hist)
```

Limites atuais do suporte a tuplas:

- valores em tupla sao produzidos apenas por builtins especificos
- valores em tupla nao podem ser armazenados como valores reutilizaveis
  ordinarios
- expressoes tuple-valued nao podem ser passadas diretamente para `plot`,
  `export`, `trigger`, condicoes ou outras expressoes
- a desestruturacao de tupla e a unica forma suportada de consumir um resultado
  em tupla

## `na`

`na` faz parte da semantica normal da linguagem. Nao e uma excecao de runtime.

`na` pode surgir de:

- historico insuficiente para indexacao
- warmup de indicador
- dados ausentes em um passo do clock base source-aware
- aritmetica ou comparacoes nas quais um operando ja e `na`
- uso explicito do literal `na`

PalmScript tambem expoe `na(value)` como um predicado builtin distinto do
literal nu `na`:

- `na` sozinho e o literal de valor ausente
- `na(expr)` retorna `bool` ou `series<bool>` dependendo do argumento
- `nz(value[, fallback])` e `coalesce(value, fallback)` sao os principais
  helpers de tratamento de nulos

## Combinacao De Serie E Escalar

PalmScript permite misturar escalares e series em expressoes quando o operador
subjacente aceita as categorias de operandos.

Regras:

- se qualquer operando aceito for `series<float>`, a aritmetica produz
  `series<float>`
- se qualquer operando aceito for `series<bool>`, operacoes logicas produzem
  `series<bool>`
- se qualquer operando aceito for `series<float>`, comparacoes numericas
  produzem `series<bool>`
- igualdade sobre qualquer operando de serie produz `series<bool>`

Isso e lifting de valor, nao materializacao implicita de uma serie ilimitada.
A avaliacao ainda segue os clocks de atualizacao descritos em
[Semantica De Avaliacao](evaluation-semantics.md).

## `na` Na Verificacao De Tipos

`na` e aceito em qualquer lugar onde uma expressao numerica ou booleana possa
ser exigida depois, sujeito ao construto ao redor.

Exemplos:

- `plot(na)` e valido
- `export x = na` e valido
- `trigger t = na` e valido
- `if na { ... } else { ... }` e valido
- `ma(spot.close, 20, ma_type.ema)` e valido

## Logica Booleana

`and` e `or` usam a logica de tres valores do PalmScript.

Eles nao convertem `na` para `false`. A tabela verdade em runtime esta definida
em [Semantica De Avaliacao](evaluation-semantics.md).

## Normalizacao De Saidas

Declaracoes de saida normalizam seus tipos de valor assim:

- `export` sobre numerico, serie numerica ou `na` produz `series<float>`
- `export` sobre booleano ou serie booleana produz `series<bool>`
- saidas `trigger`, `entry` e `exit` sempre produzem `series<bool>`

Veja [Saidas](outputs.md) para o comportamento exato das saidas.
