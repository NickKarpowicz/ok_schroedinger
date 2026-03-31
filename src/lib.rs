use pyo3::prelude::*;
use num_complex::Complex;

/// A Python module implemented in Rust.
#[pymodule]
fn ok_schroedinger<'py>(_py: Python<'py>, m: &Bound<'_, PyModule>) -> PyResult<()> {

    #[pyclass(get_all, set_all, from_py_object)]
    #[derive(Debug, Clone)]
    struct Wavefunction {
        dr: f64,
        r_max: f64,
        l_max: i64,
        is_3d: bool,
        potential: Vec<f64>,
        psi: Vec<Complex<f64>>
    }
    #[pymethods]
    impl Wavefunction {
        #[new]
        fn new(dr: f64, r_max: f64, l_max: i64, is_3d: bool, potential: Vec<f64>, psi: Vec<Complex<f64>>) -> Self {
            Self {
                dr,
                r_max,
                l_max,
                is_3d,
                potential,
                psi
            }
        }
    }

    #[pyclass(get_all, set_all, from_py_object)]
    #[derive(Debug, Clone)]
    struct SimParams {
        dt: f64,
        steps: usize,
        field_z: Vec<f64>,
        field_x: Vec<f64>,
        wavefunction: Wavefunction
    }
    #[pymethods]
    impl SimParams {
        #[new]
        fn new(dt: f64, steps: usize, field_z: Vec<f64>, field_x: Vec<f64>, wavefunction: Wavefunction) -> Self {
            Self {
                dt,
                steps,
                field_z,
                field_x,
                wavefunction
            }
        }
    }

    #[pyfunction]
    fn check_input(inputs: SimParams) -> PyResult<String> {
        Ok(format!("{inputs:?}"))
    }
    m.add_function(wrap_pyfunction!(check_input, m)?)?;
    m.add_class::<Wavefunction>()?;
    m.add_class::<SimParams>()?;
    Ok(())
}
