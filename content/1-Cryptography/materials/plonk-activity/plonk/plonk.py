from .poly import gen_poly
from .constraint import is_satisfied
from .copy_constraint import copy_constraint_simple
import pdb


def verify_naieve(Ql, Qr, Qm, Qo, Qc, a, b, c, witness_y, perm_a, perm_b, perm_c):

    # make sure constarints + witness is statisfied
    assert is_satisfied(Ql, Qr, Qm, Qo, Qc, a, b, c) == True

    # original polynomial

    Xcoef = [0, 1]

    # todo: replace with random

    v1 = hash(str(a + b + c))
    v2 = hash(str(c + a + b))

    eval_domain = range(0, len(a) * 3)

    x, Y, Px_a, rlc = copy_constraint_simple(range(0, len(a)), Xcoef, witness_y, v1, v2)
    x, Y, Px_b, rlc = copy_constraint_simple(
        range(len(a), len(a) * 2), Xcoef, witness_y, v1, v2
    )
    x, Y, Px_c, rlc = copy_constraint_simple(
        range(len(a) * 2, len(a) * 3), Xcoef, witness_y, v1, v2
    )

    # calcualte permutated polynomial
    x_1, Y_1, Px_a_prime, rlc_1 = copy_constraint_simple(
        range(0, len(a)), perm_a, witness_y, v1, v2
    )
    x_1, Y_1, Px_b_prime, rlc_1 = copy_constraint_simple(
        range(len(a), len(a) * 2), perm_b, witness_y, v1, v2
    )
    x_1, Y_1, Px_c_prime, rlc_1 = copy_constraint_simple(
        range(len(a) * 2, len(a) * 3), perm_c, witness_y, v1, v2
    )

    assert (
        Px_a[-1] * Px_b[-1] * Px_c[-1]
        == Px_a_prime[-1] * Px_b_prime[-1] * Px_c_prime[-1]
    )
    assert (
        Px_a[0]
        == Px_b[0]
        == Px_c[0]
        == Px_a_prime[0]
        == Px_b_prime[0]
        == Px_c_prime[0]
        == 1
    )
    # todo public input check


def create_proof(a, b, c):

    poly = gen_poly(a + b + c)

    return (a, b, c, poly)
