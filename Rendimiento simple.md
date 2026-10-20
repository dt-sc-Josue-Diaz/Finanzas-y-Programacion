El rendimiento simple entre dos precios consecutivos.
$$R_t = \frac{P_t}{P_{t-1}} -1$$


```rust
fn main() {
    let precios = vec![100.0, 102.0, 101.0, 105.0];

    let rendimientos: Vec<f64> = precios
        .windows(2)
        .map(|par| par[1] / par[0] - 1.0)
        .collect();

    println!("{:?}", rendimientos);
}

```
