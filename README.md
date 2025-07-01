# Sortier Algorithmus (Bubble Sort)

Name: Thijs Boersma
Kurs: TINF24B1
Datum: 01.07.2025

## Beschreibung
Im folgenden wird die Zeit gemessen die benötigt wird um eine n-Elementige liste mithilfe des Bubble-sort algorithmus zu sortieren.

Im nachhinein wurden zusätzlich die Anzahl der durchgeführten Vertauschungen (swaps) in separaten Durchläufen gezählt. (bei 100 000 Elementen)

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

Anzahl Swaps: 
- 1.: 2503145312
- 2.: 2506687965
- 3.: 2499054780

### Vorsortierte Liste:

| n Elemente | 1.     | 2.     | 3.     | AVG      |
|------------|--------|--------|--------|----------|
| 1000       | 115us  | 116us  | 116us  | 115,3ms  |
| 10 000     | 11ms   | 11ms   | 11ms   | 11ms     |
| 100 000    | 1127ms | 1144ms | 1192ms | 1154,3ms |

Anzahl Swaps: 
- 1.: 0
- 2.: 0
- 3.: 0

### Invers-Vorsortierte Liste:

| n Elemente | 1.     | 2.     | 3.     | AVG     |
|------------|--------|--------|--------|---------|
| 1000       | 158us  | 133us  | 133us  | 141,3us |
| 10 000     | 15ms   | 16ms   | 15ms   | 15,3ms  |
| 100 000    | 1632ms | 1605ms | 1656ms | 1631ms  |

Anzahl Swaps: 
- 1.: 4999950000
- 2.: 4999950000
- 3.: 4999950000

AVG Swaps: 4999950000


## Ergebniss
Die Vorsortierte Liste ist am schnellsten, was sinn ergibt, da 0 swaps durchgeführt werden.

Trotz erhöter menge an Swaps wird die Invers-Vorsortierte Liste schneller sortiert als die Zufällig Sortierte Liste.

Auch das verwenden einer quadratisch absteigenden Liste, hat das ergebniss nicht geändert.

Das nutzen einer zufällig absteigenden Liste wäre ein nächster Ansatz um festzustellen, ob hier die Zeiten ebenfalls vom erwartetem Ergebniss abweichen.

-> Das Verhalten lässt sich nur durch Rust Compiler Optimierungen erklären. Wie genau diese Funktionieren, habe ich (noch) nicht herausgefunden.

## Code
```rust
use std::time::{SystemTime, UNIX_EPOCH};

const LIST_LEN: usize = 100_000;
fn main() {
    // random list:
    let mut rand_array: [u64; LIST_LEN] = rand::random();

    // sorted list:
    //let mut rand_array: [u64; LIST_LEN] = core::array::from_fn(|i| i as u64);
    //rand_array.reverse(); // comment this for not-reversed list

    println!("{} {}", rand_array[0], rand_array[LIST_LEN -1]);

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let swaps = bubble_sort(&mut rand_array);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let duration = end - start;

    println!("Took {}ms for {} elements. {} swaps", duration.as_millis(), LIST_LEN, swaps);
    assert!(rand_array.windows(2).all(|w| w[0] <= w[1]), "Result is not sorted!");
}

fn bubble_sort(arr: &mut [u64; LIST_LEN]) -> usize {
    let last_check = LIST_LEN;
    let mut swaps = 0;

    for end_index in (1..last_check).rev() {
        for i in 0..end_index {
            if arr[i] > arr[i+1] {
                let temp = arr[i];
                arr[i] = arr[i+1];
                arr[i+1] = temp;
                swaps += 1;
            }
        }
    }

    swaps
}
```