use std::ops::Deref;

///A Datastructure for a 2 dimensional Matrix
///
/// Cloneable
#[derive(Clone)]
pub struct Mat2d<T> where T:Clone{
    pub height: usize,
    pub width: usize,
    pub vector : Vec<T>
}


impl<T> Mat2d<T> where T:Clone{
    ///converts a vector of size (width * height) into a Matrix with
    /// width columns and height rows
    pub fn from(vector : Vec<T>, width: usize, height: usize) -> Self{
        if vector.len() !=  (width * height ) {
            panic!("Mat2d::from() - Vector length != width*height");
        }
        Mat2d{
            vector,
            width,
            height
        }
    }

    pub fn get_width(&self) -> usize{
        self.width
    }
    pub fn get_height(&self) -> usize{
        self.height
    }
    pub fn get_as_vector(&self) -> &Vec<T>{
        &self.vector
    }

    ///Sets all Values to those in the Vector. It's size has to be width * height
    ///
    /// #Examples
    ///
    /// let mut mat: Mat2d<i32> = Mat2d::from(
    ///             vec!\[1,1,1,2,2,2,3,3,3\],
    ///             3,3);
    ///
    ///  let mat_2 = vec!\[2,2,2,4,4,4,6,6,6\];
    ///  mat.set_to_vector(mat_2);
    ///  assert_eq!(*mat.get_value(0,0).unwrap(), 2);
    ///  assert_eq!(*mat.get_value(2,2).unwrap(), 6);

    pub fn set_to_vector(&mut self, vector : Vec<T>){
        assert_eq!(vector.len(), (self.width * self.height));
        self.vector = vector;
    }

    /// resizes the Dimensions of the Matrix.
    /// self.height * self.width has to be the same as width * height
    pub fn resize(&mut self, width : usize, height : usize){
        assert_eq!((self.height*self.width), width*height);
        self.width = width;
        self.height = height;
    }

    ///Creates an Empty Matrix of size 0x0. If you need a default value.
    pub fn empty() -> Self{
        Mat2d {
            height: 0,
            width: 0,
            vector: vec![]
        }
    }

    ///returns the Value in the Position specified
    pub fn get_value(&self, col : usize, row : usize) -> Option<&T> {
        if col >= self.width || row >= self.height{
            return None;
        }
        self.vector.get(col + (self.width * row))
    }

    ///returns the Value in the Position specified, also takes negative numbers, but returns None
    ///
    /// Convenient if you want to find neighbours.
    pub fn get_value_in_position_of_i32(&self, col : i32, row : i32) -> Option<&T> {
        if col<0 || row<0 || col as usize>= self.width || row as usize >= self.height{
            return None;
        }
        self.vector.get(col as usize+ (self.width * row as usize))
    }

    ///Sets the value in the specified position to value
    pub fn set_value(&mut self, col : usize, row : usize, value : T) -> Result<bool, &str> {
        if col >= self.width || row >= self.height {
            return Err("Not in matrix");
        }
        self.vector.remove(col + (self.width * row));
        self.vector.insert(col + (self.width * row), value);
        Ok(true)
    }

    ///creates a new Matrix, which is a copy of the specified Part.
    ///
    /// #Examples
    /// let mat : Mat2d<i32> = Mat2d::from(
    ///         vec![1,1,1,2,2,2,3,3,3],
    ///             3,3);
    /// let mat_part = mat.copy_part(0, 2, 0, 2).unwrap();
    /// assert_eq!(mat_part.get_height() , 2);
    /// assert_eq!(mat_part.get_width() , 2);
    /// assert_eq!(*mat_part.get_value(0,0).unwrap(),1);
    /// assert_eq!(*mat_part.get_value(1,1).unwrap(),2);
    ///
    /// creates:
    /// 1 2
    /// 1 2

