//! A* en una cuadrícula para un juego web: la parte pesada que pasamos de JavaScript a Rust.
// Rama inicio: borra esta línea cuando termines; solo calla los avisos de lo que todavía no escribes.
#![allow(dead_code, unused_imports, unused_variables)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use wasm_bindgen::prelude::*;

pub const PARED: u8 = 1;
const SIN_PREVIO: u32 = u32::MAX;

/// El mapa vive del lado de Rust: JS lo manda una vez y después solo pide caminos.
#[wasm_bindgen]
pub struct Mapa {
    ancho: u32,
    alto: u32,
    celdas: Vec<u8>,
}

impl Mapa {
    /// Para Rust y las pruebas nativas: el error es un `String`, no toca JS.
    pub fn crear(ancho: u32, alto: u32, celdas: Vec<u8>) -> Result<Mapa, String> {
        if ancho == 0 || alto == 0 || celdas.len() != (ancho * alto) as usize {
            return Err(format!(
                "se esperaban {} celdas ({ancho}x{alto}) y llegaron {}",
                ancho * alto,
                celdas.len()
            ));
        }
        Ok(Mapa {
            ancho,
            alto,
            celdas,
        })
    }

    /// Distancia de Manhattan: nunca sobrestima con movimientos en 4 direcciones de costo 1.
    fn heuristica(&self, a: u32, b: u32) -> u32 {
        todo!("Paso 3: distancia de Manhattan entre a y b")
    }

    fn vecinas(&self, i: u32) -> impl Iterator<Item = u32> + '_ {
        let (x, y) = (i % self.ancho, i / self.ancho);
        [
            (x > 0).then(|| i - 1),
            (x + 1 < self.ancho).then(|| i + 1),
            (y > 0).then(|| i - self.ancho),
            (y + 1 < self.alto).then(|| i + self.ancho),
        ]
        .into_iter()
        .flatten()
        .filter(|&v| self.celdas[v as usize] != PARED)
    }
}

#[wasm_bindgen]
impl Mapa {
    /// `new Mapa(ancho, alto, celdas)` en JS. `celdas` es un `Uint8Array`: 1 = pared, 0 = suelo.
    #[wasm_bindgen(constructor)]
    pub fn new(ancho: u32, alto: u32, celdas: Vec<u8>) -> Result<Mapa, JsError> {
        Mapa::crear(ancho, alto, celdas).map_err(|e| JsError::new(&e))
    }

    /// El camino más corto de `desde` a `hasta` (índices y * ancho + x), los dos incluidos.
    /// Vacío si no hay camino. En JS llega como `Uint32Array`.
    pub fn buscar(&self, desde: u32, hasta: u32) -> Vec<u32> {
        todo!("Paso 4: A* con un BinaryHeap")
    }

    #[wasm_bindgen(getter)]
    pub fn ancho(&self) -> u32 {
        self.ancho
    }

    #[wasm_bindgen(getter)]
    pub fn alto(&self) -> u32 {
        self.alto
    }
}

impl Mapa {
    fn reconstruir(&self, previo: &[u32], hasta: u32) -> Vec<u32> {
        todo!("Paso 4: recorre `previo` desde la meta hasta el inicio")
    }
}

/// Mulberry32: el mismo generador que `js/camino.js`, para crear los mismos mapas en los dos lados.
pub struct Azar(pub u32);

impl Azar {
    pub fn siguiente(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x6D2B_79F5);
        let a = self.0;
        let mut t = (a ^ (a >> 15)).wrapping_mul(1 | a);
        t = t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t)) ^ t;
        (t ^ (t >> 14)) as f64 / 4_294_967_296.0
    }
}

/// Un mapa con `pared_pct` % de paredes al azar.
pub fn generar_celdas(ancho: u32, alto: u32, semilla: u32, pared_pct: f64) -> Vec<u8> {
    let mut azar = Azar(semilla);
    (0..ancho * alto)
        .map(|_| u8::from(azar.siguiente() * 100.0 < pared_pct))
        .collect()
}

#[cfg(test)]
mod pruebas {
    use std::collections::VecDeque;

    use super::*;

    /// La referencia: búsqueda en anchura. Lenta pero obviamente correcta.
    fn distancia_bfs(m: &Mapa, desde: u32, hasta: u32) -> Option<usize> {
        let mut dist = vec![usize::MAX; m.celdas.len()];
        let mut cola = VecDeque::from([desde]);
        dist[desde as usize] = 0;
        while let Some(i) = cola.pop_front() {
            if i == hasta {
                return Some(dist[i as usize]);
            }
            for v in m.vecinas(i) {
                if dist[v as usize] == usize::MAX {
                    dist[v as usize] = dist[i as usize] + 1;
                    cola.push_back(v);
                }
            }
        }
        None
    }

    #[test]
    fn camino_en_un_pasillo() {
        // 5x3, una pared en medio con un hueco abajo.
        #[rustfmt::skip]
        let celdas = vec![
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 0, 0, 0,
        ];
        let m = Mapa::crear(5, 3, celdas).unwrap();
        // Hay varios caminos de 9 casillas; cualquiera sirve, así que revisamos el largo y las puntas.
        let c = m.buscar(0, 4);
        assert_eq!(c.len(), 9);
        assert_eq!((c[0], c[8]), (0, 4));
        assert!(c.contains(&12), "tiene que pasar por el hueco de abajo");
        assert_eq!(m.buscar(0, 0), vec![0]);
        assert_eq!(m.buscar(0, 2), vec![]); // la meta es pared
    }

    #[test]
    fn sin_salida_devuelve_vacio() {
        let m = Mapa::crear(3, 1, vec![0, 1, 0]).unwrap();
        assert!(m.buscar(0, 2).is_empty());
    }

    #[test]
    fn el_camino_es_valido_y_tan_corto_como_bfs() {
        for semilla in 0..30 {
            let m = Mapa::crear(40, 30, generar_celdas(40, 30, semilla, 28.0)).unwrap();
            let mut azar = Azar(semilla + 1000);
            for _ in 0..20 {
                let desde = (azar.siguiente() * 1200.0) as u32;
                let hasta = (azar.siguiente() * 1200.0) as u32;
                if m.celdas[desde as usize] == PARED || m.celdas[hasta as usize] == PARED {
                    continue;
                }
                let camino = m.buscar(desde, hasta);
                match distancia_bfs(&m, desde, hasta) {
                    None => assert!(camino.is_empty()),
                    Some(d) => {
                        assert_eq!(camino.len(), d + 1, "semilla {semilla}: {desde} -> {hasta}");
                        assert_eq!((camino[0], *camino.last().unwrap()), (desde, hasta));
                        for par in camino.windows(2) {
                            // Vecinas de verdad: a una casilla, calculado aquí y no con heuristica().
                            let (a, b) = (par[0], par[1]);
                            let paso = (a % 40).abs_diff(b % 40) + (a / 40).abs_diff(b / 40);
                            assert_eq!(paso, 1, "salto en {par:?}");
                            assert_ne!(m.celdas[par[1] as usize], PARED, "cruza una pared");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn rechaza_un_tamano_que_no_cuadra() {
        assert!(Mapa::crear(4, 4, vec![0; 15]).is_err());
    }

    #[test]
    fn el_generador_coincide_con_el_de_js() {
        // Los primeros valores de mulberry32(42) en JavaScript (node, 7 nov 2026).
        let mut a = Azar(42);
        let v: Vec<u32> = (0..3).map(|_| (a.siguiente() * 1e6) as u32).collect();
        assert_eq!(v, vec![601103, 448290, 852465]);
    }
}
