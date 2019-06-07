import q1tsim

nr_qbits = 2
nr_cbits = 5
nr_shots = 1024

with q1tsim.Circuit(nr_qbits, nr_cbits) as circuit:
    circuit.rx(1.23, 0)
    circuit.peek(0, 0)
    circuit.peek(0, 1)
    circuit.execute(nr_shots)
    print(circuit.histogram())
    print(circuit.latex())
    print(circuit.open_qasm())
    print(circuit.c_qasm())
