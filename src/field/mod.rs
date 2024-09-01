use std::{ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}, vec};
// Define a field
// Define a field element
// Define arithmetic operations on field elements
//field_modulus is the prime number that defines the field

#[derive(Debug, Clone, Copy)]
pub struct Field(pub u64);
impl Field{
    pub fn new(x:u64)->Field{
        Field(x)
    }
}
impl PartialEq for Field{
    fn eq(&self, other:&Field)-> bool{
        self.0==other.0
    }
}
#[derive(Debug,Clone,Copy)]
pub struct FieldElement(pub u64, pub Field);
impl FieldElement{
    pub fn new (x:u64,field:Field)->FieldElement{
        FieldElement(x%field.0,field)
    }

 pub fn zero(field:Field)-> FieldElement{
    FieldElement(0,field)}
    pub fn one (field:Field)->FieldElement{
        FieldElement(1,field)
    } 
    pub fn modulus (&self)->u64{
        self.1.0
    }
    pub fn inverse(&self)->FieldElement{
        let mut inv=1;
        let mut base = self.0;
        let mut exp= self.1.0-2;
        while exp>0{
            if exp %2==1{
                inv =(inv*base)%self.1.0;

            }
            base = (base*base)%self.1.0;
        }
        FieldElement(inv,self.1)
    }
    pub fn pow(&self, exp:u64)->FieldElement{
        let mut result =1;
        let mut base=self.0;
        let mut exp= exp;
        while exp>0{
            if exp%2==1{
                result = (result*base)%self.1.0;
            }
            base =(base*base)%self.1.0;

        }
        FieldElement(result,self.1)
    }
    pub fn to_bytes(&self)->Vec<u8>{
        let mut e =self.0.to_be_bytes().to_vec();
        let mut f = self.1.0.to_be_bytes().to_vec();
        e.append(& mut f);
        e

    }
    pub fn from_bytes(bytes:&[u8])->FieldElement{
        let mut x =[0u8;8];
        let mut y =[0u8;8];
        x.copy_from_slice(&bytes[..8]);
        y.copy_from_slice(&bytes[8..]);
        FieldElement(
            u64::from_be_bytes(x)%u64::from_be_bytes(y),
            Field(u64::from_be_bytes(y)),)
    }}
    impl Add for FieldElement{
        type Output =FieldElement;
        fn add(self, other:FieldElement)->FieldElement{
            if (self.1)!=other.1{
                panic!("Fields must be same");
            }
            FieldElement((self.0+other.0)%self.1.0,self.1)
        }

    }
   impl AddAssign for FieldElement{
       fn add_assign(&mut self, other:FieldElement){
           if self.1!=other.1{
               panic!("Fields must be same");
           }
           self.0=(self.0+other.0)%self.1.0;
       }
   }

    impl Sub for FieldElement{
         type Output =FieldElement;
         fn sub(self, other:FieldElement)->FieldElement{
              if self.1!=other.1{
                panic!("Fields must be same");
              }
              if self.0<other.0{
                  FieldElement((self.0+ self.1.0-other.0)%self.1.0,self.1)} else{
                      FieldElement((self.0-other.0)%self.1.0,self.1)
                  } } }
                  impl SubAssign for FieldElement{
                      fn sub_assign(&mut self, other:FieldElement){
                          if self.1!=other.1{
                              panic!("Fields must be same");
                          }
                          if self.0<other.0{
                              self.0=(self.0+self.1.0-other.0)%self.1.0;
                          }else{
                              self.0=(self.0-other.0)%self.1.0;
                          }
                      }
                  }
                  impl Mul for FieldElement{
                      type Output =FieldElement;
                      fn mul(self, other:FieldElement)->FieldElement{
                          if self.1!=other.1{
                              panic!("Fields must be same");
                          }
                          FieldElement((self.0*other.0)%self.1.0,self.1)
                      }
                  }
                  impl MulAssign for FieldElement{
                      fn mul_assign(&mut self, other:FieldElement){
                          if self.1!=other.1{
                              panic!("Fields must be same");
                          }
                          self.0=(self.0*other.0)%self.1.0;
                      }
                  }
                  impl Div for FieldElement{
                      type Output =FieldElement;
                      fn div(self, other:FieldElement)->FieldElement{
                          if self.1!=other.1{
                              panic!("Fields must be same");
                          }
                          FieldElement((self.0*other.inverse().0)%self.1.0,self.1)
                      }
                  }
impl DivAssign for FieldElement{
    fn div_assign(&mut self, other:FieldElement){
        if self.1!=other.1{
            panic!("Fields must be same");
        }
        self.0=(self.0*other.inverse().0)%self.1.0;
    }
}
impl Neg for FieldElement{
    type Output =FieldElement;
    fn neg(self)->FieldElement{
        FieldElement((self.1.0-self.0)%self.1.0,self.1)
    }
}