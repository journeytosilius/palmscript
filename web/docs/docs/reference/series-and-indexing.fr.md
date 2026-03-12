# Series Et Indexation

Les valeurs de serie representent des echantillons indexes dans le temps avec
un historique borne.

## Formes Des Series De Marche

PalmScript expose les series de marche uniquement via des formes qualifiees par
source :

```palmscript
bn.close
hl.1h.close
```

Regles :

- `<alias>.<field>` reference cette source sur l'intervalle de base du script
- `<alias>.<interval>.<field>` reference cette source sur l'intervalle nomme
- les identifiants de marche nus comme `close` sont rejetes
- les references a un intervalle source superieur exigent une declaration
  `use <alias> <interval>` correspondante

## Semantique De L'Echantillon Courant

Lorsqu'une serie est utilisee sans indexation, l'expression s'evalue a
l'echantillon courant de cette serie sur l'etape d'execution en cours.

## Indexation

L'indexation a la forme :

```palmscript
x[n]
```

Regles :

- `n` doit etre un litteral entier non negatif
- l'indexation dynamique est rejetee
- seules les valeurs de serie peuvent etre indexees
- `x[0]` reference l'echantillon courant
- `x[1]` reference l'echantillon precedent
- `x[n]` reference l'echantillon d'il y a `n` mises a jour sur l'horloge de
  mise a jour propre a cette serie

Si l'historique est insuffisant, l'expression indexee s'evalue a `na`.

## Propriete De L'Horloge De Mise A Jour

Chaque serie avance sur sa propre horloge de mise a jour.

Exemples :

- `bn.close[1]` suit l'intervalle de base
- `hl.1h.close[1]` suit la source `hl` sur l'horloge horaire

Les series derivees heritent des horloges de mise a jour de leurs entrees. Une
serie plus lente n'est pas recomptee sur des horloges plus rapides quand elle
n'a pas avance.

## Echantillons Manquants

Une serie peut produire `na` pour l'echantillon courant lorsque :

- l'historique est insuffisant
- le flux source est manquant sur une etape de l'horloge de base issue de
  l'union des timestamps de base des sources declarees
- la serie est un flux d'intervalle superieur qui ne s'est pas encore cloture
  une premiere fois
- un indicateur est encore en phase de warmup

## Series Temporelles

`time` est une serie numerique dont l'echantillon est l'heure d'ouverture de
la bougie en millisecondes Unix UTC.

Regles :

- `time` de base expose l'heure d'ouverture de la bougie d'intervalle de base
- `time` d'intervalle superieur expose l'heure d'ouverture de cette bougie
  d'intervalle superieur
- `time` qualifie par source suit les memes regles de selection de source et
  d'intervalle que les champs de prix et de volume
