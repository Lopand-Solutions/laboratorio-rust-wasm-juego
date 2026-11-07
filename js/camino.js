// La versión original en JavaScript: el mismo A* que src/lib.rs, con su propio montículo.
export const PARED = 1;

// Mulberry32: el mismo generador que `Azar` en Rust, para crear los mismos mapas.
export function mulberry32(a) {
  return function () {
    a |= 0;
    a = (a + 0x6d2b79f5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

export function generarCeldas(ancho, alto, semilla, paredPct) {
  const azar = mulberry32(semilla);
  const celdas = new Uint8Array(ancho * alto);
  for (let i = 0; i < celdas.length; i++) celdas[i] = azar() * 100 < paredPct ? 1 : 0;
  return celdas;
}

// Montículo de mínimos sobre arreglos planos: [f, -g, celda] por entrada.
class Monticulo {
  constructor() { this.f = []; this.g = []; this.c = []; }
  get size() { return this.f.length; }
  menor(i, j) {
    return this.f[i] < this.f[j] || (this.f[i] === this.f[j] && this.g[i] > this.g[j]);
  }
  cambiar(i, j) {
    [this.f[i], this.f[j]] = [this.f[j], this.f[i]];
    [this.g[i], this.g[j]] = [this.g[j], this.g[i]];
    [this.c[i], this.c[j]] = [this.c[j], this.c[i]];
  }
  push(f, g, c) {
    this.f.push(f); this.g.push(g); this.c.push(c);
    let i = this.f.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (!this.menor(i, p)) break;
      this.cambiar(i, p);
      i = p;
    }
  }
  pop() {
    const g = this.g[0], c = this.c[0];
    const ultimo = this.f.length - 1;
    this.cambiar(0, ultimo);
    this.f.pop(); this.g.pop(); this.c.pop();
    let i = 0;
    for (;;) {
      const a = 2 * i + 1, b = a + 1;
      let m = i;
      if (a < this.f.length && this.menor(a, m)) m = a;
      if (b < this.f.length && this.menor(b, m)) m = b;
      if (m === i) break;
      this.cambiar(i, m);
      i = m;
    }
    return [g, c];
  }
}

export class MapaJS {
  constructor(ancho, alto, celdas) {
    if (celdas.length !== ancho * alto) throw new Error('el tamaño no cuadra');
    this.ancho = ancho; this.alto = alto; this.celdas = celdas;
  }
  heuristica(a, b) {
    const w = this.ancho;
    return Math.abs((a % w) - (b % w)) + Math.abs(Math.floor(a / w) - Math.floor(b / w));
  }
  buscar(desde, hasta) {
    const n = this.celdas.length, w = this.ancho;
    if (this.celdas[desde] === PARED || this.celdas[hasta] === PARED) return new Uint32Array(0);
    const costo = new Uint32Array(n).fill(0xffffffff);
    const previo = new Int32Array(n).fill(-1);
    const abiertos = new Monticulo();
    costo[desde] = 0;
    abiertos.push(this.heuristica(desde, hasta), 0, desde);
    while (abiertos.size > 0) {
      const [g, actual] = abiertos.pop();
      if (actual === hasta) {
        const camino = [hasta];
        for (let i = hasta; previo[i] !== -1; ) camino.push((i = previo[i]));
        return Uint32Array.from(camino.reverse());
      }
      if (g > costo[actual]) continue;
      const x = actual % w, y = Math.floor(actual / w);
      const vecinas = [x > 0 ? actual - 1 : -1, x + 1 < w ? actual + 1 : -1,
        y > 0 ? actual - w : -1, y + 1 < this.alto ? actual + w : -1];
      for (const v of vecinas) {
        if (v < 0 || this.celdas[v] === PARED) continue;
        const nuevo = g + 1;
        if (nuevo < costo[v]) {
          costo[v] = nuevo;
          previo[v] = actual;
          abiertos.push(nuevo + this.heuristica(v, hasta), nuevo, v);
        }
      }
    }
    return new Uint32Array(0);
  }
}

// Las mismas consultas en Rust y en JS: pares de celdas de suelo elegidos con la semilla.
export function consultas(celdas, cuantas, semilla) {
  const azar = mulberry32(semilla);
  const pares = [];
  while (pares.length < cuantas) {
    const a = Math.floor(azar() * celdas.length), b = Math.floor(azar() * celdas.length);
    if (celdas[a] !== PARED && celdas[b] !== PARED) pares.push([a, b]);
  }
  return pares;
}
