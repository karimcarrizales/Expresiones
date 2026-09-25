fn calcular_oleadas(derrotados: i32, por_oleada: i32) -> i32 {
    derrotados / por_oleada
}

fn calcular_dano_critico(dano_base: i32, multiplicador: f64) -> f64 {
    dano_base as f64 * multiplicador
}

fn calcular_dano_promedio(dano_base: i32, por_oleada: i32) -> f64 {
    dano_base as f64 / por_oleada as f64
}

fn main() {
    println!("Oleadas: {}", calcular_oleadas(7, 2));
    println!("Daño crítico: {}", calcular_dano_critico(100, 2.0));
    println!("Daño promedio: {}", calcular_dano_promedio(100, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prueba_calcular_oleadas() {
        assert_eq!(calcular_oleadas(7, 2), 3);
    }

    #[test]
    fn prueba_calcular_dano_critico() {
        assert_eq!(calcular_dano_critico(100, 2.0), 200.0);
    }

    #[test]
    fn prueba_calcular_dano_promedio() {
        assert_eq!(calcular_dano_promedio(100, 2), 50.0);
    }
}
