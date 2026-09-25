# Expresiones

## Bitácora - Semana 4

### Paso 0: Creación del proyecto

- Se creó la rama `Semana4` para trabajar en la actividad.
- Se creó el proyecto Rust `tipos_operandos_rust` utilizando `cargo new`.
- Se verificó que el proyecto compila y ejecuta correctamente con `cargo run`.
- Se agregó `**/target` al archivo `.gitignore` para evitar subir los archivos generados por Cargo.

### Paso 1: Prueba unitaria de calcular_oleadas

- Se implementó la función `calcular_oleadas(derrotados, por_oleada)`.
- Se agregó la prueba unitaria `prueba_calcular_oleadas`.
- Se verificó con `cargo test` que la prueba pasa correctamente: 1 pasada y 0 fallidas.
- Se actualizó `main` para mostrar el resultado de `calcular_oleadas(7, 2)`.