    pub fn copy_part(&self, from_col : usize, to_col : usize, from_row : usize, to_row : usize) -> Option<Mat2d<T>> {
        if !to_col < self.width || ! to_row < self.height || from_col > to_col || from_row > to_row {
            return None
        }
        let new_width = (to_col as i32 - from_col as i32) as usize;
        let new_height = (to_row as i32 - from_row as i32) as usize;
        let mut new_vector = Vec::new();

        for row in from_row..to_row{
            for col in from_col..to_col {
                let s = self.vector.as_slice();
                let value = (s[col + (self.width * row)]).clone();
                new_vector.push(value);
            }
        }
        Some(Mat2d::from(new_vector, new_width, new_height))
    }
    ///Splits the Matrix in 2 at the given column.
    /// First everything up to that column, Second everything from column + 1
    ///
    /// #Examples
    ///let mat : Mat2d<i32> = Mat2d::from(
    ///           vec![1,1,1,2,2,2,3,3,3],
    ///            3,3);
    ///         let (s1, s2) = mat.split_vertical(1).unwrap();
    ///         assert_eq!(s1.get_width() ,1);
    ///         assert_eq!(s2.get_width() ,2);
    ///         assert_eq!(**s1.get_value(0,1).unwrap(), 2);
    ///         assert_eq!(**s2.get_value(1,1).unwrap(), 2);
    ///
    pub fn split_vertical(&self, position : usize) -> Option<(Mat2d<T>, Mat2d<T>)> {
        if position + 1 > self.width {return None};
        let right = self.copy_part(
            position,
            self.width,
            0,
            self.height
        ).unwrap();
        let left = self.copy_part(
            0,
            position,
            0 ,
            self.height as usize
        ).unwrap();
        Some((left, right))
    }
    ///Splits the Matrix in 2 at the given row.
    /// First everything up to that row, Second everything from row + 1
    ///
    ///  #Examples
    /// let mat : Mat2d<i32> = Mat2d::from(
    ///             vec![1,1,1,2,2,2,3,3,3],
    ///             3,3);
    ///         let (s1, s2) = mat.split_horizontal(1).unwrap();
    ///         assert_eq!(s1.get_height() ,1);
    ///         assert_eq!(s2.get_height() ,2);
    ///         assert_eq!(s1.get_width(), 3);
    ///         assert_eq!(s2.get_width(), 3);
    pub fn split_horizontal(&self, position: usize) -> Option<(Mat2d<T>, Mat2d<T>)>{
        if (position + 1 > self.height) {return None};
        let bottom = self.copy_part(
            0,
            self.width,
            position,
            self.height
        ).unwrap();
        let top = self.copy_part(
            0,
            self.width,
            0 ,
            position
        ).unwrap();
        Some((top, bottom))
    }
    ///#Examples
    ///
    /// let mut mat: Mat2d<i32> = Mat2d::from(
    ///             vec!\[1,2,3,4,5,6,7,8,9\],
    ///             3,3);
    ///
    /// let inverted = mat.vertical_inverted_copy();
    /// assert_eq!(*inverted.get_value(0,0,).unwrap(), 3);
    /// assert_eq!(*inverted.get_value(1,0,).unwrap(), 2);
    /// assert_eq!(*inverted.get_value(2,0,).unwrap(), 1);
    /// assert_eq!(*inverted.get_value(0,1,).unwrap(), 6);
    /// assert_eq!(*inverted.get_value(1,1,).unwrap(), 5);
    /// assert_eq!(*inverted.get_value(2,1,).unwrap(), 4);
    /// assert_eq!(*inverted.get_value(0,2,).unwrap(), 9);
    /// assert_eq!(*inverted.get_value(1,2,).unwrap(), 8);
    /// assert_eq!(*inverted.get_value(2,2,).unwrap(), 7);
    pub fn vertical_inverted_copy(&self) -> Mat2d<T>{
        let mut vector = Vec::new();
        for row in 0..self.height{
            for col in 0..self.width{
                        let col_inverted = (self.width as i32 - (col as i32 +1)) as usize;
                        let value = (self.vector[col_inverted + (self.width * row)]).clone();
                        vector.push(value);
            }
        }
        Mat2d::from(vector, self.width, self.height)
    }
    pub fn horizontal_inverted_copy(&self) -> Mat2d<T>{
        let mut vector = Vec::new();
        for row in 0..self.height{
            let row_inverted = (self.height as i32 - (row as i32 +1)) as usize;
            for col in 0..self.width{
                let value = (self.vector[col + (self.width * row_inverted)]).clone();
                vector.push(value);
            }
        }
        Mat2d::from(vector, self.width, self.height)
    }
}

#[cfg(test)]
mod test{
    use crate::mat2d::Mat2d;

