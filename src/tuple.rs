use std::ops::{Add, Sub};

struct Tuple {
    x:f64,
    y:f64,
    z:f64,
    w:f64
}
pub fn almost_equal(a:f64,b:f64)->bool{
    return (a-b).abs()<0.0001
}
impl Tuple {
    pub fn new_tuple(x: f64, y: f64,z: f64,w:f64) -> Tuple {
        Tuple { x: x, y: y,z:z,w:w } // this is fine, as we're in the same module
    }
    pub fn new_point(x: f64, y: f64,z: f64) -> Tuple {
        Tuple { x: x, y: y,z:z,w:1.0 } // this is fine, as we're in the same module
    }
    pub fn new_vector(x: f64, y: f64,z: f64) -> Tuple {
        Tuple { x: x, y: y,z:z,w:0.0 } // this is fine, as we're in the same module
    }
    pub fn is_point(&self) -> bool {
        return almost_equal(self.w,1.0)
    }
    pub fn is_vector(&self) -> bool {
        return almost_equal(self.w,0.0)
    }

}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        return almost_equal(self.x,other.x) && almost_equal(self.y,other.y) &&almost_equal(self.z,other.z) &&almost_equal(self.w,other.w)
    }
}
impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple { x: self.x+other.x,
            y: self.y+other.y,
            z:self.z+other.z,
            w:self.w+other.w
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple { x: self.x-other.x,
            y: self.y-other.y,
            z:self.z-other.z,
            w:self.w-other.w
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::tuple::{Tuple, almost_equal};

    #[test]
    fn tuple_point_creation() {
        let t=Tuple::new_tuple(4.3,-4.2,3.1,1.0);
        assert!(almost_equal(t.x,4.3));
        assert!(almost_equal(t.y,-4.2));
        assert!(almost_equal(t.z,3.1));
        assert!(almost_equal(t.w,1.0));
        assert!(t.is_point());
        assert!(!t.is_vector());
    }

    #[test]
    fn tuple_vector_creation() {
        let t=Tuple::new_tuple(4.3,-4.2,3.1,0.0);
        assert!(almost_equal(t.x,4.3));
        assert!(almost_equal(t.y,-4.2));
        assert!(almost_equal(t.z,3.1));
        assert!(almost_equal(t.w,0.0));
        assert!(!t.is_point());
        assert!(t.is_vector());
    }

    #[test]
    fn point_creation() {
        let t=Tuple::new_point(4.3,-4.0,3.0);
        assert!(t==Tuple::new_tuple(4.3,-4.0,3.0,1.0));
    }

    #[test]
    fn vector_creation() {
        let t=Tuple::new_vector(4.3,-4.0,3.0);
        assert!(t==Tuple::new_tuple(4.3,-4.0,3.0,0.0));
    }


    #[test]
    fn tuple_add() {
        let a=Tuple::new_tuple(3.0,-2.0,5.0,1.0);
        let b=Tuple::new_tuple(-2.0,3.0,1.0,0.0);
        let c:Tuple=a+b;
        assert!(c==Tuple::new_tuple(1.0,1.0,6.0,1.0));
    }

    #[test]
    fn tuple_sub() {
        let a=Tuple::new_point(3.0,2.0,1.0);
        let b=Tuple::new_point(5.0,6.0,7.0);
        let c:Tuple=a-b;
        assert!(c==Tuple::new_vector(-2.0,-4.0,-6.0));
    }

    #[test]
    fn test_sub_point_and_vector() {
        let a=Tuple::new_point(3.0,2.0,1.0);
        let b=Tuple::new_vector(5.0,6.0,7.0);
        let c:Tuple=a-b;
        assert!(c==Tuple::new_point(-2.0,-4.0,-6.0));
    }

    #[test]
    fn test_vector_and_vector() {
        let a=Tuple::new_vector(3.0,2.0,1.0);
        let b=Tuple::new_vector(5.0,6.0,7.0);
        let c:Tuple=a-b;
        assert!(c==Tuple::new_vector(-2.0,-4.0,-6.0));
    }
}