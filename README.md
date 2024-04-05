# Lancaster Simulator

## Simulate battle outcomes with Lancasim

### Requirements

- [x] Read config file with adjustable attrition coefficient window
- [x] Create results directory structure
- [x] Perform lancaster simulation with variables
- [x] Store results in matrix
- [x] Concurrently store results in `.csv` file

### Usage

Lancasim is able to run many simulations of a battle, using the [Lanchester's square law](https://en.wikipedia.org/wiki/Lanchester%27s_laws). This is done based on a configuration file like this:

```ini
[Blue]
; integer: Start with this amount of units
units       = 100
; f32 between 0 and 1: Minimum attrition coefficient for this team
min_ac      = 0.1
; f32 between 0 and 1: Maximum attrition coefficient for this team
max_ac      = 0.2
; f32 between 0 and 1: Increment for each simulation
increment   = 0.01

[Red]
; integer: Start with this amount of units
units       = 100
; f32 between 0 and 1: Minimum attrition coefficient for this team
min_ac      = 0.01
; f32 between 0 and 1: Maximum attrition coefficient for this team
max_ac      = 0.05
; f32 between 0 and 1: Increment for each simulation
increment   = 0.001
```

The simulation environment will run a simulation for all points on a `blue` by `red` matrix, where the attrition coefficients for each team increases incrementally. It will then concurrently write the results to a file in the `results` directory.