    #[test]
    fn test_split(){
       let mat : Mat2d<i32> = Mat2d::from(
           vec![1,1,1,2,2,2,3,3,3],
           3,3);
        let (s1, s2) = mat.split_vertical(1).unwrap();
        assert_eq!(s1.get_width() ,1);
        assert_eq!(s2.get_width() ,2);
        assert_eq!(s1.get_height(), 3);
        for x in 0..s1.width{
            for y in 0..s1.height{
                print!("{} ", s1.get_value(x,y).unwrap());
            }
            println!();
        }
        for x in 0 ..s1.width{
            for y in 0..s1.height{
                print!("{} ", s1.get_value(x,y).unwrap());
            }
            println!();
        }
        for x in 0 ..s2.width{
            for y in 0..s2.height{
                print!("{} ", s2.get_value(x,y).unwrap());
            }
            println!();
        }
        assert_eq!(*s1.get_value(0,0).unwrap(), 1);
        assert_eq!(*s2.get_value(1,1).unwrap(), 2);
    }
    #[test]
    fn test_get_partition(){
        let mat : Mat2d<i32> = Mat2d::from(
            vec![1,1,1,2,2,2,3,3,3],
            3,3);
        let mat_part = mat.copy_part(0, 2, 0, 2).unwrap();

        for x in 0 ..mat_part.width{
            for y in 0..mat_part.height{
                print!("{} ", mat_part.get_value(x,y).unwrap());
            }
            println!();
        }
        assert_eq!(mat_part.get_height() , 2);
        assert_eq!(mat_part.get_width() , 2);
        assert_eq!(*mat_part.get_value(0,0).unwrap(),1);
        assert_eq!(*mat_part.get_value(1,1).unwrap(),2);
    }

    #[test]
    fn test_split_horizontal(){
        let mat : Mat2d<i32> = Mat2d::from(
            vec![1,1,1,2,2,2,3,3,3],
            3,3);
        let (s1, s2) = mat.split_horizontal(1).unwrap();
        assert_eq!(s1.get_height() ,1);
        assert_eq!(s2.get_height() ,2);
        assert_eq!(s1.get_width(), 3);
        assert_eq!(s2.get_width(), 3);
    }

    #[test]
    fn test_set_vec(){
        let mut mat: Mat2d<i32> = Mat2d::from(
            vec![1,1,1,2,2,2,3,3,3],
            3,3);
        let mat_2 = vec![2,2,2,4,4,4,6,6,6];
        mat.set_to_vector(mat_2);
        assert_eq!(*mat.get_value(0,0).unwrap(), 2);
        assert_eq!(*mat.get_value(2,2).unwrap(), 6);
    }

    #[test]
    fn test_invert_vertical(){
        let mut mat: Mat2d<i32> = Mat2d::from(
            vec![1,2,3,4,5,6,7,8,9],
            3,3);
        let inverted = mat.vertical_inverted_copy();
        assert_eq!(*inverted.get_value(0,0,).unwrap(), 3);
        assert_eq!(*inverted.get_value(1,0,).unwrap(), 2);
        assert_eq!(*inverted.get_value(2,0,).unwrap(), 1);
        assert_eq!(*inverted.get_value(0,1,).unwrap(), 6);
        assert_eq!(*inverted.get_value(1,1,).unwrap(), 5);
        assert_eq!(*inverted.get_value(2,1,).unwrap(), 4);
        assert_eq!(*inverted.get_value(0,2,).unwrap(), 9);
        assert_eq!(*inverted.get_value(1,2,).unwrap(), 8);
        assert_eq!(*inverted.get_value(2,2,).unwrap(), 7);
    }

    #[test]
    fn test_invert_horizontal(){
        let mut mat: Mat2d<i32> = Mat2d::from(
            vec![1,2,3,4,5,6,7,8,9],
            3,3);
        let inverted = mat.horizontal_inverted_copy();
        assert_eq!(*inverted.get_value(0,0,).unwrap(), 7);
        assert_eq!(*inverted.get_value(1,0,).unwrap(), 8);
        assert_eq!(*inverted.get_value(2,0,).unwrap(), 9);
        assert_eq!(*inverted.get_value(0,1,).unwrap(), 4);
        assert_eq!(*inverted.get_value(1,1,).unwrap(), 5);
        assert_eq!(*inverted.get_value(2,1,).unwrap(), 6);
        assert_eq!(*inverted.get_value(0,2,).unwrap(), 1);
        assert_eq!(*inverted.get_value(1,2,).unwrap(), 2);
        assert_eq!(*inverted.get_value(2,2,).unwrap(), 3);
    }

}