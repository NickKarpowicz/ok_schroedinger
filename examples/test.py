import marimo

__generated_with = "0.21.1"
app = marimo.App(width="medium")


@app.cell
def _():
    import ok_schroedinger as oks
    import numpy as np

    return np, oks


@app.cell
def _(np, oks):
    a = oks.Wavefunction(
        dr = 1.0,
        r_max = 100.0,
        l_max = 10,
        is_3d = False,
        potential = np.array([1.0, 2.0, 3.0]),
        psi = np.zeros(4, dtype=np.complex128)
    )
    return (a,)


@app.cell
def _(a, np, oks):
    b = oks.SimParams(
        dt = 0.5,
        steps = 10,
        field_z = np.zeros(100, dtype=float),
        field_x = np.zeros(100, dtype=float),
        wavefunction = a)
    return (b,)


@app.cell
def _(b, oks):
    oks.check_input(b)
    return


@app.cell
def _():
    return


if __name__ == "__main__":
    app.run()
