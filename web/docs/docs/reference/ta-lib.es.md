# Superficie TA-Lib

PalmScript ahora incluye una capa de integracion TA-Lib tipada anclada al
commit upstream `1bdf54384036852952b8b4cb97c09359ae407bd0`.

PalmScript todavia no expone todo el catalogo de funciones TA-Lib como
superficie ejecutable del lenguaje, pero si reserva el catalogo mas amplio y
usa las caracteristicas tipadas del lenguaje requeridas para esa superficie:

- literales enum `ma_type.<variant>`
- destructuracion de tuplas para builtins TA-Lib de multiples salidas
- un snapshot fijado de metadatos TA-Lib
- un catalogo generado de 161 funciones

Comportamiento actual de la superficie guiada por metadatos:

- los 161 nombres de funciones TA-Lib estan reservados como nombres builtin
- el IDE puede mostrar firmas y descripciones TA-Lib generadas en completion y
  hover
- llamar a una funcion del catalogo que aun no esta implementada produce un
  diagnostico de compilacion determinista en vez de tratarse como identificador
  desconocido
- los fixtures oracle confirmados bajo `tests/data/ta_lib/` validan el
  subconjunto implementado contra la biblioteca C upstream

En otras palabras:

- la cobertura de nombres reservados es mas amplia que la cobertura de
  ejecucion en runtime
- la visibilidad en IDE/catalogo no implica que una funcion sea ejecutable hoy
- [Builtins](builtins.md) y la seccion [Indicadores](indicators.md) son la
  documentacion autoritativa para el subconjunto ejecutable

Builtins estilo TA-Lib implementados:

- `ma(series, length, ma_type)`
- `apo(series[, fast_length=12[, slow_length=26[, ma_type=ma_type.sma]]])`
- `ppo(series[, fast_length=12[, slow_length=26[, ma_type=ma_type.sma]]])`
- `macd(series, fast_length, slow_length, signal_length)`
- `macdfix(series[, signal_length=9])`
- `macdext(series[, fast_length=12[, fast_ma=ma_type.sma[, slow_length=26[, slow_ma=ma_type.sma[, signal_length=9[, signal_ma=ma_type.sma]]]]]])`
- transformaciones matematicas unarias: `acos`, `asin`, `atan`, `ceil`, `cos`,
  `cosh`, `exp`, `floor`, `ln`, `log10`, `sin`, `sinh`, `sqrt`, `tan`, `tanh`
- operadores matematicos: `add`, `div`, `mult`, `sub`, `max`, `min`, `sum`,
  `maxindex`, `minindex`, `minmax`, `minmaxindex`
- transformaciones de precio: `avgprice`, `medprice`, `typprice`, `wclprice`
- helpers de superposicion: `accbands`, `bbands`, `dema`, `ema`, `kama`, `ma`,
  `mavp`, `midpoint`, `midprice`, `sar`, `sarext`, `sma`, `t3`, `tema`,
  `trima`, `wma`
- helpers de ciclo: `ht_dcperiod`, `ht_dcphase`, `ht_phasor`, `ht_sine`,
  `ht_trendline`, `ht_trendmode`, `mama`
- helpers estadisticos: `avgdev`, `stddev`, `var`, `linearreg`,
  `linearreg_angle`, `linearreg_intercept`, `linearreg_slope`, `tsf`, `beta`,
  `correl`
- helpers de momentum: `adx`, `adxr`, `apo`, `aroon`, `aroonosc`, `bop`, `cci`,
  `cmo`, `dx`, `imi`, `mfi`, `minus_di`, `minus_dm`, `mom`, `plus_di`,
  `plus_dm`, `ppo`, `roc`, `rocp`, `rocr`, `rocr100`, `stoch`, `stochf`,
  `stochrsi`, `trix`, `willr`
- helpers de volumen y volatilidad: `ad`, `adosc`, `atr`, `natr`, `obv`,
  `trange`

Variantes actuales de `ma_type`:

