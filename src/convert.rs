use std::borrow::Cow;

use anyhow::Result;
use ndarray::*;
use numpy::{Element, PyArray, ToPyArray};
use opencv::core::*;
use pyo3::Python;

pub trait MatToPyArray {
    fn to_pyarray<'py, T: DataType + Element>(
        &self,
        _: Python<'py>,
    ) -> Result<&'py PyArray<T, Ix2>>;
}

impl MatToPyArray for Mat {
    fn to_pyarray<'py, T: DataType + Element>(
        &self,
        py: Python<'py>,
    ) -> Result<&'py PyArray<T, Ix2>> {
        let (rows, cols) = (self.rows(), self.cols());
        let mat_continuous = mat_to_continuous(self)?;
        let flatten_data_bytes = mat_continuous.as_ref().data_typed::<T>()?;
        let flatten_pyarray = flatten_data_bytes.to_pyarray(py);
        Ok(flatten_pyarray.reshape(Ix2(rows as usize, cols as usize))?)
    }
}

fn mat_to_continuous(m: &Mat) -> Result<Cow<Mat>> {
    Ok(if m.is_continuous()? {
        Cow::Borrowed(m)
    } else {
        Cow::Owned(m.clone())
    })
}
