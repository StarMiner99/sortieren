# Sortier Algorithmus (Bubble Sort)

Name: Thijs Boersma
Kurs: TINF24B1
Datum: 01.07.2025

## Beschreibung
Im folgenden wird die Zeit gemessen die benötigt wird um eine n-Elementige liste mithilfe des Bubble-sort algorithmus zu sortieren.

Jeder Versuch wurde 3 mal durchgeführt und der Mitelwert gebildet.

Folgende Versuche werden durchgeführt:
- Vorsortierte Liste:
    - 1000, 10 000, 100 000 Elemente
- Invers-Vorsortierte Liste:
    - 1000, 10 000, 100 000 Elemente
- Zufällige Liste:
    - 1000, 10 000, 100 000 Elemente

## Erwartete Ergebnisse

Am schnellsten wird die Vorsortierte Liste, "nochmals" sortiert.

Darauf folgt die Zufällige Liste, da diese durchschnittlich viele Vertauschungen und Vergleiche benötigt.

Am längsten dauert die Invers-Vorsortierte Liste, da hier jedes Element getauscht werden muss.

## Durchführung

### Zufällige Liste:

| n Elemente | 1.     | 2.     | 3.     | AVG      |
|------------|--------|--------|--------|----------|
| 1000       | 275us  | 287us  | 272us  | 278us    |
| 10 000     | 24ms   | 23ms   | 23ms   | 23,3ms   |
| 100 000    | 7303ms | 7356ms | 7360ms | 7339,6ms |

### Vorsortierte Liste:

| n Elemente | 1.     | 2.     | 3.     | AVG      |
|------------|--------|--------|--------|----------|
| 1000       | 115us  | 116us  | 116us  | 115,3ms  |
| 10 000     | 11ms   | 11ms   | 11ms   | 11ms     |
| 100 000    | 1127ms | 1144ms | 1192ms | 1154,3ms |

### Invers-Vorsortierte Liste:

| n Elemente | 1.     | 2.     | 3.     | AVG     |
|------------|--------|--------|--------|---------|
| 1000       | 158us  | 133us  | 133us  | 141,3us |
| 10 000     | 15ms   | 16ms   | 15ms   | 15,3ms  |
| 100 000    | 1632ms | 1605ms | 1656ms | 1631ms  |

## Ergebniss

Aus Mysteriösen Gründen ist es schneller eine invers sortierte liste zu sortieren als eine zufällige.
