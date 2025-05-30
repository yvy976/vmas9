public class Exit : IInstruction
{
    private readonly int _code;

    public Exit(string[] s)
    {
        _code = s.Length == 2 ? Convert.ToInt32(s[1]) : 0;
    }
    public int Encode()
    {
        return ((1 << 8) - 1) & _code;
    }
}

public class Swap : IInstruction
{
    private readonly int _from;
    private readonly int _to;

    public Swap(string[] s)
    {
        if (s.Length == 3)
        {
            if (s[1].StartsWith("0x"))
            {
                _from = Int32.Parse(s[1][2..], System.Globalization.NumberStyles.HexNumber);
            }
            else
            {
                _from = Convert.ToInt32(s[1]);
            }

            if (s[2].StartsWith("0x"))
            {
                _to = Int32.Parse(s[2][2..], System.Globalization.NumberStyles.HexNumber);
            }
            else
            {
                _to = Convert.ToInt32(s[2]);
            }


        }
        else if (s.Length == 2)
        {
            if (s[1].StartsWith("0x"))
            {
                _from = Int32.Parse(s[1][2..], System.Globalization.NumberStyles.HexNumber);
            }
            else
            {
                _from = Convert.ToInt32(s[1]);
            }
            _to = 0;
        }
        else
        {
            _from = 4;
            _to = 0;
        }

        _from >>= 2;
        _to >>= 2;
        if (_from < 0)
        {
            _from += 1 << 12;
        }
        _from &= 0xFFF;
        if (_to < 0)
        {
            _to += 1 << 12;
        }
        _to &= 0xFFF;

    }
    public int Encode()
    {
        // return (0b1 << 24) | ((((_from >> 2) << 12) | (_to >> 2)) & ((1 << 24) - 1));
        return (0b1 << 24) | (_from << 12) | _to;
    }

}

public class Nop : IInstruction
{
    public int Encode()
    {
        return 0b10 << 24;
    }
}

public class Input : IInstruction
{
    public int Encode()
    {
        return 0b100 << 24;
    }
}

public class StringInput : IInstruction
{
    private readonly uint _size;
    public StringInput(string[] s)
    {
        if (s.Length == 1)
        {
            _size = ((1 << 24) - 1);
        }
        else
        {
            if (s[1].StartsWith("0x"))
            {
                _size = (uint)Int32.Parse(s[1][2..], System.Globalization.NumberStyles.HexNumber);
            }
            else
            {
                _size = ((1 << 24) - 1) & Convert.ToUInt32(s[1]);

            }
        }
    }
    public int Encode()
    {
        return (int)((0b0101 << 24) | (_size & ((1 << 24) - 1)));
    }
}

public class Debug : IInstruction
{
    private readonly int _value;
    public Debug(string[] s)
    {
        if (s.Length == 1)
        {
            _value = 0;
        }
        else
        {
            if (s[1].StartsWith("0x"))
            {
                _value = Int32.Parse(s[1][2..], System.Globalization.NumberStyles.HexNumber);
            }
            else
            {
                _value = Convert.ToInt32(s[1]);
            }

        }
    }
    public int Encode()
    {
        return (0b1111 << 24) | _value;
    }
}

public class Pop : IInstruction
{
    private readonly uint _offset;
    public Pop(string[] s)
    {
        if (s.Length == 2)
        {

            if (s[1].StartsWith("0x"))
            {
                _offset = (uint)Int32.Parse(s[1][2..], System.Globalization.NumberStyles.HexNumber);
            }
            else
            {
                _offset = Convert.ToUInt32(s[1]);
            }
        }
        else
        {
            _offset = 4;
        }

    }
    public int Encode()
    {
        return (int)((1 << 28) | (_offset & ((1 << 26) - 1) << 2));
    }

}

public class Add : IInstruction
{
    public int Encode()
    {
        return 0b10 << 28;
    }
}

public class Sub : IInstruction
{
    public int Encode()
    {
        return 0b100001 << 24;
    }
}

public class Mul : IInstruction
{
    public int Encode()
    {
        return 0b100010 << 24;
    }
}

public class Div : IInstruction
{
    public int Encode()
    {
        return 0b100011 << 24;
    }
}

public class Rem : IInstruction
{
    public int Encode()
    {
        return 0b100100 << 24;
    }
}

public class And : IInstruction
{
    public int Encode()
    {
        return 0b100101 << 24;
    }
}

public class Or : IInstruction
{
    public int Encode()
    {
        return 0b100110 << 24;
    }
}

public class Xor : IInstruction
{
    public int Encode()
    {
        return 0b100111 << 24;
    }
}

public class Lsl : IInstruction
{
    public int Encode()
    {
        return 0b101000 << 24;
    }
}

public class Lsr : IInstruction
{
    public int Encode()
    {
        return 0b101001 << 24;
    }
}

public class Asr : IInstruction
{
    public int Encode()
    {
        return 0b101011 << 24;
    }
}

public class Neg : IInstruction
{
    public int Encode()
    {
        return 0b110000 << 24;
    }
}

public class Not : IInstruction
{
    public int Encode()
    {
        return 0b110001 << 24;
    }
}

