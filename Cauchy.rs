use std::fmt;
use std::ops;
use std::cmp;

#[derive(Debug)]
pub struct CauchyList {
    pub p: i32,
    pub content: Vec<i32>
}

impl CauchyList {
    // CauchyList methods go here, if desired!
}

impl cmp::PartialEq for CauchyList {
    fn eq(&self, other: &Self) -> bool {    
        // Implement == operation here
        println!("------------");
        println!("x is: {:?}", self);
        println!("y is: {:?}", other);

        println!("------------");
        if self.content.len() != other.content.len(){
            return false;
        }
        let mut i = 0;
        for t in &other.content{
            if self.content[i] != *t{
                return false;
            }
            i += 1;
        }
        return true;
    }
}

impl ops::Add<CauchyList> for CauchyList {
    type Output = CauchyList;
    fn add(self, other: CauchyList) -> CauchyList {
        // Implement + operation here

        let out_len: i32;
        if self.content.len() >= other.content.len() {
            out_len = self.content.len() as i32 + 1;
            let mut out_l = vec![0; self.content.len()];
            let mut i = 0;
            for t in other.content{
                out_l[i] = self.content[i] + t;
                while out_l[i]>self.p{
                    out_l[i] -= 31;
                }
                i += 1;

            }
            return CauchyList {p: out_len, content: out_l};
        }
        else {
            out_len = other.content.len() as i32 + 1;
            let mut out_l = vec![0; other.content.len()];
            let mut i = 0;
            for t in other.content{
                out_l[i] = self.content[i] + t;
                while out_l[i]>self.p{
                    out_l[i] -= 31;
                }
                i += 1;

            }
            return CauchyList {p: out_len, content: out_l};
        }
    }
}

impl ops::Sub<CauchyList> for CauchyList {
    type Output = CauchyList;
    fn sub(self, other: CauchyList) -> CauchyList {
        // Implement - operation here

        let out_len: i32;
        if self.content.len() >= other.content.len() {
            out_len = self.content.len() as i32 + 1;
            let mut out_l = vec![0; self.content.len()];
            let mut i = 0;
            for t in other.content{
                out_l[i] = self.content[i] - t;
                while out_l[i]>self.p{
                    out_l[i] -= 31;
                }
                i += 1;

            }
            return CauchyList {p: out_len, content: out_l};
        }
        else {
            out_len = other.content.len() as i32 + 1;
            let mut out_l = vec![0; other.content.len()];
            let mut i = 0;
            for t in other.content{
                out_l[i] = self.content[i] - t;
                while out_l[i]>self.p{
                    out_l[i] -= 31;
                }
                i += 1;

            }
            return CauchyList {p: out_len, content: out_l};
        }
    }
}

impl ops::Mul<CauchyList> for CauchyList {
    type Output = CauchyList;
    fn mul(self, other: CauchyList) -> CauchyList {
        // Implement * operation here

        let out_len: i32;
        if self.content.len() >= other.content.len() {
            out_len = self.content.len() as i32;
            let mut out_l = vec![0; self.content.len()];
            let mut i = 0;
            for t in other.content{

                out_l[i] = self.content[i] * t;
                
                while out_l[i]>self.p{
                    out_l[i] -= 31;
                }
                i += 1;
                
            }
            return CauchyList {p: out_len, content: out_l};
        }
        else {
            out_len = other.content.len() as i32;
            let mut out_l = vec![0; other.content.len()];
            let mut i = 0;
            for t in other.content{
                out_l[i] = self.content[i] * t;
                while out_l[i]>self.p{
                    out_l[i] -= 31;
                }
                i += 1;

            }
            return CauchyList {p: out_len, content: out_l};
        }

    }
}

impl ops::Mul<i32> for CauchyList {
    type Output = CauchyList;
    fn mul(self, other: i32) -> CauchyList {
        // Implement * operation here
        let out_len = self.content.len() as i32 + 1;
        let mut out_l = vec![0; self.content.len()];
        let mut i = 0;
        for t in self.content{
            out_l[i] = t * other;
            while out_l[i]>self.p{
                out_l[i] -= 31;
            }
            i += 1;

        }
        return CauchyList {p: out_len, content: out_l};

    }
}

impl fmt::Display for CauchyList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {    
        // Implement print formatting here       
        write!(f, "p: {}\nlen: {}\ncontent: {}", self.p, self.content.len(), format!("{:?}", self.content))
    }
}

// fn main() {
//     let x = CauchyList{ p: 3, content: [1, 10, 100].to_vec() };
//     let y = CauchyList{ p: 3, content: [2, 20, 200].to_vec() };
//     // let y = 3;
//     let z = x * y;
//     println!("{}", z)
// }