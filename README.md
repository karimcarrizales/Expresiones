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

### Paso 2: Prueba unitaria de calcular_dano_critico

- Se implementó la función `calcular_dano_critico(dano_base, multiplicador)`.
- Se agregó la prueba unitaria `prueba_calcular_dano_critico`.
- Se verificó con `cargo test` que las 2 pruebas pasan correctamente: 2 pasadas y 0 fallidas.
- Se actualizó `main` para mostrar el resultado de `calcular_dano_critico(100, 2.0)`.


### Paso 3: Prueba unitaria de calcular_dano_promedio

- Se implementó la función `calcular_dano_promedio(dano_base, por_oleada)`.
- Se agregó la tercera prueba unitaria `prueba_calcular_dano_promedio`.
- Se verificó con `cargo test` que las 3 pruebas pasan correctamente: 3 pasadas y 0 fallidas.
- Se actualizó `main` para mostrar el resultado de las tres funciones.


### Paso 4: Verificación final

Se realizó la verificación final con `cargo run` y `cargo test`.

Salida final de `cargo test`:

running 3 tests
test tests::prueba_calcular_dano_critico ... ok
test tests::prueba_calcular_dano_promedio ... ok
test tests::prueba_calcular_oleadas ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
