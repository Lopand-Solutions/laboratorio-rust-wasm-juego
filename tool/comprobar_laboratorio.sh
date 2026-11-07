#!/usr/bin/env bash
# Comprueba el laboratorio «Pasa a WASM la parte pesada de un juego web». Uso: ./tool/comprobar_laboratorio.sh
# Corre tus pruebas, clippy y formato; rompe tu código en una copia para ver que tus pruebas lo detectan; y revisa
# que Rust y JavaScript encuentren caminos del mismo largo. No toca tus archivos. Funciona con el bash 3.2 de macOS.
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1
RAIZ=$(pwd)
ARCHIVO=src/lib.rs
bien=0
mal=0
ok() { printf '  ✓ %s\n' "$1"; bien=$((bien + 1)); }
no() { printf '  ✗ %s\n' "$1"; mal=$((mal + 1)); }
nota() { printf '  · %s\n' "$1"; }

command -v cargo >/dev/null || { echo "No encuentro cargo. Instala Rust con rustup: https://rustup.rs"; exit 1; }

echo "1. Tus pruebas"
pruebas_ok=0
if salida=$(cargo test --quiet 2>&1); then
  pruebas_ok=1
  ok "cargo test en verde ($(printf '%s\n' "$salida" | grep -m1 'test result' | sed 's/test result: //'))"
else
  no "cargo test en rojo: arréglalo antes de seguir"
  printf '%s\n' "$salida" | grep -E "panicked at|not yet implemented|FAILED|error\[" | head -8 | sed 's/^/      /'
fi

echo "2. Clippy y formato"
if cargo clippy --quiet --all-targets -- -D warnings >/dev/null 2>&1; then
  ok "cargo clippy sin advertencias"
else
  no "cargo clippy encontró advertencias: corre cargo clippy --all-targets para verlas"
fi
if cargo fmt --check >/dev/null 2>&1; then ok "cargo fmt --check: el formato está bien"; else no "falta formato: corre cargo fmt"; fi

echo "3. Tus pruebas detectan los errores de los retos (en una copia)"
[ "$pruebas_ok" -eq 1 ] || no "primero pon tus pruebas en verde: romper código que ya falla no dice nada"
romper() {
  local original=$1 roto=$2 que=$3 tmp codigo
  [ "$pruebas_ok" -eq 1 ] || return
  codigo=$(cat "$ARCHIVO")
  case "$codigo" in
    *"$original"*) ;;
    *) no "no encuentro en $ARCHIVO «$original»: ¿la cambiaste? ($que)"; return ;;
  esac
  tmp=$(mktemp -d)
  cp -R Cargo.toml Cargo.lock src "$tmp/"
  printf '%s\n' "${codigo/"$original"/"$roto"}" > "$tmp/$ARCHIVO"
  if (cd "$tmp" && CARGO_TARGET_DIR="$RAIZ/target/comprobador" cargo test --quiet --lib >/dev/null 2>&1); then
    no "NO detectan $que: falta una prueba"
  else
    ok "detectan $que"
  fi
  rm -rf "$tmp"
}
romper '        ax.abs_diff(bx) + ay.abs_diff(by)' '        2 * (ax.abs_diff(bx) + ay.abs_diff(by))' 'una heurística que sobrestima (reto 1)'
romper '        .filter(|&v| self.celdas[v as usize] != PARED)' '        .filter(|_| true)' 'un camino que cruza paredes (reto 2)'
romper '            (x + 1 < self.ancho).then(|| i + 1),' '            (i + 1 < self.ancho * self.alto).then(|| i + 1),' 'un salto del borde derecho a la fila siguiente (reto 3)'

echo "4. Rust y JavaScript encuentran caminos del mismo largo"
if ! command -v node >/dev/null; then
  nota "no encuentro node: instala Node.js para comparar contra la versión en JavaScript"
elif [ "$pruebas_ok" -eq 0 ]; then
  nota "primero pon tus pruebas en verde"
else
  suma_js=$(node js/medir.mjs 2>/dev/null | grep '^JS' | sed 's/.*suma de largos: //')
  suma_rs=$(cargo run --quiet --release --example medir 2>/dev/null | sed 's/.*suma de largos: //')
  if [ -n "$suma_js" ] && [ "$suma_js" = "$suma_rs" ]; then
    ok "300 consultas en un mapa de 256x256: suma de largos $suma_rs en los dos"
  else
    no "los largos no coinciden (JS: ${suma_js:-sin dato}, Rust: ${suma_rs:-sin dato})"
  fi
fi

echo "5. Tu paquete de WebAssembly"
if [ -f pkg-node/camino.js ] && command -v node >/dev/null; then
  if salida=$(node js/medir.mjs 2>&1); then
    ok "WASM y JS dan los mismos largos"
    printf '%s\n' "$salida" | grep -E '^(JS|WASM|WASM tardó)' | sed 's/^/      /'
  else
    no "WASM no coincide con JS:"
    printf '%s\n' "$salida" | sed 's/^/      /'
  fi
elif command -v rustup >/dev/null && rustup target list --installed 2>/dev/null | grep -q '^wasm32-unknown-unknown$'; then
  no "falta compilar a WASM: wasm-pack build --target nodejs --out-dir pkg-node (ver el README)"
else
  nota "sin el target wasm32-unknown-unknown no se puede revisar: rustup target add wasm32-unknown-unknown"
fi

echo
if [ "$mal" -eq 0 ]; then echo "Laboratorio completo: $bien de $bien."; exit 0; fi
echo "Falta$([ "$mal" -eq 1 ] || echo n) $mal punto$([ "$mal" -eq 1 ] || echo s) ($bien bien)."
exit 1
