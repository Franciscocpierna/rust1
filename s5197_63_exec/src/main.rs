pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut baldes = [0; 101];
    let (m, n) = (matriz.len(), matriz[0].len());

    // Encadeia iteradores para coordenadas do primeiro elemento de todas as diagonais relevantes
    (1..m-1).map(|i| (i, 0)).chain((0..n-1).map(|j| (0, j))).for_each(|(i0, j0)| {
        let (mut i, mut j) = (i0, j0);
        let (mut min, mut max) = (i32::MAX, i32::MIN);
        // Popula os baldes para ordenação por baldes
        while i < m && j < n {
            let elemento = matriz[i][j];
            min = min.min(elemento);
            max = max.max(elemento);
            baldes[elemento as usize] += 1;
            i += 1;
            j += 1;
        }
        // Popula a matriz com o resultado da ordenação por baldes
        let (mut i, mut j) = (i0, j0);
        let mut k = min as usize;
        while i < m && j < n {
            while baldes[k] == 0 {
                k += 1;
            }
            matriz[i][j] = k as i32;
            baldes[k] -= 1;
            i += 1;
            j += 1;
        }
    });
    matriz
}