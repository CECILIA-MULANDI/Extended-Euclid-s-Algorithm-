fn main() {
    let a=36;
    let b=60;
    let (gcd, x, y) = extended_euclidean(a, b);
    println!("GCD: {}", gcd);
    println!("x: {}", x);
    println!("y: {}", y);
}


fn extended_euclidean(a:i32,b:i32)->(i32,i32,i32){
    if b==0{
        (a,1,0)
    }else{
        let (gcd,x1,y1)=extended_euclidean(b,a%b);
        let x=y1;
        let y=x1-(a/b)*y1;
        (gcd,x,y)
    }
}
