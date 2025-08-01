use crate::def::{AttrVal, Count, OF};

pub(crate) enum Output<T>
where
    T: Copy + Clone,
{
    Simple(OutputSimple<T>),
    Complex(OutputComplex<T>),
}
impl<T> Output<T>
where
    T: Copy + Clone + std::ops::Mul<AttrVal, Output = T>,
{
    pub(in crate::svc) fn get_total(&self) -> T {
        match self {
            Output::Simple(simple) => simple.get_total(),
            Output::Complex(complex) => complex.get_total(),
        }
    }
    pub(in crate::svc) fn get_max(&self) -> T {
        match self {
            Output::Simple(simple) => simple.get_max(),
            Output::Complex(complex) => complex.get_max(),
        }
    }
}

pub(crate) struct OutputSimple<T>
where
    T: Copy + Clone,
{
    pub(crate) amount: T,
    pub(crate) delay: AttrVal,
}
impl<T> OutputSimple<T>
where
    T: Copy + Clone,
{
    pub(in crate::svc) fn get_total(&self) -> T {
        self.amount
    }
    pub(in crate::svc) fn get_max(&self) -> T {
        self.amount
    }
}

pub(crate) struct OutputComplex<T>
where
    T: Copy + Clone,
{
    pub(crate) amount: T,
    pub(crate) delay: AttrVal,
    pub(crate) repeats: Count,
    pub(crate) interval: AttrVal,
}
impl<T> OutputComplex<T>
where
    T: Copy + Clone,
{
    pub(in crate::svc) fn get_max(&self) -> T {
        self.amount
    }
}
impl<T> OutputComplex<T>
where
    T: Copy + Clone + std::ops::Mul<AttrVal, Output = T>,
{
    pub(in crate::svc) fn get_total(&self) -> T {
        self.amount * OF(self.repeats as f64)
    }
}
