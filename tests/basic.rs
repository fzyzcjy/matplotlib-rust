use anyhow::Result;

use matplotlib::*;

#[test]
fn basic() {
    PyPlot::with_gil(|plt| -> Result<()> {
        plt.plot(([10, 20], [30, 40]), None)?;
        plt.show()?;
        Ok(())
    })
    .unwrap();
}
