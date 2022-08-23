pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[allow(dead_code)]
impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[allow(dead_code)]
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[allow(dead_code)]
impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    fn get_index(&self, i: usize) -> f32 {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            _ => panic!("Vector4::get_index(): Out of bounds index provided!"),
        }
    }

    // Make this consuming or no? probably no
    pub fn as_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    pub fn add_scalar(&mut self, n: f32) {
        //self.iter();
        self.x += n;
        self.y += n;
        self.z += n;
        self.w += n;
    }
    pub fn subtract_scalar(&mut self, n: f32) {
        self.x -= n;
        self.y -= n;
        self.z -= n;
        self.w -= n;
    }
    pub fn multiply_scalar(&mut self, n: f32) {
        self.x *= n;
        self.y *= n;
        self.z *= n;
        self.w *= n;
    }
    pub fn divide_scalar(&mut self, n: f32) {
        todo!();
    }
}

// impl Iterator for Vector4 {
//     type Item = f32;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self
//     }
// }

// @TODO: not sure if needed
// impl IntoIterator for Vector4{
//     type Item = f32;
//     type IntoIter = std::array::IntoIter<f32 ,4>;

//     fn into_iter(self) -> Self::IntoIter {
//         IntoIterator::into_iter([self.x, self.y, self.z, self.w])
//     }
// }

#[allow(dead_code)]
//Each vec here is a COLUMN
struct Matrix2 {
    pub x: Vector2,
    pub y: Vector2,
}

#[allow(dead_code)]
struct Matrix3 {
    pub x: Vector3,
    pub y: Vector3,
    pub z: Vector3,
}

#[allow(dead_code)]
struct Matrix4 {
    pub x: Vector4,
    pub y: Vector4,
    pub z: Vector4,
    pub w: Vector4,
}

#[allow(dead_code)]
impl Matrix2 {
    pub fn new(c0r0: f32, c0r1: f32, c1r0: f32, c1r1: f32) -> Self {
        Matrix2::from_columns(Vector2::new(c0r0, c0r1), Vector2::new(c1r0, c1r1))
    }

    pub fn from_columns(c0: Vector2, c1: Vector2) -> Matrix2 {
        Matrix2 { x: c0, y: c1 }
    }

    pub fn identity() -> Matrix2 {
        Matrix2::new(1.0, 0.0, 0.0, 1.0)
    }
}

#[allow(dead_code)]
impl Matrix3 {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new(
        c0r0: f32, c0r1: f32, c0r2: f32,
        c1r0: f32, c1r1: f32, c1r2: f32,
        c2r0: f32, c2r1: f32, c2r2: f32,
    ) -> Self {
        Matrix3::from_columns(
            Vector3::new(c0r0, c0r1, c0r2),
            Vector3::new(c1r0, c1r1, c1r2),
            Vector3::new(c2r0, c2r1, c2r2),
        )
    }

    pub fn from_columns(c0: Vector3, c1: Vector3, c2: Vector3) -> Matrix3 {
        Matrix3 {
            x: c0,
            y: c1,
            z: c2,
        }
    }

    pub fn identity() -> Matrix3 {
        Matrix3::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)
    }
}

#[allow(dead_code)]
impl Matrix4 {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new(
        c0r0: f32, c0r1: f32, c0r2: f32, c0r3: f32,
        c1r0: f32, c1r1: f32, c1r2: f32, c1r3: f32,
        c2r0: f32, c2r1: f32, c2r2: f32, c2r3: f32,
        c3r0: f32, c3r1: f32, c3r2: f32, c3r3: f32,
    ) -> Self {
        Matrix4::from_columns(
            Vector4::new(c0r0, c0r1, c0r2, c0r3),
            Vector4::new(c1r0, c1r1, c1r2, c1r3),
            Vector4::new(c2r0, c2r1, c2r2, c2r3),
            Vector4::new(c3r0, c3r1, c3r2, c3r3),
        )
    }

