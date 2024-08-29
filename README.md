# Extended-Euclid-s-Algorithm
This Rust code demonstrates the Extended Euclidean Algorithm, which calculates the greatest common divisor (GCD) of two integers and finds the coefficients x and y that satisfy:

GCD(a,b)=a⋅x+b⋅y

The extended_euclidean function is called recursively to compute the GCD and coefficients. 
It uses the base case when b is 0, returning (a, 1, 0).
 For other values, it recursively calculates the results for (b, a % b) and then updates the coefficients accordingly.
