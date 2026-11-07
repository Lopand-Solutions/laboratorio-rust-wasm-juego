//! Las mismas consultas que js/medir.mjs, en Rust nativo: cargo run --release --example medir
use std::time::Instant;

use camino::{Azar, Mapa, PARED, generar_celdas};

fn main() {
    let (ancho, alto) = (256, 256);
    let celdas = generar_celdas(ancho, alto, 7, 28.0);
    let mut azar = Azar(99);
    let mut pares = Vec::new();
    while pares.len() < 300 {
        let a = (azar.siguiente() * celdas.len() as f64) as u32;
        let b = (azar.siguiente() * celdas.len() as f64) as u32;
        if celdas[a as usize] != PARED && celdas[b as usize] != PARED {
            pares.push((a, b));
        }
    }
    let mapa = Mapa::crear(ancho, alto, celdas).unwrap();
    let inicio = Instant::now();
    let suma: usize = pares.iter().map(|&(a, b)| mapa.buscar(a, b).len()).sum();
    println!(
        "Rust nativo {:.1} ms  suma de largos: {suma}",
        inicio.elapsed().as_secs_f64() * 1000.0
    );
}
