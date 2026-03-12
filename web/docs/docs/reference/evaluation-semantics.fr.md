# Semantique Devaluation

Cette page definit comment les expressions et instructions PalmScript sont
evaluees a l'execution.

## Modele D'Execution

PalmScript compile un script une fois puis l'evalue une fois par pas de
l'horloge de base.

A chaque etape :

1. le runtime materialise les echantillons courants des series de marche pour
   cette etape
2. les flux d'intervalle plus lent n'avancent que si leurs bougies sont
   completement cloturees a cette etape
3. le programme bytecode s'execute
4. les sorties `plot`, `export` et `trigger` sont collectees pour cette etape

Les differents flux en mode marche peuvent construire les entrees d'etape de
maniere differente, mais l'evaluation des expressions reste la meme une fois
l'etape commencee.

## Categories D'Expressions

Les expressions s'evaluent a l'echantillon courant d'une valeur scalaire ou
d'une valeur de serie.

Pour une expression de serie :

- le resultat a une etape est un unique echantillon courant
- l'indexation adresse des echantillons precedents sur l'horloge de mise a
  jour propre a cette expression

## Precedence Des Operateurs

PalmScript evalue les operateurs dans cet ordre, du plus faible au plus fort :

1. `or`
2. `and`
3. `==`, `!=`, `<`, `<=`, `>`, `>=`
4. `+`, `-`
5. `*`, `/`
6. unaire `-`, unaire `!`
7. les postfixes d'appel, d'indexation et de qualification

Les operateurs de meme precedence s'associent de gauche a droite.

## Arithmetique

Les operateurs arithmetiques sont `+`, `-`, `*` et `/`.

Regles :

- les deux operandes doivent etre numeriques, series numeriques ou `na`
- si un operande est `na`, le resultat est `na`
- si un operande est `series<float>`, le resultat est `series<float>`
- sinon le resultat est `float`

## Comparaisons

Les operateurs de comparaison sont `==`, `!=`, `<`, `<=`, `>`, `>=`.

Regles :

- `<`, `<=`, `>`, `>=` exigent des operandes numeriques
- `==` et `!=` sont definis pour tout operande non-`na`
- l'egalite entre types differents est fausse
- si un operande est `na`, le resultat est `na`
- si un operande est une serie, le resultat est un booleen de serie
- sinon le resultat est `bool`

## Operateurs Unaires

PalmScript prend en charge :

- l'unaire `-` pour les operandes numeriques
- l'unaire `!` pour les operandes booleens

Regles :

- les operateurs unaires propagent `na`
- l'unaire `-` sur `series<float>` produit `series<float>`
- l'unaire `!` sur `series<bool>` produit `series<bool>`

## Operateurs Logiques

`and` et `or` exigent `bool`, `series<bool>` ou `na`.

Ils utilisent une logique deterministe a trois valeurs :

### `and`

| Left | Right | Result |
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

| Left | Right | Result |
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

PalmScript evalue les deux operandes avant d'appliquer l'operateur logique. Le
langage ne garantit pas l'evaluation court-circuit, de sorte que les
expressions logiques sont analysees et executees de maniere eager selon les
regles normales du langage.

## Semantique De `if`

`if` est une forme d'instruction, pas une forme d'expression.

Regles :

- la condition doit s'evaluer en `bool`, `series<bool>` ou `na`
- `na` dans une condition `if` est traite comme faux pour le choix de branche
- une seule branche s'execute a chaque etape
- les deux branches doivent etre syntaxiquement presentes car `else` est
  obligatoire

## Evaluation Des Fonctions

Les fonctions definies par l'utilisateur ont un corps d'expression et sont
compilees par specialisation plutot que par dispatch dynamique a l'execution.

Regles :

- le nombre d'arguments doit correspondre au nombre de parametres declares
- les fonctions sont specialisees par type d'argument et horloge de mise a jour
- les graphes de fonctions recursifs et cycliques sont rejetes a la compilation
- un corps de fonction ne peut pas appeler `plot`

## Regle Sans Lookahead

PalmScript ne doit pas exposer des bougies d'intervalle superieur partiellement
formees.

Consequences :

- une serie d'intervalle superieur ne change qu'apres la cloture complete de
  cette bougie
- l'indexation sur une serie d'intervalle superieur parcourt l'historique des
  echantillons d'intervalle superieur completement clotures
- les intervalles supplementaires source-aware suivent la meme regle

## Semantique Des Helpers Builtin

Les formules des helpers builtin, les contrats des indicateurs, les regles de
fenetre et le comportement de `na` sont definis dans [Builtins](builtins.md) et
la section [Indicateurs](indicators.md).

Regles :

- les helpers builtin suivent les horloges de mise a jour de leurs entrees
  series
- les sorties des helpers participent a `if`, a l'indexation et aux autres
  appels builtin en suivant les memes regles de valeur et de `na` definies sur
  cette page
- `if` traite toujours `na` comme faux pour le choix de branche, meme lorsque
  la condition provient d'un helper comme `crossover(...)`

## Determinisme

L'evaluation des expressions est deterministe.

Pendant l'execution d'une strategie, la semantique du langage depend
uniquement :

- du programme compile
- des flux d'entree prepares
- des limites de VM configurees

Elle ne depend ni de l'heure murale, ni du systeme de fichiers, ni du hasard,
ni d'un acces reseau.
