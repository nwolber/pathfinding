v4.3.0 / 2023-05-30
==================

  * Allow creating a Matrix based on a function from position to value
  * Make method cancel_flow of edmondskarp only cancel the minimum amount of flow among all edges along a path, instead of the maximum, in order to avoid negative flows
  * Use sort_unstable_by() instead of sort_unstable_by_key()
  * New Grid example for from_coordinates() method
  * Use RemSP and path splitting
  * Remove optimization which gives worst benchmark results
  * Integrate CodSpeed
  * Update criterion requirement from 0.4.0 to 0.5.1
  * Make Kuhn-Munkres benchmarks reproducible

v4.2.1 / 2023-01-17
==================

  * Document that A*/Dijkstra/Fringe/idA* costs must be non-negative
  * Upgrade dependencies
  * Use new clippy lint name
  * Add bench for separate_components
  * Bench Kuhn-Munkres algorithm
  * Remove itertools dependency
  * Remove unnecessary .into_iter() in tests

v4.2.0 / 2022-12-25
==================

  * Add Grid::from_coordinates()
  * Add the possibility to display the grid with reversed line order
  * Add more Grid documentation

v4.1.1 / 2022-12-14
==================

  * Better performances in Grid, Kruskal and Edmonds-Karp

v4.1.0 / 2022-12-14
==================

  * Add Matrix::items() and Matrix::items_mut()
  * Rename Matrix::indices() as Matrix::keys() and deprecate Matrix::indices()
  * Clarify the ordering of coordinate tuples in Matrix
  * Add more Grid documentation
  * Enable clippy pedantic mode by default

v4.0.1 / 2022-12-12
==================

  * Improve bfs performance
  * Add documentation for possible errors and panics

v4.0.0 / 2022-11-30
==================

  * Add move_in_direction and in_direction to utils
  * Make some function const
  * Cleanups
  * Count paths
  * Add minimum_cut capability to EdmondsKarp
  * Bump MSRV to 1.65.0
  * Update dependencies

v3.0.14 / 2022-10-03
==================

  * Use into_keys() where appropriate
  * Add fake regex dev dependency
  * Use boolean::then_some()
  * Update criterion requirement from 0.3.4 to 0.4.0
  * Optimize Yen's algorithm
  * Routes are already sorted by cost and path len

v3.0.13 / 2022-06-16
==================

  * Document possibility of looping endlessly in kuhn_munkres related functions
  * Use matches!() to simplify expression

v3.0.12 / 2022-04-13
==================

  * Add two algorithms (Floyd and Brent) to detect cycles
  * Deprecate absdiff() in favor of Rust 1.60 abs_diff()
  * Remove double must-use

v3.0.11 / 2022-03-11
==================

  * Introduce `Grid::{bfs,dfs}_reachable()` and `deprecate Grid::reachable()`
  * Remove `Copy` bound on predicate of `Matrix::{bfs,dfs}_reachable()`
  * Use anonymous lifetimes when appropriate
  * Add example for `kuhn_munkres()`

v3.0.10 / 2022-02-14
====================

  * Remove unused `Matrix::uninit`/`Matrix::assume_init()`
  * Remove remaining `debug_assert!()` calls

v3.0.9 / 2022-02-02
===================

  * Add conversion from `Matrix<bool>` to `Grid`
  * Add `Grid` equality
  * Add `Matrix::map()`

v3.0.8 / 2022-01-24
===================

  * Add `Matrix::new_uninit()` and `Matrix::assume_init()`
  * Forbid all missing or partially missing docs
  * Mark iterators as fused

v3.0.7 / 2022-01-23
===================

  * Deprecate `Matrix::reachable()` for `Matrix::bfs_reachable(`) and
    `Matrix::dfs_reachable()`
  * Add `dfs_reach()`
  * Use an enumeration to represent `MatrixFormatError`

v3.0.6 / 2022-01-12
===================

  * Add MSRV and check for consistency
  * Add `#[must_use]` on `Weights` trait
  * Use thiserror crate to build `MatrixFormatError`
  * Add an example for `Grid` as `Debug`

v3.0.5 / 2021-12-13
===================

  * Alternate `Grid` debug mode

v3.0.4 / 2021-12-12
===================

  * Add `Grid::reachable()`
  * Add `Matrix::get()` and `Matrix::get_mut()`

v3.0.3 / 2021-12-09
===================

  * Add `Matrix::reachable()`
  * Better `Matrix` corner cases documentation

v3.0.2 / 2021-12-09
===================

  * Remove references in `Grid` methods
  * Remove more references in `Matrix` methods

v3.0.1 / 2021-12-09
===================

  * Remove unnecessary `Clone` bounds

v3.0.0 / 2021-12-09
===================

  * Use tuples instead of tuples reference for `Matrix` index
