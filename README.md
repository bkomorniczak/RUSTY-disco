# Opis projektu

## Główne funkcje programu
Program pozwala na:
- szyfrowanie i deszyfrowanie tekstu przy użyciu szyfru podstawieniowego,
- obliczenie monogramów, bigramów, trigramów, quadgramów oraz prawdopodobieństw ich wystąpienia w tekście,
- obliczenie wartości T dla każdego z ngramów.

### Funkcja main
Funkcja main odpowiada za obsługę argumentów programu oraz wywołanie odpowiednich funkcji. 
Główne elementy funkcji main:
- Wczytanie argumentów programu. Wczytywanie komend z argumentów programu odbywa się przy użyciu biblioteki clap. 
Używane są następujące flagi:
  - -i - ścieżka do pliku z tekstem wejściowym,
  - -o - ścieżka do pliku z tekstem wyjściowym,
  - -k - ścieżka do pliku z kluczem,
  - --g1 - ścieżka do pliku z monogramami,
  - --g2 - ścieżka do pliku z bigramami,
  - --g3 - ścieżka do pliku z trigramami,
  - --g4 - ścieżka do pliku z quadgramami,
  - --ri - ścieżka do pliku referencyjnego z ngramami,
  - --ro - ścieżka do pliku referencyjnego z prawdopodobieństwami ngramów,
  - --p - ścieżka do pliku z prawdopodobieństwami ngramów,
  - --t1 - flaga oznaczająca obliczenie wartości T dla monogramów,
  - --t2 - flaga oznaczająca obliczenie wartości T dla bigramów,
  - --t3 - flaga oznaczająca obliczenie wartości T dla trigramów,
  - --t4 - flaga oznaczająca obliczenie wartości T dla quadgramów.
### handle_encryption - funkcja odpowiadająca za szyfrowanie tekstu
```rust
fn handle_encryption(plain_path: &str, encrypted_path: &str, dictionary_path: &str) -> io::Result<()> {
    match encrypt::encrypt_file(plain_path, encrypted_path, dictionary_path) {
        Err(e) => {
            eprintln!("Error encrypting file: {}", e);
            Err(e)
        },
        Ok(_) => {
            println!("File encrypted successfully.");
            Ok(())
        }
    }
}
```
Jako argumenty przyjmuje ścieżkę do pliku z tekstem wejściowym, ścieżkę do pliku z tekstem wyjściowym oraz ścieżkę do pliku z kluczem.
Funkcja zwraca wynik operacji w postaci Result<()>. 
W przypadku wystąpienia błędu zwraca komunikat o jego wystąpieniu, w przeciwnym wypadku komunikat informujący o sukcesie.
Żeby zaszyfrować tekst, wywoływana jest funkcja encrypt_file z modułu encrypt. 
Szyfrowanie pliku zostało zaimplementowane w osobnym module dla lepszej czytelności kodu. Znajdują się tam funkcje:
### encrypt_file - funkcja odpowiadająca za szyfrowanie pliku
```rust
pub fn encrypt_file(plain_path: &str, encrypted_path: &str, dictionary_path: &str) -> io::Result<()> {
let dictionary_map = read_dictionary_to_map(dictionary_path)?;


    let plain_text = fs::read_to_string(plain_path)?;
    let filtered_text: String = plain_text.chars().filter(|c| c.is_alphabetic()).collect();

    let mut encrypted_text = String::new();

    let capital_plain_text = filtered_text.to_uppercase();
    for c in capital_plain_text.chars() {
        if let Some(&encrypted_char) = dictionary_map.get(&c) {
            encrypted_text.push(encrypted_char);
        } else {
            encrypted_text.push(c);
        }
    }
    fs::write(encrypted_path, encrypted_text)?;
    Ok(())
}
```
W tej funkcji wczytywany jest słownik z pliku, a następnie wczytywany jest tekst wejściowy. 
Tekst wejściowy jest filtrowany, aby usunąć znaki inne niż litery. 
Następnie tekst jest zamieniany na duże litery. 
Dla każdej litery z tekstu wejściowego, jeśli litera znajduje się w słowniku, to litera jest zamieniana na zaszyfrowaną literę. 
W przeciwnym wypadku litera jest pozostawiana bez zmian. Zaszyfrowany tekst jest zapisywany do pliku.
### read_dictionary_to_map - funkcja odpowiadająca za wczytanie słownika do mapy
```rust
pub fn read_dictionary_to_map(dictionary_path: &str) -> io::Result<HashMap<char, char>> {
  let mut map = HashMap::new();
  let content = fs::read_to_string(dictionary_path)?;

  for line in content.lines() {
  let parts: Vec<&str> = line.split(' ').collect();
  if parts.len() == 2 {
  let (plain, encrypted) = (parts[0].chars().next(), parts[1].chars().next());
  if let (Some(p), Some(e)) = (plain, encrypted) {
  map.insert(p, e);
        }
      }
    }
  Ok(map)
  }
```
W tej funkcji najpierw inicjalizowana jest pusta HashMapa - do niej będą zapisywane pary <litera_do_zaszyfrowania, litera_klucza>.
Żeby to osiągnąć najpierw iterujemy po wczytanej z pliku zawartości. Każda linia jest dzielona na dwie cześci po " ".
Wynik zapisywany jest do wektora parts. Następnie iterujemy po tym wektorze (pod warunkiem, że każda czesc ma dlugość 2 - czyli, ze każdej literze 
"do zaszyfrowania" odpowiada element klucza). Potem ta dwójka jest rozpakowywana z Optional<char> i zapisywana do mapy. 
### count_ngrams - funkcja odpowiadająca za zliczanie ngramów
```rust
fn count_ngrams(text: &str, n: u32) -> Vec<(String, u32)> {
    let mut counts = HashMap::new();
    let chars = text.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_uppercase().next().unwrap())
        .collect::<Vec<_>>();
    for window in chars.windows(n as usize) {
        let ngram = window.iter().collect::<String>();
        *counts.entry(ngram).or_insert(0) += 1;
    }

    let mut counts_vec: Vec<(String, u32)> = counts.into_iter().collect();
    counts_vec.sort_by(|a, b| b.1.cmp(&a.1));
    counts_vec
}
```
Funkcja przyjmuje dwa argumenty - test do analizy i wielkosc ngramu. Najpierw tekst jest przefiltrowywany - usuwane są z
niego wszyskie znaki nie będące literami. Następnie używając kolejnej niejawowej struktury danych - window - iteruję po tekście
i zliczam występowanie ngramu. 
```
        *counts.entry(ngram).or_insert(0) += 1;
```
Ta linijka sprawdza czy w Hashmapie jest juz szukany przez nas ngram, jesli tak inkrementuje ilosc jego wystapien, jesli nie,
najpierw zapisuje go do hashmapy z iloscia wystapien 0, a potem inkrementuje tą wartość. 
na koniec sortuje wynikowy wektor, żeby łatwiej móc odczytać ilość wystąpień poszczególnych ngramow.
Zwraca bardzo niejavowy wektor tupli - ngram i ilosc jego powtorzeń.


