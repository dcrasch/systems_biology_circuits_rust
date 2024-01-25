# Rust version of Biological Circuit Design




## Install jupyter rust notebook

```shell
apt install jupyter
cargo install evcxr_jupyter
evcxr_jupyter --install
```

```jupyter notebook```

## plotters

## include graph in notebook

```notebook
:dep plotters = { version = "^0.3.5", default_features = false, features = ["evcxr", "all_series", "all_elements"] }
```
```rust
extern create plotters;
use plotters::prelude::*;
let figure = evcxr_figure((640, 480), |root| {
	...	
Ok(())
});
figure
```

### Issues:

* LineSeries lines/points are clamp on graphs not clipped.
https://github.com/plotters-rs/plotters/issues/120


## Links

* https://srenevey.github.io/ode-solvers/
* https://github.com/Armavica/rebop/
* https://github.com/wiseaidev/rust-data-analysis/tree/main
* https://docs.rs/petgraph/latest/petgraph/

## Books

* https://www.weizmann.ac.il/mcb/UriAlon/introduction-systems-biology-design-principles-biological-circuits
* https://biocircuits.github.io/
* https://doc.rust-lang.org/book/
* https://datacrayon.com/shop/product/data-analysis-with-rust-notebooks/

## Libraries

* https://github.com/plotters-rs/plotters and https://crates.io/crates/plotters
* https://github.com/srenevey/ode-solvers and https://crates.io/crates/ode_solvers
