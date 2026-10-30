# Elevator Simulator

Desktopová aplikace pro simulaci a vizuální porovnání dvou algoritmů řízení výtahu — **základního** a **chytrého (SCAN)** — postavená na [Tauri](https://tauri.app/), [SvelteKit](https://kit.svelte.dev/) a [Rust](https://www.rust-lang.org/).

---

## O projektu

Elevator Simulator je nástroj pro vizuální srovnání efektivity dvou různých strategií řízení výtahu ve stejném scénáři se stejnými cestujícími.

### Jak aplikace funguje

Po načtení konfiguračního souboru (`data/test.json`) se spustí souběžně dvě simulace výtahu na stejném scénáři:

| Algoritmus | Popis |
|---|---|
| **Základní výtah** | Jednoduché FCFS (First Come, First Served) řízení — výtah obslouží požadavky v pořadí, v jakém přišly. |
| **Chytrý výtah** | SCAN algoritmus (také znám jako „výtahový algoritmus") — výtah se pohybuje jedním směrem, obslouží všechny požadavky po cestě, a teprve poté se obrátí. |

Oba výtahy jsou vizualizovány vedle sebe v reálném čase. Aplikace sleduje a zobrazuje **ujetou vzdálenost** každého výtahu, což umožňuje přímé porovnání efektivity obou přístupů.

Cestující jsou definováni v JSON souboru — každý má čas příchodu, nástupní patro a cílové patro. Simulace je deterministická: stejná konfigurace vždy produkuje stejný výsledek.

---

## Požadavky (Prerequisites)

Před spuštěním projektu je nutné mít nainstalované následující nástroje:

### 1. Node.js a pnpm

- Stáhni a nainstaluj [Node.js](https://nodejs.org/) (doporučená LTS verze).
- Nainstaluj správce balíčků **pnpm**:
  ```powershell
  npm install -g pnpm
  ```

### 2. Rust a Cargo

- Stáhni a spusť instalátor [rustup](https://rustup.rs/).
- Po instalaci restartuj terminál a ověř instalaci:
  ```powershell
  rustc --version
  cargo --version
  ```

### 3. WebView2 (Windows)

Na Windows musí být nainstalovaný **Microsoft Edge WebView2 Runtime**. Na většině systémů s Windows 10/11 je již přítomen. Pokud ne, stáhni ho z [webu Microsoftu](https://developer.microsoft.com/en-us/microsoft-edge/webview2/).

### 4. Visual Studio Build Tools

Rust na Windows vyžaduje C++ build tools. **rustup je při instalaci automaticky nabídne nainstalovat** — stačí v průvodci zvolit možnost instalace Visual Studio Build Tools a pokračovat podle pokynů.

Pokud je potřeba nainstalovat je ručně, stáhni [**Visual Studio Build Tools**](https://visualstudio.microsoft.com/cs/visual-cpp-build-tools/) a při instalaci zaškrtni:
- *Desktop development with C++*

Nebo nainstaluj celé [Visual Studio Community](https://visualstudio.microsoft.com/cs/vs/community/).

---

## Konfigurace simulace

Simulace načítá scénář ze souboru **`data/test.json`**. Tento soubor je nutné před spuštěním vytvořit nebo upravit.

### Formát souboru `data/test.json`

```json
{
  "format": "vytah-zadani",
  "version": 1,
  "lift": {
    "floors": 8,
    "capacity": 6,
    "startFloor": 0
  },
  "people": [
    { "t": 0.0, "from": 0, "to": 3 },
    { "t": 5.5, "from": 2, "to": 7 },
    { "t": 12.0, "from": 5, "to": 1 }
  ]
}
```

| Pole | Popis |
|---|---|
| `lift.floors` | Celkový počet pater |
| `lift.capacity` | Maximální kapacita výtahu (počet osob) |
| `lift.startFloor` | Počáteční patro výtahu |
| `people[].t` | Čas příchodu osoby (v sekundách) |
| `people[].from` | Patro, kde osoba nastupuje |
| `people[].to` | Patro, kam osoba jede |

---

## Spuštění projektu

### 1. Nainstaluj závislosti

```powershell
pnpm install
```

### 2. Spusť vývojový režim

```powershell
pnpm tauri dev
```

Příkaz zkompiluje backend (Rust) a spustí frontendový dev server. Při první kompilaci Rustu může spuštění trvat déle (několik minut).

---

## Úprava rychlosti simulace

Rychlost simulace lze změnit úpravou konstanty `SPEED` v souboru [`src-tauri/src/lib.rs`](src-tauri/src/lib.rs):

```rust
// src-tauri/src/lib.rs, řádek 7
const SPEED: u32 = 50;
```

Hodnota `SPEED` udává počet **milisekund** simulačního času, které uplynou za jeden krok (tick) simulace — zároveň určuje interval obnovy v ms. **Nižší hodnota = rychlejší simulace**, vyšší hodnota = pomalejší.

> [!NOTE]
> Po změně `SPEED` je nutné projekt znovu sestavit (`pnpm tauri dev`), aby se změna projevila.

---

## Autoři

| Jméno | Role |
|---|---|
| **Ladislav Jaromír Pleštil** | Frontend (SvelteKit, vizualizace) |
| **Jakub Kopas** | Backend (Rust, simulační logika) |

---

## Technologie

- [Tauri 2](https://tauri.app/) — desktopový framework
- [SvelteKit](https://kit.svelte.dev/) — frontend framework
- [Rust](https://www.rust-lang.org/) — backend a simulační engine
- [Vite](https://vite.dev/) — build nástroj
