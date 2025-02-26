
public class Exit : IInstruction {
    private readonly int _code;

    public Exit(int code = 0) {
        _code = code;
    }
    public int Encode() {
        return ((1 << 8) - 1) & _code;
    }
}

public class Swap : IInstruction {
    private readonly int _from;
    private readonly int _to;

    public Swap(int from = 4, int to = 0) {
        _from = from << 12;
        _to = to;
    }
    public int Encode() {
        return (0b1 << 24 ) | _from | _to;
    }
}

public class Nop : IInstruction {
    public int Encode() {
        return 0b10 << 24;
    }
}

public class Input : IInstruction {
    public int Encode() {
        return 0b100 << 24;
    }
}

public class StringInput : IInstruction {
    private readonly uint _size;
    public StringInput(uint size = (1 << 24) - 1) {
        _size = ((1 << 24) - 1) & size;
    }
    public int Encode() {
        return (int) ((0101 << 24) | _size);
    }
}

public class Debug : IInstruction {
    private readonly int _value;
    public Debug(int value) {
        _value = value;
    }
    public int Encode() {
        return (0b1111 << 24) | _value;
    }
}

public class Pop : IInstruction {
    private readonly uint _offset;
    public Pop(uint offset = 4) {
        _offset = (uint)(offset & ~3);
    }
    public int Encode() {
        return (int) ((0b1 << 28) | _offset);
    }
}

public class Add : IInstruction {
    public int Encode() {
        return 0b10 << 28;
    }
}

public class Sub : IInstruction {
    public int Encode() {
        return 0b100001 << 24;
    }
}

public class Mul : IInstruction {
    public int Encode() {
        return 0b100010 << 24;
    }
}

public class Div : IInstruction {
    public int Encode() {
        return 0b100011 << 24;
    }
}

public class Rem : IInstruction {
    public int Encode() {
        return 0b100100 << 24;
    }
}

public class And : IInstruction {
    public int Encode() {
        return 0b100101 << 24;
    }
}

public class Or : IInstruction {
    public int Encode() {
        return 0b100110 << 24;
    }
}

public class Xor : IInstruction {
    public int Encode() {
        return 0b100111 << 24;
    }
}

public class Lsl : IInstruction {
    public int Encode() {
        return 0b101000 << 24;
    }
}

public class Lsr : IInstruction {
    public int Encode() {
        return 0b101001 << 24;
    }
}

public class Asr : IInstruction {
    public int Encode() {
        return 0b101011 << 24;
    }
}

public class Neg : IInstruction {
    public int Encode() {
        return 0b110000 << 24;
    }
}

public class Not : IInstruction {
    public int Encode() {
        return 0b110001 << 24;
    }
}

public class Stprint : IInstruction {
    private readonly int _offset;
    public Stprint(int offset = 0) {
        _offset = offset;
    } 
    public int Encode() {
        return (0b100 << 28) | _offset;
    }
}

public class Call : IInstruction {
    private readonly int _offset;
    public Call(string label, Pass1 pass1) {
        _offset = pass1.MemLocationAtLabel(label) & ~3;
    }
    public int Encode() {
        return (0b101 << 28) | _offset;
    }
}

public class Return : IInstruction {
    private readonly int _offset;
    public Return(int offset) {
        _offset = offset & ~3;
    }
    public int Encode() {
        return (0b110 << 28) | _offset;
    }
}

public class Goto : IInstruction {
    private readonly int _offset; 
    public Goto(string label, Pass1 pass1) {
        _offset = pass1.MemLocationAtLabel(label);
    }
    public int Encode() {
        return (0b111 << 28) | _offset;
    }
}

public class If : IInstruction {
    private readonly int _opcode;
    private readonly int _code;
    private readonly int _offset; 
    // private readonly bool binary;
    private Dictionary<string, int> IfCodes = new Dictionary<string, int> {
        {"eq", 0b0},
        {"ne", 0b1},
        {"lt", 0b10},
        {"gt", 0b11},
        {"le", 0b100},
        {"ge", 0b101},
        {"ez", 0b1000},
        {"nz", 0b1001},
        {"mi", 0b1010},
        {"pl", 0b1011},
    };
    public If(string cond, Pass1 pass1, int offset = 0, string label = "") {
        _code = IfCodes[cond];
        if (_code >= 8) { // unary if
            _code = _code & ~8;
            _opcode = 0b1001;
            // binary = false;
        } else { // binary if
            _opcode = 0b1000;
            // binary = true;
        }
        _offset = label == string.Empty ? offset : pass1.MemLocationAtLabel(label);
        // sigh extend??
    }
    public int Encode() {
        return (_opcode << 28) | (_opcode << 25) | _offset;
    }
}

public class Dup : IInstruction {
    private readonly int _offset;
    public Dup(int offset) {
        _offset = offset & ~3;
    }
    public int Encode() {
        return (0b1100 << 28) | _offset;
    }
}

public class Print : IInstruction {
    private readonly int _offset;
    private readonly int _fmt;
    private Dictionary<string, int> Format = new Dictionary<string, int> {
        {"", 0b0},
        {"h", 0b1},
        {"b", 0b10},
        {"o", 0b11},
    };
    public Print(int offset = 0, int fmt = 0) {
        _offset = offset;
        switch (fmt) {
            case 1: // hex
                _fmt = 0b1;
                break;
            case 2: // binary
                _fmt = 0b10;
                break;
            case 3: // octal
                _fmt = 0b11;
                break;
            default: // dec
                _fmt = 0b0;
                break;
        }
    } 
    public int Encode() {
        return (0b1101 << 28) | (_offset << 2) | _fmt;
    }
}

public class Dump : IInstruction {
    public int Encode() {
        return 0b1110 << 28;
    }
}

public class Push : IInstruction {
    private readonly int _value;
    public Push(int value) {
        _value = value;
    }
    public int Encode() {
        return (0b1111 << 28) | _value;
    }
}

