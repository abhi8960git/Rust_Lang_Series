//A Fibonacci number is a number in a series where each number is the sum of the two preceding ones, usually starting with 0 and 1. The sequence begins \(0,1,1,2,3,5,8,13,21,\dots \). The formula for the sequence is \(F_{n}=F_{n-1}+F_{n-2}\), with \(F_{0}=0\) and \(F_{1}=1\).                


fn fibbonachi(n:u32)->u32{
    match n {
        0 => return 0,
        1 => return 1,
        _ => return (n-1) + (n-2),
    }
}

fn main(){
    println!("Fibbonachi Numbers");

}

#[cfg(test)]

mod tests{
    use super::*;
    
    #[test]
    fn Zero_fibbonachi(){
        assert_eq!(fibbonachi(0),0);
    }
    #[test]
    fn One_fibbonachi(){
        assert_eq!(fibbonachi(1),1);
    }
    #[test] 
    fn Other_Fibbonachi(){
        assert_eq!(fibbonachi(5),7);
    }

}