- `ma_type.sma`
- `ma_type.ema`
- `ma_type.wma`
- `ma_type.dema`
- `ma_type.tema`
- `ma_type.trima`
- `ma_type.kama`
- `ma_type.mama`
- `ma_type.t3`

Todas las variantes `ma_type` actuales son ejecutables mediante `ma(...)`,
`apo(...)`, `ppo(...)`, `bbands(...)`, `macdext(...)`, `mavp(...)`, `stoch(...)`,
`stochf(...)` y `stochrsi(...)`. Para la familia generica de medias moviles
TA-Lib, `ma_type.mama` sigue el comportamiento upstream: ignora el parametro
explicito `length` y usa los defaults de MAMA `fast_limit=0.5` y
`slow_limit=0.05`.

Defaults TA-Lib actuales respetados por la superficie ejecutable:

- `max`, `min` y `sum` usan una ventana por defecto de `30`
- `midpoint` y `midprice` usan una ventana por defecto de `14`
- `wma`, `maxindex`, `minindex`, `minmax` y `minmaxindex` usan una ventana por
  defecto de `30`
- `avgdev` usa por defecto una ventana de `14`
- `stddev` y `var` usan `length=5` por defecto
- `linearreg`, `linearreg_angle`, `linearreg_intercept`, `linearreg_slope` y
  `tsf` usan `length=14` por defecto
- `beta` usa `length=5` por defecto y su calculo beta basado en retornos de
  TA-Lib
- `correl` usa `length=30` por defecto
- `apo` y `ppo` usan `fast_length=12`, `slow_length=26` y `ma_type.sma` por
  defecto
- `macdfix` usa `signal_length=9` por defecto
- `macdext` usa `fast_length=12`, `slow_length=26`, `signal_length=9` y
  `ma_type.sma` para los tres roles MA por defecto
- `bbands` usa `length=5`, `deviations_up=2`, `deviations_down=2` y
  `ma_type.sma` por defecto
- `accbands` usa `length=20` por defecto
- `mavp` requiere `minimum_period`, `maximum_period` y `ma_type` explicitos
- `sar` usa `acceleration=0.02` y `maximum=0.2` por defecto
- `sarext` usa `start_value=0`, `offset_on_reverse=0`, `af_init_long=0.02`,
  `af_long=0.02`, `af_max_long=0.2`, `af_init_short=0.02`, `af_short=0.02` y
  `af_max_short=0.2` por defecto
- `aroon` y `aroonosc` usan `length=14` por defecto
- `atr`, `natr`, `plus_dm`, `minus_dm`, `plus_di`, `minus_di`, `dx`, `adx`,
  `adxr`, `mfi` e `imi` usan `length=14` por defecto
- `adosc` usa `fast_length=3` y `slow_length=10` por defecto
- `cci` usa `length=14` por defecto
- `cmo` usa `length=14` por defecto
- `dema`, `tema`, `trima`, `kama` y `trix` usan `length=30` por defecto
- `t3` usa `length=5` y `volume_factor=0.7` por defecto
- `mama` usa `fast_limit=0.5` y `slow_limit=0.05` por defecto
- `mom`, `roc`, `rocp`, `rocr` y `rocr100` usan `length=10` por defecto
- `stoch` usa `fast_k=5`, `slow_k=3`, `slow_d=3` y `ma_type.sma` en ambas
  etapas de suavizado por defecto
- `stochf` usa `fast_k=5`, `fast_d=3` y `ma_type.sma` por defecto
- `stochrsi` usa `time_period=14`, `fast_k=5`, `fast_d=3` y `ma_type.sma` por
  defecto
- `willr` usa `length=14` por defecto

Actualizacion de fixtures oracle para el subconjunto implementado:

```bash
python3 tools/generate_talib_fixtures.py
cargo test --test ta_lib_parity
```

Ejemplo de retorno tuple:

```palm
interval 1m
source spot = binance.spot("BTCUSDT")

let (line, signal, hist) = macd(spot.close, 12, 26, 9)
plot(line)
plot(signal)
plot(hist)
```