public class Stprint : IInstruction
{
    private readonly int _offset;
    public Stprint(string[] s)
    {
        if (s.Length == 1)
        {
            _offset = 0;
        }
        else
        {
            if (s[1].StartsWith("0x"))
            {
                _offset = Int32.Parse(s[1][2..], System.Globalization.NumberStyles.HexNumber);
            }
            else
            {
                _offset = Convert.ToInt32(s[1]);
            }
        }
    }
    public int Encode()
    {
        return (0b100 << 28) | (_offset & ((1 << 28) - 1));
    }
}

public class Call : IInstruction
{
    private readonly int _offset;
    public Call(string[] s, Dictionary<string, int> d, int ln)
    {

        _offset = d[s[1]] - ln;

        _offset *= 4;

    }
    public int Encode()
    {
        return (0b101 << 28) | (_offset & (((1 << 26) - 1) << 2));
    }
}

public class Return : IInstruction
{
    private readonly int _offset;
    public Return(string[] s)
    {
        if (s.Length == 1)
        {
            _offset = 0;
        }
        else
        {
            _offset = Convert.ToInt32(s[1]);
        }
        _offset &= ~3;
    }

    public int Encode()
    {
        return (0b110 << 28) | _offset;
    }
}

public class Goto : IInstruction
{
    private readonly int _offset;
    public Goto(string[] s, int ln, Dictionary<string, int> d)
    {

        _offset = d[s[1]] - ln;

        _offset *= 4;

    }
    public int Encode()
    {
        return (0b111 << 28) | (_offset & ((1 << 28) - 1));
    }
}

public class If : IInstruction
{
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
    public If(string[] s, int ln, Dictionary<string, int> d)
    {
        var cond = s[0].Substring(2, 2);


        if (s[1].StartsWith("0x"))
        {
            _offset = Int32.Parse(s[1][2..], System.Globalization.NumberStyles.HexNumber);
        }
        else
        {
            _offset = int.TryParse(s[1], out int result) ? result : d[s[1]] - ln;
        }

        _offset *= 4;

        _code = IfCodes[cond];
        if (_code >= 8)
        { // unary if
            _code = _code & ~8;
            _opcode = 0b1001;
        }
        else
        { // binary if
            _opcode = 0b1000;
        }

    }
    public int Encode()
    {
        return (_opcode << 28) | (_code << 25) | _offset & ((1 << 24) - 1);
    }
}


public class Dup : IInstruction
{
    private readonly int _offset;
    public Dup(string[] s)
    {
        if (s.Length == 1)
        {
            _offset = 0;
        } else
        {
            if (s[1].StartsWith("0x"))
            {
                _offset = Int32.Parse(s[1][2..], System.Globalization.NumberStyles.HexNumber);
            }
            else
            {
                _offset = Convert.ToInt32(s[1]);
            }
        }
        _offset &= ~3;
    }
    public int Encode()
    {
        return (0b1100 << 28) | _offset & ((1 << 28) - 1);
    }
}

public class Print : IInstruction
{
    private readonly int _offset;
    private readonly int _fmt;
    public Print(string[] s)
    {
        var fmt = s[0].Last();

        switch (fmt)
        {
            case 'h': // hex
                _fmt = 0b1;
                break;
            case 'b': // binary
                _fmt = 0b10;
                break;
            case 'o': // octal
                _fmt = 0b11;
                break;
            default: // dec
                _fmt = 0b0;
                break;
        }
        if (s.Length == 1)
        {
            _offset = 0;
        }
        else
        {
            if (s[1].StartsWith("0x"))
            {
                _offset = Int32.Parse(s[1][2..], System.Globalization.NumberStyles.HexNumber);
            }
            else
            {
                _offset = Convert.ToInt32(s[1]);
            }
        }


    }
    public int Encode()
    {
        return (0b1101 << 28) | ((_offset | _fmt) & ((1 << 28)-1));

    }
}

public class Dump : IInstruction
{
    public int Encode()
    {
        return 0b1110 << 28;
    }
}

public class Push : IInstruction
{
    private readonly int _value;
    private readonly int _type;
    public Push(string[] s, int type, Dictionary<string, int> d)
    {

        if (s.Length == 1)
        {
            _value = 0;
        }
        else
        {
            if (s[1].StartsWith("0x"))
            {
                _value = Int32.Parse(s[1][2..], System.Globalization.NumberStyles.HexNumber);
            } else if (d.ContainsKey(s[1])) {
                _value = d[s[1]];
            }
            else
            {
                _value = Convert.ToInt32(s[1]);
            }
        }
        _type = type;

    }
    public int Encode()
    {
        if (_type == 1)
        {
            return _value;
        }
        return (0b1111 << 28) | _value & ((1 << 28) - 1);
    }
}


public class Stpush
{

    private readonly List<int> _value;
    public Stpush(string value)
    {
        _value = new List<int> { };

        int tmp = 0;
        int counter = 0;
        int index = 0;
        for (int i = 0; i < value.Length; i++)
        {
            tmp |= Convert.ToByte(value[i]) << (counter * 8);
            counter += 1;
            if (counter == 3 || i == value.Length - 1)
            {
                if (i < value.Length - 1)
                {
                    tmp |= 0xF1 << 24;
                }
                else
                {
                    for (int j = counter; j < 3; j++)
                    {
                        tmp |= 0x01 << (j * 8);
                    }
                    tmp |= 0xF0 << 24;
                }

                _value.Add(tmp);
                index += 1;

                tmp = 0;
                counter = 0;
            }

        }
    }


    public List<int> Encode()
    {
        return _value;
    }
}