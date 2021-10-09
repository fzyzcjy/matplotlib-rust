use anyhow::Result;
use numpy::{PyArray, ToPyArray};
use opencv::core::{Rect, CV_8UC1};
use opencv::imgproc::{rectangle, LINE_8};
use opencv::prelude::*;

use matplotlib::*;

#[test]
fn basic() {
    PyPlot::with_gil(|plt| -> Result<()> {
        let (nr, nc) = (2, 3);

        // plot + array
        plt.subplot(nr, nc, 1, None)?;
        plt.plot(([10, 20], [30, 40]), None)?;

        // plot + Vec
        plt.subplot(nr, nc, 2, None)?;
        plt.plot((vec![10, 20], vec![30, 40]), None)?;

        // imshow + opencv Mat
        let mut im = Mat::zeros(100, 200, CV_8UC1)?.to_mat()?;
        rectangle(
            &mut im,
            Rect::new(10, 20, 30, 40),
            100.0.into(),
            1,
            LINE_8,
            0,
        )?;
        plt.subplot(nr, nc, 3, None)?;
        plt.imshow((im.to_pyarray(py)?,), None)?;

        plt.show()?;
        Ok(())
    })
    .unwrap();
}
