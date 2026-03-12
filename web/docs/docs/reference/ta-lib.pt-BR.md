# TA-Lib Surface

PalmScript agora inclui uma camada de integracao tipada com TA-Lib ancorada no
commit upstream `1bdf54384036852952b8b4cb97c09359ae407bd0`.

PalmScript ainda nao expoe o catalogo completo de funcoes do TA-Lib como
superficie de linguagem executavel, mas reserva esse catalogo mais amplo e usa
os recursos tipados da linguagem necessarios para essa superficie:

- literais enum `ma_type.<variant>`
- desestruturacao de tuplas para builtins TA-Lib com multiplas saidas
- um snapshot fixado de metadata do TA-Lib
- um catalogo gerado com 161 funcoes

Comportamento atual da superficie guiada por metadata:

- todos os 161 nomes de funcao do TA-Lib sao reservados como nomes builtin
- autocomplete e hover no IDE podem mostrar assinaturas e resumos TA-Lib
  gerados
- chamar uma funcao do catalogo ainda nao implementada produz um diagnostico de
  compilacao deterministico em vez de ser tratada como identificador
  desconhecido
- fixtures oracle commitadas em `tests/data/ta_lib/` agora validam o
  subconjunto implementado contra a biblioteca C upstream

Em outras palavras:

- a cobertura de nomes reservados e maior do que a cobertura de execucao em
  runtime
- visibilidade em IDE / catalogo nao implica que uma funcao seja executavel
- [Builtins](builtins.md) e a secao [Indicators](indicators.md) sao a
  documentacao autoritativa do subconjunto executavel

Builtins estilo TA-Lib implementados:

- `ma(series, length, ma_type)`
- `apo(series[, fast_length=12[, slow_length=26[, ma_type=ma_type.sma]]])`
- `ppo(series[, fast_length=12[, slow_length=26[, ma_type=ma_type.sma]]])`
- `macd(series, fast_length, slow_length, signal_length)`
- `macdfix(series[, signal_length=9])`
- `macdext(series[, fast_length=12[, fast_ma=ma_type.sma[, slow_length=26[, slow_ma=ma_type.sma[, signal_length=9[, signal_ma=ma_type.sma]]]]]])`
- transformacoes matematicas unarias: `acos`, `asin`, `atan`, `ceil`, `cos`,
  `cosh`, `exp`, `floor`, `ln`, `log10`, `sin`, `sinh`, `sqrt`, `tan`,
  `tanh`
- operadores matematicos: `add`, `div`, `mult`, `sub`, `max`, `min`, `sum`,
  `maxindex`, `minindex`, `minmax`, `minmaxindex`
- transformacoes de preco: `avgprice`, `medprice`, `typprice`, `wclprice`
- helpers de overlap: `accbands`, `bbands`, `dema`, `ema`, `kama`, `ma`,
  `mavp`, `midpoint`, `midprice`, `sar`, `sarext`, `sma`, `t3`, `tema`,
  `trima`, `wma`
- helpers de ciclo: `ht_dcperiod`, `ht_dcphase`, `ht_phasor`, `ht_sine`,
  `ht_trendline`, `ht_trendmode`, `mama`
- helpers estatisticos: `avgdev`, `stddev`, `var`, `linearreg`,
  `linearreg_angle`, `linearreg_intercept`, `linearreg_slope`, `tsf`, `beta`,
  `correl`
- helpers de momentum: `adx`, `adxr`, `apo`, `aroon`, `aroonosc`, `bop`,
  `cci`, `cmo`, `dx`, `imi`, `mfi`, `minus_di`, `minus_dm`, `mom`, `plus_di`,
  `plus_dm`, `ppo`, `roc`, `rocp`, `rocr`, `rocr100`, `stoch`, `stochf`,
  `stochrsi`, `trix`, `willr`
- helpers de volume e volatilidade: `ad`, `adosc`, `atr`, `natr`, `obv`,
  `trange`

Variantes atuais de `ma_type`:

- `ma_type.sma`
- `ma_type.ema`
- `ma_type.wma`
- `ma_type.dema`
- `ma_type.tema`
- `ma_type.trima`
- `ma_type.kama`
- `ma_type.mama`
- `ma_type.t3`

Todas as variantes atuais de `ma_type` sao executaveis por meio de `ma(...)`,
`apo(...)`, `ppo(...)`, `bbands(...)`, `macdext(...)`, `mavp(...)`,
`stoch(...)`, `stochf(...)` e `stochrsi(...)`. Para a familia generica de
medias moveis do TA-Lib, `ma_type.mama` segue o comportamento do TA-Lib
upstream: ignora o parametro explicito `length` e usa os defaults de MAMA
`fast_limit=0.5` e `slow_limit=0.05`.

Defaults atuais do TA-Lib respeitados pela superficie executavel:

- `max`, `min` e `sum` usam janela default `30`
- `midpoint` e `midprice` usam janela default `14`
- `wma`, `maxindex`, `minindex`, `minmax` e `minmaxindex` usam janela default
  `30`
- `avgdev` usa janela default `14`
- `stddev` e `var` usam `length=5` por padrao
- `linearreg`, `linearreg_angle`, `linearreg_intercept`, `linearreg_slope` e
  `tsf` usam `length=14` por padrao
- `beta` usa `length=5` por padrao e o calculo de beta baseado em retorno do
  TA-Lib
- `correl` usa `length=30` por padrao
- `apo` e `ppo` usam `fast_length=12`, `slow_length=26` e `ma_type.sma` por
  padrao
- `macdfix` usa `signal_length=9` por padrao
- `macdext` usa `fast_length=12`, `slow_length=26`, `signal_length=9` e
  `ma_type.sma` para os tres roles de media movel
- `bbands` usa `length=5`, `deviations_up=2`, `deviations_down=2` e
  `ma_type.sma` por padrao
- `accbands` usa `length=20` por padrao
- `mavp` exige `minimum_period`, `maximum_period` e `ma_type` explicitos
- `sar` usa `acceleration=0.02` e `maximum=0.2` por padrao
- `sarext` usa `start_value=0`, `offset_on_reverse=0`, `af_init_long=0.02`,
  `af_long=0.02`, `af_max_long=0.2`, `af_init_short=0.02`, `af_short=0.02` e
  `af_max_short=0.2` por padrao
- `aroon` e `aroonosc` usam `length=14` por padrao
- `atr`, `natr`, `plus_dm`, `minus_dm`, `plus_di`, `minus_di`, `dx`, `adx`,
  `adxr`, `mfi` e `imi` usam `length=14` por padrao
- `adosc` usa `fast_length=3` e `slow_length=10` por padrao
- `cci` usa `length=14` por padrao
- `cmo` usa `length=14` por padrao
- `dema`, `tema`, `trima`, `kama` e `trix` usam `length=30` por padrao
- `t3` usa `length=5` e `volume_factor=0.7` por padrao
- `mama` usa `fast_limit=0.5` e `slow_limit=0.05` por padrao
- `mom`, `roc`, `rocp`, `rocr` e `rocr100` usam `length=10` por padrao
- `stoch` usa `fast_k=5`, `slow_k=3`, `slow_d=3` e `ma_type.sma` para os dois
  estagios de suavizacao
- `stochf` usa `fast_k=5`, `fast_d=3` e `ma_type.sma` por padrao
- `stochrsi` usa `time_period=14`, `fast_k=5`, `fast_d=3` e `ma_type.sma` por
  padrao
- `willr` usa `length=14` por padrao

Atualizacao das fixtures oracle para o subconjunto implementado:

```bash
python3 tools/generate_talib_fixtures.py
cargo test --test ta_lib_parity
```

Exemplo de retorno em tupla:

```palm
interval 1m
source spot = binance.spot("BTCUSDT")

let (line, signal, hist) = macd(spot.close, 12, 26, 9)
plot(line)
plot(signal)
plot(hist)
```
