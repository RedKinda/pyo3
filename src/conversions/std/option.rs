use crate::{Borrowed, Bound};
use crate::{
    BoundObject, FromPyObject, PyAny, Python, conversion::IntoPyObject, types::any::PyAnyMethods,
};

impl<'py, T> IntoPyObject<'py> for Option<T>
where
    T: IntoPyObject<'py>,
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = T::Error;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        self.map_or_else(
            || Ok(py.None().into_bound(py)),
            |val| {
                val.into_pyobject(py)
                    .map(BoundObject::into_any)
                    .map(BoundObject::into_bound)
            },
        )
    }

    #[cfg(feature = "experimental-inspect")]
    fn type_output() -> crate::inspect::types::TypeInfo {
        crate::inspect::types::TypeInfo::optional_of(T::type_output())
    }
}

impl<'a, 'py, T> IntoPyObject<'py> for &'a Option<T>
where
    &'a T: IntoPyObject<'py>,
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = <&'a T as IntoPyObject<'py>>::Error;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        self.as_ref().into_pyobject(py)
    }

    #[cfg(feature = "experimental-inspect")]
    fn type_output() -> crate::inspect::types::TypeInfo {
        crate::inspect::types::TypeInfo::optional_of(<&'a T as IntoPyObject<'py>>::type_output())
    }
}

impl<'a, 'py, T> FromPyObject<'a, 'py> for Option<T>
where
    T: FromPyObject<'a, 'py>,
{
    type Error = T::Error;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if obj.is_none() {
            Ok(None)
        } else {
            obj.extract().map(Some)
        }
    }
}
