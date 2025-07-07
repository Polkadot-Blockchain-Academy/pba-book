from .poly import lagrange, polynomial_eval


def copy_constraint_simple(eval_domain, Xcoef, Ycoef, v1, v2):
    Px = [1]
    Y = []
    rlc = []
    x = []

    for i in range(0, len(eval_domain)):
        x.append(polynomial_eval(Xcoef, eval_domain[i]))
        Y.append(polynomial_eval(Ycoef, x[i]))

        rlc.append(v1 + x[i] + v2 * Y[i])
        Px.append(Px[i] * (v1 + x[i] + v2 * Y[i]))

    return (x, Y, Px, rlc)


def find_permutation(copies, eval_domain):

    perm = lagrange(eval_domain, copies)
    perm = [float(x) for x in reversed(perm.coefficients)]
    return perm
