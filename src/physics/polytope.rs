use std::{iter::Sum, ops::{Add, AddAssign, Div, Mul}};

use crate::physics::SimplePolytope;

use mathnd::vecn::VecN;
use num_traits::{Bounded, FromPrimitive, One, Signed, Zero};

#[derive(Clone)]
pub struct Polytope<T, const N: usize> {
    pub center: VecN<T, N>,
    pub elements: [SimplePolytope<T, N>; N],
}

impl<T, const N: usize> Polytope<T, N> where T: 
Add<Output = T> + Mul<Output = T> + AddAssign +
PartialOrd + Sum + Signed + Zero + One + Bounded +
FromPrimitive + Copy {
    pub fn support(&self, dir: &VecN<T, N>) -> [VecN<T, N>; N] {
        std::array::from_fn(|i| self.elements[i].support(dir) + self.center)
    }

    pub fn elementsupport(&self, dir: &VecN<T, N>, element: usize) -> VecN<T, N> {
        self.elements[element].support(dir) + self.center
    }

    // pub fn dual(&self) -> Polytope<T, N> {
    //     Polytope {
    //         elements: std::array::from_fn(|i| ((*self).elements[N - i - 1]).clone()),
    //     }
    // }
}

impl<T> Polytope<T, 3> where T: Div<Output = T> + Zero + One + FromPrimitive + Copy {
    pub fn cube() -> Self {
        let verts: SimplePolytope<T, 3> = SimplePolytope {
            // center: VecN::<T, 3>::zero(),
            zonotopes: vec![vec![
                VecN::basis(0),
                VecN::basis(1),
                VecN::basis(2),
            ]],
            zextend: vec![vec![true; 3]],
            vertices: Vec::new(),
            vextend: Vec::new(),
        };
        let edges: SimplePolytope<T, 3> = SimplePolytope {
            zonotopes: vec![
                vec![
                    VecN::basis(0),
                    VecN::basis(1),
                ],
                vec![
                    VecN::basis(0),
                    VecN::basis(2),
                ],
                vec![
                    VecN::basis(1),
                    VecN::basis(2),
                ]
            ],
            zextend: vec![vec![true; 2]; 3],
            vertices: Vec::new(),
            vextend: Vec::new(),
        };
        let faces: SimplePolytope<T, 3> = SimplePolytope {
            zonotopes: Vec::new(),
            zextend: Vec::new(),
            vertices: vec![
                VecN::basis(0),
                VecN::basis(1),
                VecN::basis(2),
            ],
            vextend: vec![true; 3],
        };
        Polytope {
            center: VecN::<T, 3>::zero(),
            elements: [verts, edges, faces],
        }
    }

    pub fn octahedron() -> Self {
        let mut cube = Self::cube();
        cube.elements.reverse();

        for (i, element) in cube.elements.iter_mut().enumerate() {
            for zonotope in element.zonotopes.iter_mut() {
                for vec in zonotope.iter_mut() {
                    *vec = *vec / T::from_usize(i+1).unwrap();
                }
            }
            for vec in element.vertices.iter_mut() {
                *vec = *vec / T::from_usize(i+1).unwrap();
            }
        }

        cube
    }
}