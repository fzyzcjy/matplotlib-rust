use std::borrow::Cow;

use numpy::{PyArray, ToPyArray};
use opencv::core::*;
use pyo3::Python;

pub trait MatToPyArray {
    fn to_pyarray<'py>(&self, _: Python<'py>) -> &'py PyArray<Self::Item, Self::Dim>;
}

impl MatToPyArray for Mat {
    fn to_pyarray<'py>(&self, py: Python<'py>) -> &'py PyArray<Self::Item, Self::Dim> {
        let mat_continuous = mat_to_continuous(self);
        let data_bytes = mat_continuous.as_ref().data_bytes()?;
        data_bytes.to_pyarray(py)
    }
}

fn mat_to_continuous(m: &Mat) -> Cow<Mat> {
    if m.is_continuous() {
        Cow::Borrowed(m)
    } else {
        Cow::Owned(m.clone())
    }
}
