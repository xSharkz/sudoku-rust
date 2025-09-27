// Importamos las librerias de hilos y tiempo
use std::thread;
use std::time::Instant;

// Creamos la estructura  del sudoku de 9x9
type Sudoku = [[u8; 9]; 9];

// Funcion para imprimir un Sudoku
fn imprimir_tablero(tablero: &Sudoku) {
    for f in 0..9 {
        for c in 0..9 {
            print!("{} ", tablero[f][c]);
        }
        println!();
    }
}

// Funcion para encontrar una casilla vacia
fn encontrar_vacio(tablero: &Sudoku) -> Option<(usize, usize)> {
    // Recorremos filas y columnas
    for f in 0..9 {
        for c in 0..9 {
            // Si es que se encuentra un valor 0 en la matriz, significa
            // que esta vacio y lo devuelve
            if tablero[f][c] == 0 {
                return Some((f, c));
            }
        }
    }
    None
}

// Funcion para encontrar un valor posible
fn posible_valor(tablero: &Sudoku, fil: usize, col: usize, val: u8) -> bool {
    // Recorremos las filas y columnas donde esta ubicado el valor
    for i in 0..9 {
        // Si es que se encuentra el mismo numero en su fila o columna
        if tablero[fil][i] == val || tablero[i][col] == val {
            return false;
        }
    }
    // Recorremos el recuadro donde esta ubicado el valor, al ser una
    // matriz de 9x9, los recuadros son de 3x3
    let bf = (fil / 3) * 3;
    let bc = (col / 3) * 3;
    for f in bf..bf+3 {
        for c in bc..bc+3 {
            // Si es que se encuentra el mismo numero en su sub division
            if tablero[f][c] == val {
                return false;
            }
        }
    }
    true
}

// Funcion recursiva para elegir el valor de 1 a 9 a colocar en un vacio
fn recursivo(mut tablero: Sudoku) -> Option<Sudoku> { // El option se usa cuando se puede devolver vacio o el tablero
    // Si se encuentra un valor vacio
    if let Some((f, c)) = encontrar_vacio(&tablero) {
        // Se prueban los valores posibles del 1 al 9
        for val in 1..=9 {
            // Si el valor es posible
            if posible_valor(&tablero, f, c, val) {
                // Colocamos el valor y lo pasamos recursivamente
                tablero[f][c] = val;
                if let Some(sol) = recursivo(tablero) {
                    return Some(sol);
                }
                // Si no funciona el valor colocado provisionalmente, devuelve el valor a 0
                tablero[f][c] = 0;
            }
        }
        None
    } else {
        Some(tablero)
    }
}

//Funcion paralela para resolver el tablero
fn paralelo(tablero: Sudoku, k: usize) -> Option<Sudoku> {
    // Si es que hay algun vacio
    if let Some((r, c)) = encontrar_vacio(&tablero) {
        // Guardamos las posibles opciones como una copia de tablero
        let mut opciones = Vec::new();
        // Si es que el valor de 1 a 9 es posible, guardamos en opciones
        for val in 1..=9 {
            if posible_valor(&tablero, r, c, val) {
                let mut t = tablero;
                t[r][c] = val;
                opciones.push(t);
            }
        }

        // Tomamos las primeras k opciones
        let candidatos: Vec<Sudoku> = opciones.into_iter().take(k).collect();

        // Asignamos un hilo a cada candidato
        let mut handles = Vec::new();

        for candidato in candidatos {
            handles.push(thread::spawn(move || {
                recursivo(candidato)
            }));
        }

        // Tomamos los resultados
        for h in handles {
            // Devolvemos la primera solucion encontrada
            if let Ok(Some(solution)) = h.join() {
                return Some(solution);
            }
        }
        None
    }
    // En caso de que ya este resuelto
    else {
        Some(tablero)
    }
}

fn main() {
    // Creamos el sudoku solicitado
    let sudoku: Sudoku = [
        [8, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 3, 6, 0, 0, 0, 0, 0],
        [0, 7, 0, 0, 0, 0, 2, 0, 0],
        [0, 5, 0, 0, 0, 7, 0, 0, 0],
        [0, 0, 0, 0, 4, 5, 7, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 3, 0],
        [0, 0, 1, 0, 0, 0, 0, 6, 8],
        [0, 0, 8, 5, 0, 0, 0, 1, 0],
        [0, 9, 0, 0, 0, 0, 4, 0, 0],
    ];
    // Hilos a utilizar
    let k = 9;
    println!("Resolviendo con {} hilos...", k);

    //Tomamos el tiempo
    let inicio = Instant::now();

    match paralelo(sudoku, k) {
        // Si se encuentra una solucion, imprimimos tablero y tiempo usado
        Some(solucion) => {
            let duracion = inicio.elapsed(); // ⏱ fin
            println!("Solución encontrada en {:?}", duracion);
            imprimir_tablero(&solucion);
        }
        // Si no se encuentra solucion, devolvemos tiempo una respuesta y tiempo usado
        None => {
            let duracion = inicio.elapsed();
            println!("No se encontró solución. Tiempo: {:?}", duracion);
        }
    }
}