    pub fn from_columns(c0: Vector4, c1: Vector4, c2: Vector4, c3: Vector4) -> Matrix4 {
        Matrix4 {
            x: c0,
            y: c1,
            z: c2,
            w: c3,
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn identity() -> Matrix4 {
        Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn get_row(&self, i: usize) -> [f32; 4] {
        if i > 3 {
            panic!("Index provided to get_row is out of bounds!");
        } else {
            let row: [f32; 4] = [
                self.x.get_index(i),
                self.y.get_index(i),
                self.z.get_index(i),
                self.w.get_index(i),
            ];

            row
        }

        // // non-consuming
        // pub fn to_array(self) -> [[f32; 4]; 4] {
        //     let swag =
        // }
    }

    pub fn get_column(&self, i: usize) -> [f32; 4] {
        match i {
            0 => self.x.as_array(),
            1 => self.y.as_array(),
            2 => self.z.as_array(),
            3 => self.w.as_array(),
            _ => panic!("Index provided to get_column is out of bounds!"),
        }
    }

    pub fn add_scalar(&mut self, n: f32) {
        self.x.add_scalar(n);
        self.y.add_scalar(n);
        self.z.add_scalar(n);
        self.w.add_scalar(n);
    }

    pub fn subtract_scalar(&mut self, n: f32) {
        self.x.subtract_scalar(n);
        self.y.subtract_scalar(n);
        self.z.subtract_scalar(n);
        self.w.subtract_scalar(n);
    }

    pub fn multiply_scalar(&mut self, n: f32) {
        self.x.multiply_scalar(n);
        self.y.multiply_scalar(n);
        self.z.multiply_scalar(n);
        self.w.multiply_scalar(n);
    }

    pub fn divide_scalar(&mut self, n: f32) {
        todo!();
    }

    // #[cfg_attr(rustfmt, rustfmt_skip)]
    // remember that each Vector is a COLUMN
    // traverse left to right of LEFT matrix , top to bottom of RIGHT matrix
    // Consuming bc we dont want the old matrix
    pub fn transform(&mut self, m: Matrix4) {
        let col0 = Vector4::new(
            Matrix4::multiply_col_with_row(m.get_column(0), self.get_row(0)),
            Matrix4::multiply_col_with_row(m.get_column(0), self.get_row(1)),
            Matrix4::multiply_col_with_row(m.get_column(0), self.get_row(2)),
            Matrix4::multiply_col_with_row(m.get_column(0), self.get_row(3)),
        );

        let col1 = Vector4::new(
            Matrix4::multiply_col_with_row(m.get_column(1), self.get_row(0)),
            Matrix4::multiply_col_with_row(m.get_column(1), self.get_row(1)),
            Matrix4::multiply_col_with_row(m.get_column(1), self.get_row(2)),
            Matrix4::multiply_col_with_row(m.get_column(1), self.get_row(3)),
        );

        let col2 = Vector4::new(
            Matrix4::multiply_col_with_row(m.get_column(2), self.get_row(0)),
            Matrix4::multiply_col_with_row(m.get_column(2), self.get_row(1)),
            Matrix4::multiply_col_with_row(m.get_column(2), self.get_row(2)),
            Matrix4::multiply_col_with_row(m.get_column(2), self.get_row(3)),
        );

        let col3 = Vector4::new(
            Matrix4::multiply_col_with_row(m.get_column(3), self.get_row(0)),
            Matrix4::multiply_col_with_row(m.get_column(3), self.get_row(1)),
            Matrix4::multiply_col_with_row(m.get_column(3), self.get_row(2)),
            Matrix4::multiply_col_with_row(m.get_column(3), self.get_row(3)),
        );

        self.x = col0;
        self.y = col1;
        self.z = col2;
        self.w = col3;
    }

    fn multiply_col_with_row(col: [f32; 4], row: [f32; 4]) -> f32 {
        let mut sum = 0.0;

        for i in 0..col.len() {
            sum += col[i] * row[i];
        }

        sum
    }

    pub fn print(&self) {
        //for i print self.x.getindex(i)?
        for i in 0..4 {
            print!(
                "{} {} {} {}",
                self.x.get_index(i),
                self.y.get_index(i),
                self.z.get_index(i),
                self.w.get_index(i)
            );
            println!();
        }
    }
}

// ###########################  TESTS  ####################################################################
#[cfg(test)]
mod vector_tests {
    use crate::math::Vector4;
    #[test]
    fn test_get_index() {
        let vec = Vector4::new(0.0, 1.0, 2.0, 3.0);

        assert_eq!(0.0, vec.get_index(0));
        assert_eq!(1.0, vec.get_index(1));
        assert_eq!(2.0, vec.get_index(2));
        assert_eq!(3.0, vec.get_index(3));
    }

    #[test]
    fn test_as_array() {
        let vec = Vector4::new(0.9, 0.9, 0.9, 0.9);
        let vec_arr = vec.as_array();

        for i in vec_arr {
            assert_eq!(i, 0.9);
        }
    }

    #[test]
    fn test_add_scalar() {
        let mut vec = Vector4::new(0.5, 0.5, 0.6, 0.2);
        vec.add_scalar(2.0);

        let vec = vec.as_array();

        assert_eq!(vec[0], 2.5);
        assert_eq!(vec[1], 2.5);
        assert_eq!(vec[2], 2.6);
        assert_eq!(vec[3], 2.2);
    }

    #[test]
    fn test_subtract_scalar() {
        let mut vec = Vector4::new(1.0, 1.0, 1.0, 1.0);
        vec.subtract_scalar(2.0);

        let vec = vec.as_array();

        for i in vec {
            assert_eq!(i, -1.0);
        }
    }

    #[test]
    fn test_multiply_scalar() {
        let mut vec = Vector4::new(2.0, 2.0, 2.0, 2.0);
        vec.multiply_scalar(0.5);

        let vec = vec.as_array();

        for i in vec {
            assert_eq!(i, 1.0);
        }
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod matrix_tests {
    use super::Matrix4;

    #[test]
    fn test_matrix4_identity() {
        let mat = Matrix4::identity();

        mat.print();

        assert_eq!(mat.x.x, 1.0);
        assert_ne!(mat.x.y, 1.0);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_matrix4_getrow() {
        let mat = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 
            1.0, 2.0, 3.0, 4.0, 
            1.0, 2.0, 3.0, 4.0, 
            1.0, 2.0, 3.0, 4.0,
        );

        mat.print();

        let row = mat.get_row(2);

        for i in row {
            assert_eq!(i, 3.0);
        }
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_matrix4_getcolumn(){
        let mat = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 
            1.0, 2.0, 3.0, 4.0, 
            1.0, 2.0, 3.0, 4.0, 
            1.0, 2.0, 3.0, 4.0,
        );

        let col = mat.get_column(0);

        mat.print();

        assert_eq!(col[0], 1.0);
        assert_eq!(col[1], 2.0);
        assert_eq!(col[2], 3.0);
        assert_eq!(col[3], 4.0);
        
    }

    #[test]
    #[should_panic]
    fn test_matrix4_getcolumn_panic() {
        let mat = Matrix4::identity();

        let _col = mat.get_column(4);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_matrix_transform() {
        let mut m1 = Matrix4::new(
            2.0, 1.0, 6.0, 1.0,
            3.0, 2.0, 7.0, 2.0,
            4.0, 3.0, 8.0, 3.0,
            5.0, 4.0, 9.0, 4.0
        );

        let m2 = Matrix4::new(
            5.0,5.0,6.0,6.0,
            5.0,5.0,6.0,6.0,
            5.0,5.0,6.0,6.0,
            5.0,5.0,6.0,6.0
        );
        println!("Printing m1: ");
        m1.print();
        println!("Printing m2: ");
        m2.print();

        m1.transform(m2);

        println!("Results of m1 x m2: ");
        m1.print();

        let row0 = m1.get_row(0);
        for i in row0 {
            assert_eq!(i, 79.0);
        }
        let col0 = m1.get_column(0);
        
        assert_eq!(col0[0], 79.0);
        assert_eq!(col0[1], 57.0);
        assert_eq!(col0[2], 167.0);
        assert_eq!(col0[3],57.0);

    }

    #[test]
    fn test_multiply_colrow() {
        let col: [f32; 4] = [4.0, 5.0, 3.5, 0.7];
        let row: [f32; 4] = [1.3, 1.2, 0.7, 0.4];

        let res = (Matrix4::multiply_col_with_row(col, row) * 100.0).round() / 100.0; // rounded to two decimal places

        assert_eq!(res, 13.93);
    }
}
