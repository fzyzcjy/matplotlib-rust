use std::borrow::Cow;

use anyhow::Result;
use numpy::{PyArray, ToPyArray};
use opencv::core::*;
use pyo3::Python;

pub trait MatToPyArray {
    fn to_pyarray<'py>(&self, _: Python<'py>) -> Result<&'py PyArray<Self::Item, Self::Dim>>;
}

impl MatToPyArray for Mat {
    fn to_pyarray<'py>(&self, py: Python<'py>) -> Result<&'py PyArray<Self::Item, Self::Dim>> {
        let (rows, cols) = (self.rows(), self.cols());
        let mat_continuous = mat_to_continuous(self);
        let flatten_data_bytes = mat_continuous.as_ref().data_bytes()?;
        let flatten_pyarray = flatten_data_bytes.to_pyarray(py);
        Ok(flatten_pyarray.reshape((rows, cols))?)
    }
}

fn mat_to_continuous(m: &Mat) -> Cow<Mat> {
    if m.is_continuous() {
        Cow::Borrowed(m)
    } else {
        Cow::Owned(m.clone())
    }
}
