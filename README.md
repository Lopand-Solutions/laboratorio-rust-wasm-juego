# Laboratorio: pasa a WASM la parte pesada de un juego web

Repositorio del laboratorio de [Lopand Academy](https://academy.lopandsolutions.com/tutoriales/laboratorio-rust-wasm-juego).
Un juego web donde cientos de enemigos persiguen al jugador con A*. La búsqueda de caminos está en JavaScript
(`js/camino.js`) y la pasas a Rust compilado a WebAssembly (`src/lib.rs`), con una medición de las dos versiones sobre
el mismo mapa. Con pruebas, un comprobador y el reto «rómpelo y arréglalo».

## Ramas

- **`inicio`**: el juego en JavaScript, la medición y el crate de Rust con las pruebas; `heuristica`, `buscar` y
  `reconstruir` son `todo!()`.
- **`main`**: la solución, tal como está en la lección.

```bash
git clone -b inicio https://github.com/Lopand-Solutions/laboratorio-rust-wasm-juego.git
cd laboratorio-rust-wasm-juego
```

## Qué hay

| Ruta | Qué es |
| --- | --- |
| `src/lib.rs` | El A* en Rust, la capa `#[wasm_bindgen]` y sus pruebas. Es lo que escribes |
| `js/camino.js` | El A* original en JavaScript, con el mismo generador de mapas |
| `js/medir.mjs` | Mide JS contra WASM en Node con el mismo mapa y las mismas consultas |
| `examples/medir.rs` | Las mismas consultas en Rust nativo |
| `web/index.html` | El juego: elige JS o WASM y mira cuánto tarda cada cuadro |
| `tool/comprobar_laboratorio.sh` | Revisa todo lo anterior |

## Córrelo

Primero la lógica, nativa:

```bash
cargo test
cargo run --release --example medir
node js/medir.mjs
```

Después, a WebAssembly (una vez: `rustup target add wasm32-unknown-unknown` e instala
[wasm-pack](https://wasm-bindgen.github.io/wasm-pack/installer)):

```bash
wasm-pack build --target nodejs --out-dir pkg-node
node js/medir.mjs
wasm-pack build --target web
python3 -m http.server 8080
```

Abre `http://localhost:8080/web/` y cambia entre JavaScript y Rust (WASM) con cientos de enemigos.

## Cómo saber si quedó

```bash
./tool/comprobar_laboratorio.sh
```

Corre `cargo test`, `cargo clippy -- -D warnings` y `cargo fmt --check`; rompe tu código en una copia (los tres retos
de la lección) para ver que tus pruebas lo detectan; compara la suma de los largos de 300 caminos entre Rust y
JavaScript; y, si ya compilaste `pkg-node/`, compara WASM contra JS.

## Requisitos

Rust 1.85 o superior (edición 2024), Node.js 18 o superior y, para WASM, el target `wasm32-unknown-unknown` y
`wasm-pack`. Las versiones con las que se probó la lección están en `Cargo.lock`.

Código libre para usar y adaptar (licencia MIT).
