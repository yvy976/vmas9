using System;
using System.IO;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics.CodeAnalysis;
using System.Reflection.Emit;
class Assembler
{
    public static void Main(string[] args)
    {
        Pass1 p = new Pass1(args[0]);


    }

}

public class Pass1
{
    public Dictionary<string, int[]> _labels;

    private List<string[]> _instrLine;
    private List<IInstruction> _instructions;

    private int _label_mem_location;
    private int _instruction_mem_location;

    public Pass1(string filename)
    {
        _labels = new Dictionary<string, int[]> { };
        _instructions = new List<IInstruction> { };
        _instrLine = new List<string[]> { };
        _label_mem_location = 0;
        _instruction_mem_location = 0;
        int lineNumber = 1;
        string? line;
        StreamReader sr;
        try
        {
            sr = new StreamReader(filename);

        }
        catch (FileNotFoundException e)
        {
            throw new FileNotFoundException($"{e}");
        }
        while ((line = sr.ReadLine()) != null)
        {
            bool alreadyAdd = false;
            if (line.Trim().StartsWith("#") || string.IsNullOrWhiteSpace(line)) continue;
            line = line.Split("#", 2, StringSplitOptions.None)[0].Trim();

            var ins = line.Split(" ", 1, StringSplitOptions.RemoveEmptyEntries)[0];
            if (line.Trim().EndsWith(':'))
            {
                var label = line[0..(line.Length - 1)];
                _labels.Add(label, new int[3]);
                _labels[label][0] = 0;
                _labels[label][1] = lineNumber;
                _labels[label][2] = lineNumber;
                _label_mem_location += 4;
                _instrLine.Add(line.Split(" ", StringSplitOptions.RemoveEmptyEntries));

            }
            else
            {
                var x = line.Split(" ", StringSplitOptions.RemoveEmptyEntries);
                _instrLine.Add(x);
                if (x[0] == "stpush")
                {
                    var _tmp = x.Skip(1).ToArray();
                    string value = String.Join(" ", _tmp);

                    value = value.Replace("\\\"", "`");
                    value = value.Replace("\"", "");
                    value = value.Replace("`", "\"");

                    value = value.Replace("\\n", "\n");
                    value = value.Replace("\\\\", "\\");
                    lineNumber += (int)Math.Ceiling((double)(value.Length / 3));
                    alreadyAdd = true;
                }
            if (!alreadyAdd)      lineNumber++;

            }
        }
        sr.Close();


        int prog_counter = 0;
        lineNumber = 0;
        // foreach (var s in _instrLine)
        for (int s = 0; s < _instrLine.Count; s++)
        {

            {
                int sub = 0;
                bool isLabel = false;
                // Console.WriteLine(_instrLine[s][0]);
                switch (_instrLine[s][0])
                {

                    case "exit":
                        _instructions.Add(new Exit(_instrLine[s]));
                        break;
                    case "swap":
                        _instructions.Add(new Swap(_instrLine[s]));
                        Console.WriteLine($"{prog_counter} {s}");
                        break;
                    case "input":
                        _instructions.Add(new Input());
                        break;
                    case "stinput":
                        _instructions.Add(new StringInput(_instrLine[s]));
                        break;
                    case "debug":
                        _instructions.Add(new Debug(_instrLine[s]));
                        break;
                    case "pop":
                        _instructions.Add(new Pop(_instrLine[s]));
                        break;
                    case "add":
                        _instructions.Add(new Add());
                        break;
                    case "sub":
                        _instructions.Add(new Sub());
                        break;
                    case "mul":
                        _instructions.Add(new Mul());
                        break;
                    case "div":
                        _instructions.Add(new Div());
                        break;
                    case "rem":
                        _instructions.Add(new Rem());
                        break;
                    case "or":
                        _instructions.Add(new Or());
                        break;
                    case "xor":
                        _instructions.Add(new Xor());
                        break;
                    case "lsl":
                        _instructions.Add(new Lsl());
                        break;
                    case "lsr":
                        _instructions.Add(new Lsr());
                        break;
                    case "asr":
                        _instructions.Add(new Asr());
                        break;
                    case "neg":
                        _instructions.Add(new Neg());
                        break;
                    case "not":
                        _instructions.Add(new Not());
                        break;
                    case "stprint":
                        _instructions.Add(new Stprint(_instrLine[s]));
                        break;
                    case "call":
                        _instructions.Add(new Call(_instrLine[s], _labels));
                        break;
                    case "return":
                        _instructions.Add(new Return(_instrLine[s]));
                        break;
                    case "goto":
                        _instructions.Add(new Goto(_instrLine[s], prog_counter, _labels));
                        break;

                    case "ifeq":
                    case "ifne":
                    case "iflt":
                    case "ifgt":
                    case "ifle":
                    case "ifge":
                    case "ifez":
                    case "ifnz":
                    case "ifmi":
                    case "ifpl":
                        _instructions.Add(new If(_instrLine[s], prog_counter, _labels));
                        Console.WriteLine(prog_counter);
                        // Console.WriteLine(_labels["NotEqual20"][1]);
                        break;
                    case "dup":
                        _instructions.Add(new Dup(_instrLine[s]));
                        break;
                    case "print":
                        _instructions.Add(new Print(_instrLine[s]));
                        break;
                    case "dump":
                        _instructions.Add(new Dump());
                        break;
                    case "push":


                        _instructions.Add(new Push(_instrLine[s], 0));
                        break;
                    case "stpush":
                        // _instructions.Add(new Stpush(s, 0));
                        // string value = s[1].Trim().Split(" ", 2, StringSplitOptions.RemoveEmptyEntries)[1];

                        var _tmp = _instrLine[s].Skip(1).ToArray();
                        string value = String.Join(" ", _tmp);
                        // value += " ";
                        // Console.WriteLine(_tmp.ToString());



                        value = value.Replace("\\\"", "`");
                        value = value.Replace("\"", "");
                        value = value.Replace("`", "\"");

                        value = value.Replace("\\n", "\n");
                        value = value.Replace("\\\\", "\\");
                        Stpush p = new Stpush(value);

                        var pp = p.Encode();
                        sub = pp.Count;
                        for (int i = pp.Count - 1; i >= 0; i -= 1)
                        {
                            string[] k = new string[2];
                            k[1] = pp[i].ToString();

                            _instructions.Add(new Push(k, 1));
                        }


                        break;
                    default:
                        isLabel = true;
                        // _labels[_instrLine[s][0]] = prog_counter;
                        break;


                }
                lineNumber++;

            foreach (var key in _labels.Keys.ToList())  // Convert Keys to a List to avoid modification issues
            {

                _labels[key][1] = _labels[key][2] - (prog_counter);
            if (_instrLine[s][0] == "call")Console.WriteLine($"{key} k0={_labels[key][0]} k1={_labels[key][1]} k2={_labels[key][2]} {s} {_instrLine[s][0]} progcounter= {prog_counter} reloffset = {_labels[key][2] - prog_counter}");

            }
                prog_counter += sub == 0 ? 1 : sub;



            }
        }




        for (int i = _instructions.Count; i < (_instructions.Count + 3 & -4); i++) _instructions.Add(new Nop());
        
        Console.WriteLine($"len of mem {_instructions.Count}");
        BinaryWriter br = new BinaryWriter(File.Open("h.v", FileMode.Create));

        br.Write(0xefbeadde);
        foreach (var v in _instructions)
        {
            // Console.WriteLine(v.ToString());

            String e = Convert.ToString(v.Encode(), 2).PadLeft(32, '0');



            // Console.WriteLine($"{v.Encode()}  {e}");
            List<byte> bytes = new List<byte>();
            for (int i = 0; i < 32; i += 8)
            {
                string b = e.Substring(i, 8);
                bytes.Add(Convert.ToByte(b, 2));
            }
            bytes.Reverse();
            br.Write(bytes.ToArray());

            // foreach (var p in e)
            // {
            //     // Console.WriteLine(p);
            //     br.Write($"{p.ToString()}");
            // }
        }
        br.Close();

    }

    private StreamReader StreamReader(string filename)
    {
        throw new NotImplementedException();
    }

    // public int MemLocationAtLabel(string label)
    // {
    //     return _labels[label];
    //     // return _labels.ContainsKey(label) ? _labels[label] : throw new KeyNotFoundException($"Label '{label}' not found.");
    // }

}
