// Mide el A* de JS contra el de Rust compilado a WASM, con el mismo mapa y las mismas consultas.
// Uso: node js/medir.mjs   (para WASM, antes: wasm-pack build --target nodejs --out-dir pkg-node)
import { createRequire } from 'node:module';
import { MapaJS, consultas, generarCeldas } from './camino.js';

const ANCHO = 256, ALTO = 256, CONSULTAS = 300;
const celdas = generarCeldas(ANCHO, ALTO, 7, 28);
const pares = consultas(celdas, CONSULTAS, 99);

function medir(nombre, mapa) {
  for (const [a, b] of pares.slice(0, 20)) mapa.buscar(a, b); // calentamiento del JIT
  const inicio = performance.now();
  let suma = 0;
  for (const [a, b] of pares) suma += mapa.buscar(a, b).length;
  const ms = performance.now() - inicio;
  console.log(`${nombre.padEnd(5)} ${ms.toFixed(1).padStart(7)} ms  suma de largos: ${suma}`);
  return { ms, suma };
}

console.log(`mapa ${ANCHO}x${ALTO}, ${CONSULTAS} consultas`);
const js = medir('JS', new MapaJS(ANCHO, ALTO, celdas));

let wasm;
try {
  wasm = createRequire(import.meta.url)('../pkg-node/camino.js');
} catch {
  console.log('WASM  (sin compilar: corre wasm-pack build --target nodejs --out-dir pkg-node)');
  process.exit(0);
}
const mapa = new wasm.Mapa(ANCHO, ALTO, celdas);
const rs = medir('WASM', mapa);
mapa.free();
console.log(rs.suma === js.suma ? '✓ mismos largos de camino' : '✗ los largos no coinciden');
console.log(`WASM tardó ${(rs.ms / js.ms * 100).toFixed(0)} % de lo que tardó JS`);
process.exit(rs.suma === js.suma ? 0 : 1);