Pozostałe funkcje w projekcie:
- save_ngrams - funkcja odpowiadająca za zapisywanie ngramów do pliku
- sum_values_in_file - funkcja odpowiadająca za zliczanie wszystkich ngramow w pliku
- calculate_and_save_ngram_probabilities - funkcja odpowiadająca za obliczanie i zapisywanie do pliku prawdopodobieństw ngramów
- calculate_t - funkcja odpowiadająca za obliczanie wartości T dla ngramów zgodnie ze wzorem z zadania
- read_probabilities - funkcja odpowiadająca za wczytywanie prawdopodobieństw ngramów z pliku



## Komendy

### Szyfruj
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt
```
### Oblicz monogramy
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt --g1 src/resource/monogram.txt
```
### Oblicz bigramy
``` 
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt --g2 src/resource/bigram.txt
```
### Oblicz trigramy
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt --g3 src/resource/trigram.txt
```
### Oblicz quadgramy
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt --g4 src/resource/quadgram.txt
```
### Oblicz prawdopodobieństwa quadgramów
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt --ri src/resource/quadgram.txt --ro src/resource/quadgram_probabilities.txt
```
### Oblicz prawdopodobieństwa trigramów
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt --ri src/resource/trigram.txt --ro src/resource/trigram_probabilities.txt
```
### Oblicz prawdopodobieństwa bigramów
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt --ri src/resource/bigram.txt --ro src/resource/bigram_probabilities.txt
```
### Oblicz prawdopodobieństwa monogramów
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt --ri src/resource/monogram.txt --ro src/resource/monogram_probabilities.txt
```
### Oblicz T dla wszystkich ngramow

#### Monogramy
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt  --p src/resource/monogram_probabilities.txt --t1
```
#### Bigramy
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt  --p src/resource/bigram_probabilities.txt --t2
```
#### Trigramy
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt  --p src/resource/trigram_probabilities.txt --t3
```
#### Quadgramy
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt  --p src/resource/quadgram_probabilities.txt --t4
```



