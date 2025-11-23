fn bivectortomatrix(bivec: BiVector) -> Matrix {
    let mut mat: Matrix = Matrix::zeros();
    let mut k = 0;
    for i in 0..(DIMENSION-1) {
        for j in (i+1)..(DIMENSION) {
            mat[(i,j)] = bivec[k];
            mat[(j,i)] = -bivec[k];
            k+=1;
        }
    }
    return mat;
}

fn matrixtobivector(mat: Matrix) -> BiVector {
    let mut bivec: BiVector = BiVector::zeros();
    let mut k = 0;
    for i in 0..(DIMENSION-1) {
        for j in (i+1)..(DIMENSION) {
            bivec[k] = mat[(i,j)];
            k+=1;
        }
    }
    return bivec;
}

fn rotationmatrixfromvectors(v1: Unit<Vector>, v2: Unit<Vector>) -> Matrix {
    let v3: Vector = v1.as_ref() + v2.as_ref();
    return Matrix::identity() - (v3 * v3.transpose() / (1.0 + v1.dot(&v2))) + (2.0 * v2.as_ref() * v1.as_ref().transpose());
}

fn bivectormatrix(v1: Vector, v2: Vector) -> Matrix {
    return v1 * v2.transpose() - v2 * v1.transpose();
}

fn matrixdotproduct(m1: Matrix, m2: Matrix) -> Scalar {
    return (m1 / m1.norm()).component_mul(&(m2 / m2.norm())).sum();
}