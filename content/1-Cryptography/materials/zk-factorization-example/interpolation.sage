from hashlib import sha256

circom_p = 21888242871839275222246405745257275088548364400416034343698204186575808495617
our_p = 89
ZKField = FiniteField(our_p)
#59 | 71^10-1

n = 21

q_l_values = [ 0,  1,  1, -1, -1, -1,  0,  1,  1, -1, -1, -1,  0 ]
q_r_values = [ 0,  2,  4,  0,  0,  0, -1,  2,  4,  0,  0,  0, -1 ]
q_o_values = [ 0, -1, -1,  0,  0,  0,  0, -1, -1,  0,  0,  0,  0 ]
q_m_values = [ 1,  0,  0,  1,  1,  1,  1,  0,  0,  1,  1,  1,  1 ]
q_c_values = [-n,  0,  0,  0,  0,  0, -1,  0,  0,  0,  0,  0, -1 ]

r = 7
s = 3


def compute_input_output_poly_values(secret_factor):
   r0 = secret_factor % 2
   r1 = (secret_factor // 2) % 2
   r2 = (secret_factor // 4) % 2
   
   r01 = r0 + r1*2

   secret_factor_minus_1_inverse = ZKField(secret_factor - 1)^(-1)
   
   return ([r0, r01, r0, r1, r2, secret_factor], [r1, r2, r0, r1, r2, secret_factor_minus_1_inverse] , [r01, secret_factor, 0,0,0,0] )

r_input_output_values = compute_input_output_poly_values(r)
s_input_output_values = compute_input_output_poly_values(s)
left_input_values = [r] + r_input_output_values[0] + s_input_output_values[0]
right_input_values = [s] + r_input_output_values[1] + s_input_output_values[1]

c_output_values = [0] + r_input_output_values[2] + s_input_output_values[2]

x_values = range(1, 14)

PolysOnZKField.<x> = PolynomialRing(ZKField)

Qlx = PolysOnZKField.lagrange_polynomial(zip(x_values,q_l_values))
Qrx = PolysOnZKField.lagrange_polynomial(zip(x_values,q_r_values))
Qox = PolysOnZKField.lagrange_polynomial(zip(x_values,q_o_values))
Qmx = PolysOnZKField.lagrange_polynomial(zip(x_values,q_m_values))
Qcx = PolysOnZKField.lagrange_polynomial(zip(x_values,q_c_values))

ax  = PolysOnZKField.lagrange_polynomial(zip(x_values,left_input_values))
bx  = PolysOnZKField.lagrange_polynomial(zip(x_values,right_input_values))
cx  = PolysOnZKField.lagrange_polynomial(zip(x_values,c_output_values))

#Q_l(x)*a(x) + Q_r(x)*b(x) + Q_o(x)* c(x) + Q_m(x)*a(x)*b(x) + Q_c(x) = 0
gate_poly =  Qlx * ax + Qrx*bx+ Qox * cx + Qmx*ax*bx + Qcx  

zero_test_poly = prod(list(map(lambda x_val: x - x_val,x_values)))

print("All Gate polynomial: ", gate_poly)
print("Zero test polynomial:", zero_test_poly)
print("The remainder of gatepoly/zerotest: ",gate_poly % zero_test_poly)

qx = gate_poly / zero_test_poly
print("q(x):",qx)

# bulid the trace polynomial T
T = [0]*39
for i in range(0,13):
   T[i*3] = left_input_values[i]
   T[i*3 + 1 ] = right_input_values[i]
   T[i*3 + 2 ] = c_output_values[i]

psi = [ 9, 27,  3,
        10, 13, 7,
        6, 16, 19,
        11, 4, 12,
        14, 5, 15,
        17,8,18,
        1,20, 21,
        28, 31, 25,
        24, 34, 37,
        29, 22, 30,
        32, 23, 33,
        35, 26, 36,
        2, 38, 39,
        ]

#check the permutation 
for i in range(0,39):
   if (T[i] != T[psi[i]-1]):
      print("T's wiring doesn't match at index ", i)

#naive wire test
x_values = range(1, 40)

Tx = PolysOnZKField.lagrange_polynomial(zip(x_values, T))
psix = PolysOnZKField.lagrange_polynomial(zip(x_values, psi))

zero_test_poly = prod(list(map(lambda x_val: x - x_val,x_values)))

naive_wire_poly = Tx(psix(x)) - Tx(x)

print("Naive wire  polynomial: ", naive_wire_poly)
print("Zero test polynomial:", zero_test_poly)
print("The remainder of wirepoly/zerotest: ",naive_wire_poly % zero_test_poly)

u1 = ZKField('0x'+hashlib.sha256("Hello, PBA! I'm u1".encode()).hexdigest())
u2 = ZKField('0x'+hashlib.sha256("Hello, PBA! I'm u2".encode()).hexdigest())

#$\prod_{a\in\{1,..,39\}}\frac{u_1 - u_1 \times a - T(a)}{u_1 - u_2 \times \psi(a) - T(\psi(a))}
f_numerator = [0]*40
g_denominator = [0]*40
for i in range(0, 39):
   f_numerator[i] = u1 - u2 * psi[i] - T[psi[i]-1]
   g_denominator[i] = u1 - u2 * (i + 1) - T[i]

#because our group is not cyclic of size 39 we need to deal with t(40)
f_numerator[39] = f_numerator[0] 
g_denominator[39] = g_denominator[0]

for i in range(0, 39):
   if (f_numerator[i] != g_denominator[psi[i]-1]):
      print("perm wiring doesn't match at index ", i)

p_acc = ZKField(1)
for i in range(0, 39):
   p_acc *= f_numerator[i]/g_denominator[psi[i]-1]

assert(p_acc == 1)

t = [0]*40
t[0] = f_numerator[0]/g_denominator[0]
for i in range(1,40):
   t[i] = t[i-1] * f_numerator[i]/g_denominator[i]

x_values = range(1, 41)
fx = PolysOnZKField.lagrange_polynomial(zip(x_values,f_numerator))
gx = PolysOnZKField.lagrange_polynomial(zip(x_values,g_denominator))
tx = PolysOnZKField.lagrange_polynomial(zip(x_values,t))

perm_poly = tx(x+1) * gx(x+1) - tx(x) * fx(x+1)
for i in range(1,40):
   assert(perm_poly(i) == 0)

print("Perm check polynomial: ", perm_poly)
print("Zero test polynomial:", zero_test_poly)
print("The remainder of permpoly/zerotest: ", perm_poly % zero_test_poly)

qx = perm_poly / zero_test_poly
print("q(x):",qx)

# y^2 = x^3 + 8*x + 10 embeding degree 8 on 101 order 89
# (trace_poly(r) - trace_poly(tau)) / (r - tau)

