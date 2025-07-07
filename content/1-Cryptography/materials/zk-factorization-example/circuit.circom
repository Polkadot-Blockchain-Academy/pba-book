pragma circom 2.0.0;
/*This circuit template checks that r * s is a non-trivial interger factorization of c  */

/*
 * check if the input is not equal to 1
 */
template NotEqualToOne() {
    signal input in;
    signal output out;

    signal inv;

    inv <-- 1/(in - 1);

    1 === (in - 1)*inv;
}

/*
 * Decomposes `in` into `b` bits, given by `bits`.
 * Least significant bit in `bits[0]`.
 * Enforces that `in` is at most `b` bits long.
 */
template Num2Bits(b) {
    signal input in;
    signal output bits[b];

    //log("numbit bits:", b);
    for (var i = 0; i < b; i++) {
        bits[i] <-- (in >> i) & 1;
        bits[i] * (1 - bits[i]) === 0;
    }
    var sum_of_bits = 0;
    for (var i = 0; i < b; i++) {
        sum_of_bits += (2 ** i) * bits[i];
    }
    sum_of_bits === in;
}

template CheckFactorization () {  

   // Declaration of signals.  
   signal input r;  
   signal input s;  
   
   component r_is_not_1;
   component s_is_not_1;
   component r_integer_less_than_8;
   component s_integer_less_than_8;

   // Constraints.  
   21 === r * s;

   r_is_not_1 = NotEqualToOne();
   r_is_not_1.in <== r;

   r_integer_less_than_8 = Num2Bits(3);
   r_integer_less_than_8.in <== r;

   s_is_not_1 = NotEqualToOne();
   s_is_not_1.in <== s;

   s_integer_less_than_8 = Num2Bits(3);
   s_integer_less_than_8.in <== s;

}

component main = CheckFactorization();
