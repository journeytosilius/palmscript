# Types Et Valeurs

PalmScript opere sur des nombres scalaires, des booleens scalaires, des
litteraux enum types, des series de ces valeurs, `na` et `void`.

## Types Concrets

L'implementation distingue ces types concrets :

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

`void` est le type de resultat d'expressions comme `plot(...)` qui ne
renvoient pas de valeur reutilisable.

## Valeurs Primitives

Les valeurs PalmScript prennent les formes runtime suivantes :

- les valeurs numeriques sont des `f64`
- les valeurs booleennes sont `true` ou `false`
- les valeurs `ma_type.<variant>` sont des litteraux enum types
- les valeurs `tif.<variant>` sont des litteraux enum types
- les valeurs `trigger_ref.<variant>` sont des litteraux enum types
- les valeurs `position_side.<variant>` sont des litteraux enum types
- les valeurs `exit_kind.<variant>` sont des litteraux enum types
- `na` est le sentinelle de valeur manquante
- `void` n'est pas un litteral ecrivable par l'utilisateur

Surface actuelle des enums types :

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

Toutes les variantes `ma_type` actuelles sont executables via les builtins de
moyenne mobile de style TA-Lib ; voir [TA-Lib Surface](ta-lib.md). Les valeurs
`tif`, `trigger_ref`, `position_side` et `exit_kind` existent actuellement pour
parametrer les declarations d'ordre de backtest ainsi que l'etat de position et
de sortie pilote par le backtest.

## Types Series

Les valeurs de serie sont des flux indexes dans le temps.

Un type serie :

- avance sur une horloge de mise a jour
- conserve un historique borne
- expose son echantillon courant lorsqu'il est utilise dans des expressions
- peut produire `na` sur un echantillon donne

Les champs de marche sont des valeurs de serie. Les builtins d'indicateurs,
d'aide aux signaux et de memoire d'evenement peuvent aussi renvoyer des
valeurs de serie.

Certains builtins peuvent aussi renvoyer des tuples de taille fixe de valeurs
de serie. Dans l'implementation actuelle, les resultats tuple sont pris en
charge uniquement comme resultats immediats de builtin et doivent etre
destructures avec `let (...) = ...`.

Exemple :

```palm
let (line, signal, hist) = macd(spot.close, 12, 26, 9)
plot(hist)
```

Limites actuelles de la prise en charge des tuples :

- seules certaines builtins produisent des valeurs tuple
- les valeurs tuple ne peuvent pas etre stockees comme valeurs reutilisables
  ordinaires
- les expressions a valeur tuple ne peuvent pas etre passees directement a
  `plot`, `export`, `trigger`, aux conditions ou a d'autres expressions
- la destructuration de tuple est la seule facon prise en charge de consommer
  un resultat tuple

## `na`

`na` fait partie de la semantique normale du langage. Ce n'est pas une
exception runtime.

`na` peut provenir de :

- un historique insuffisant pour l'indexation
- le warmup d'un indicateur
- des donnees manquantes sur un pas d'horloge de base source-aware
- une arithmetique ou une comparaison dont un operande est deja `na`
- l'utilisation explicite du litteral `na`

PalmScript expose aussi `na(value)` comme predicat builtin distinct du
litteral nu `na` :

- `na` seul est le litteral de valeur manquante
- `na(expr)` renvoie `bool` ou `series<bool>` selon l'argument
- `nz(value[, fallback])` et `coalesce(value, fallback)` sont les principaux
  helpers de gestion du null

## Combinaison Serie Et Scalaire

PalmScript autorise le melange scalaire / serie dans les expressions quand
l'operateur sous-jacent accepte les categories d'operandes.

Regles :

- si l'un des operandes acceptes est `series<float>`, l'arithmetique produit
  `series<float>`
- si l'un des operandes acceptes est `series<bool>`, les operations logiques
  produisent `series<bool>`
- si l'un des operandes acceptes est `series<float>`, les comparaisons
  numeriques produisent `series<bool>`
- l'egalite sur n'importe quel operande serie produit `series<bool>`

Il s'agit d'un lifting de valeur, pas d'une materialisation implicite d'une
serie non bornee. L'evaluation suit toujours les horloges de mise a jour
decrites dans [Semantique d'evaluation](evaluation-semantics.md).

## `na` Dans Le Typage

`na` est accepte partout ou une expression numerique ou booleenne peut etre
requise par la suite, sous reserve de la construction englobante.

Exemples :

- `plot(na)` est valide
- `export x = na` est valide
- `trigger t = na` est valide
- `if na { ... } else { ... }` est valide
- `ma(spot.close, 20, ma_type.ema)` est valide

## Logique Booleenne

`and` et `or` utilisent la logique a trois valeurs de PalmScript.

Ils ne coercissent pas `na` en `false`. Leur table de verite runtime est
definie dans [Semantique d'evaluation](evaluation-semantics.md).

## Normalisation Des Sorties

Les declarations de sortie normalisent leurs types de valeur comme suit :

- `export` sur numerique, serie numerique ou `na` produit `series<float>`
- `export` sur booleen ou serie booleenne produit `series<bool>`
- les sorties `trigger`, `entry` et `exit` produisent toujours `series<bool>`

Voir [Sorties](outputs.md) pour le comportement exact des sorties.
