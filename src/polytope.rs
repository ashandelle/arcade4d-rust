use nalgebra::{SVector, Scalar, Unit};

pub(crate) trait ScalarType: Scalar + num_traits::identities::Zero + std::ops::AddAssign + std::ops::MulAssign + std::ops::Mul + nalgebra::ClosedMulAssign + std::cmp::PartialOrd {}
impl<T> ScalarType for T where T: Scalar + num_traits::identities::Zero + std::ops::AddAssign + std::ops::MulAssign + std::ops::Mul + nalgebra::ClosedMulAssign + std::cmp::PartialOrd {}

pub(crate) struct Polytope<T: ScalarType, const D: usize> {
    center: SVector<T, D>,
    zonotopes: Vec<Vec<SVector<T, D>>>,
    zextend: Vec<Vec<bool>>, // TODO Replace with optomized structure
    vertices: Vec<SVector<T, D>>,
    vextend: Vec<bool>, // TODO Replace with optomized structure
}

// struct PolytopePoint<T: ScalarType, const D: usize> {
//     zonotopes: Vec<Vec<T>>,
//     vertices: Vec<T>,
// }

// impl<T: ScalarType, const D: usize> PolytopePoint<T, D> {
//     fn new(polytope: Polytope<T, D>) -> Self {
//         let mut zonotopes: Vec<Vec<T>> = Vec::new();
//         let mut vertices: Vec<T> = Vec::new();

//         for vec in &polytope.zonotopes {
//             let mut zonotope: Vec<T> = Vec::new();
//             for generator in vec {
//                 zonotope.push(T::zero());
//             }
//             zonotopes.push(zonotope);
//         }

//         for vertex in &polytope.vertices {
//             vertices.push(T::zero());
//         }

//         PolytopePoint {
//             zonotopes: zonotopes,
//             vertices: vertices,
//         }
//     }

//     fn clip(&self, polytope: Polytope<T, D>) {
//         // TODO
//     }
// }

impl<T: ScalarType, const D: usize> Polytope<T, D> {
    fn support(&self, dir: Unit<SVector<T, D>>) -> SVector<T, D> {
        let mut dist: T = T::zero(); // Should probably be -inf
        let mut furthest: SVector<T, D> = SVector::<T, D>::zeros();

        for zonotope in &self.zonotopes {
            let mut dot: T = T::zero();
            let mut point: SVector<T, D> = SVector::<T, D>::zeros();
            for generator in zonotope {
                let d: T = generator.dot(&dir);
                if (d > T::zero()) {
                    dot += d;
                    point += generator;
                }
            }
            if (dot > dist) {
                dist = dot;
                furthest = point;
            }
        }

        for vertex in &self.vertices {
            let dot: T = vertex.dot(&dir);
            if (dot > dist) {
                dist = dot;
                furthest = vertex.clone();
            }
        }

        furthest
    }

    fn nearestpoint(&self, point: SVector<T, D>, eps: T) -> SVector<T, D> {
        let mut point: SVector<T, D> = SVector::<T, D>::zeros();

        // TODO

        return point
    }
}