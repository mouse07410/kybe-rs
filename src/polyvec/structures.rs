//! Structures
//!
//! Basic Traits for algebraic structures

/// Finite Group element
pub trait FiniteGroup: Sized + Eq {
    /// Check if the element is the additive identity
    fn is_zero(&self) -> bool;

    /// Returns the additive identity
    fn zero(&self) -> Self;

    /// Returns the additive inverse of the element
    fn neg(&self) -> Self;

    /// Defines the addition of two elements
    fn add(&self, other: &Self) -> Self;

    /// Defines the substraction of two elements
    fn sub(&self, other: &Self) -> Self;
}

/// Finite Ring element
pub trait FiniteRing: Sized + Eq {
    /// Check if the element is the additive identity
    fn is_zero(&self) -> bool;

    /// Returns the additive identity
    fn zero(&self) -> Self;

    /// Returns the additive inverse of the element
    fn neg(&self) -> Self;

    /// Defines the addition of two elements
    fn add(&self, other: &Self) -> Self;

    /// Defines the substraction of two elements
    fn sub(&self, other: &Self) -> Self;

    /// Returns the multiplicative identity
    fn one(&self) -> Self;

    /// Defines the multiplication of two elements
    fn mul(&self, other: &Self) -> Self;
}

/// Finite field element
pub trait FiniteField: Sized + Eq {
    /// Check if the element is the additive identity
    fn is_zero(&self) -> bool;

    /// Returns the additive identity
    fn zero(&self) -> Self;

    /// Returns the additive inverse of the element
    fn neg(&self) -> Self;

    /// Defines the addition of two elements
    fn add(&self, other: &Self) -> Self;

    /// Defines the substraction of two elements
    fn sub(&self, other: &Self) -> Self;

    /// Returns the multiplicative identity
    fn one(&self) -> Self;

    /// Defines the multiplication of two elements
    fn mul(&self, other: &Self) -> Self;

    /// Returns the dimension of the finite field
    fn dimension(&self) -> usize;

    /// Returns the multiplicative inverse of the element
    fn inv<'a>(&self) -> Result<Self, &'a str>;

    /// Defines the divison of two elements
    fn div<'a>(&self, other: &Self) -> Result<Self, &'a str>;
}

/// The `Vector` trait describes the general properties of an element in a vector space.
pub trait VectorSpace<T: FiniteField> {
    /// Check if the element is the additive identity
    fn is_zero(&self) -> bool;

    /// Returns the additive identity
    fn zero(&self) -> Self;

    /// Returns the additive inverse of the element
    fn neg(&self) -> Self;

    /// Defines the addition of two elements
    fn add(&self, other: &Self) -> Self;

    /// Defines the substraction of two elements
    fn sub(&self, other: &Self) -> Self;

    /// Returns the vector's dimension
    fn dimension(&self) -> usize;

    /// Initialise vector type
    fn init(dimension: usize) -> Self;

    /// Scalar multiplication
    fn mulf(&self, other: &T) -> Self;

    /// Basis vector
    fn basis_vector(&self, position: usize) -> Self;

    /// Set coefficient
    fn set(&mut self, position: usize, value: T);

    /// Get coefficient
    fn get(&self, position: usize) -> T;

    /// Dot product
    fn dot(&self, other: &Self) -> T;
}

/// The `Vector` trait describes the general properties of an element in a module.
pub trait RingModule<T: FiniteRing> {
    /// Check if the element is the additive identity
    fn is_zero(&self) -> bool;

    /// Returns the additive identity
    fn zero(&self) -> Self;

    /// Returns the additive inverse of the element
    fn neg(&self) -> Self;

    /// Defines the addition of two elements
    fn add(&self, other: &Self) -> Self;

    /// Defines the substraction of two elements
    fn sub(&self, other: &Self) -> Self;

    /// Returns the vector's dimension
    fn dimension(&self) -> usize;

    /// Initialise vector type
    fn init(dimension: usize) -> Self;

    /// Scalar multiplication
    fn mulf(&self, other: &T) -> Self;

    /// Basis vector
    fn basis_vector(&self, position: usize) -> Self;

    /// Set coefficient
    fn set(&mut self, position: usize, value: T);

    /// Get coefficient
    fn get(&self, position: usize) -> T;

    /// Dot product
    fn dot(&self, other: &Self) -> T;
}
