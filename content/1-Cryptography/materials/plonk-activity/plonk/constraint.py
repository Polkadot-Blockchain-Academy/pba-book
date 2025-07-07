def constraint_polynomial(Qli, Qri, Qmi, Qoi, Qci, ai, bi, ci):
    return Qli * ai + Qri * bi + Qoi * ci + Qmi * ai * bi + Qci == 0


def is_satisfied(Ql, Qr, Qm, Qo, Qc, a, b, c):
    for Qli, Qri, Qmi, Qoi, Qci, ai, bi, ci in zip(Ql, Qr, Qm, Qo, Qc, a, b, c):
        if constraint_polynomial(Qli, Qri, Qmi, Qoi, Qci, ai, bi, ci) == False:
            return False
    return True


def add_add_constarint(Ql, Qr, Qm, Qo, Qc):
    Ql.append(1)
    Qr.append(1)
    Qm.append(0)
    Qo.append(-1)
    Qc.append(0)
    return (Ql, Qr, Qm, Qo, Qc)


def add_mul_constarint(Ql, Qr, Qm, Qo, Qc):

    Ql.append(0)
    Qr.append(0)
    Qm.append(1)
    Qo.append(-1)
    Qc.append(0)

    return (Ql, Qr, Qm, Qo, Qc)


def add_constant_constraint(Ql, Qr, Qm, Qo, Qc, const):

    Ql.append(0)
    Qr.append(0)
    Qm.append(1)
    Qo.append(0)
    Qc.append(-const)

    return (Ql, Qr, Qm, Qo, Qc)
