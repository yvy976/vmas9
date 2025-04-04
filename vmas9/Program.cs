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
        Pass1 p = new Pass1(args[0], args[1]);
    }
}

public class Pass1
{
    //Dictionary keyed on label name, values at int[] index [1] and [2] are relative and absolute offset respectively. Value at [0]
    //is 0 by default.
    public Dictionary<string, int> _labels;

    //List containing the entire line contents of each instruction
    private List<string[]> _instrLine;
            
    //List of IInstruction class instances which will need their encode method called
    private List<IInstruction> _instructions;


    public Pass1(string filename, string out_filename)
    {
        //Initializing fields when constructor is called
        _labels = new Dictionary<string, int> { };
        _instructions = new List<IInstruction> { };
        _instrLine = new List<string[]> { };
        int lineNumber = 0;

        // String for each line of the file to be read into
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

        //Read and parse .asm file, skipping comment and whitespace lines, and recording proper memory location of each label
        while ((line = sr.ReadLine()) != null)
        {
            bool alreadyAdd = false;
            if (line.Trim().StartsWith("#") || string.IsNullOrWhiteSpace(line)) continue;

            //Trim comments when they occur on the same line as label or instruction
            line = line.Split("#", 2, StringSplitOptions.None)[0].Trim();       
            
            //var ins = line.Split(" ", 1, StringSplitOptions.RemoveEmptyEntries)[0];

            //Recording proper label info
            if (line.Trim().EndsWith(':'))
            {
                var label = line[0..(line.Length - 1)];     //parse label to remove ':'
                _labels.Add(label, lineNumber);
                _instrLine.Add(line.Split(" ", StringSplitOptions.RemoveEmptyEntries));
                continue;
            }

            //Recording instruction info and adding the instruction to its list
            else
            {
                //parsing instruction to remove additional whitespace. instr[0] will be the instruction title, instr[1] will be its number or string parameter.
                var instr = line.Split(" ", StringSplitOptions.RemoveEmptyEntries);  
                
                //If the instruction is stpush pseudoinstruction, handle correct number of instructions it would add when pushing the associated
                //string
                if (instr[0] == "stpush")
                {
                    //Parse the string being pushed, trimming the " " marks
                    var entire_stpush_line_contents = line.Split(" ", 2, StringSplitOptions.RemoveEmptyEntries);
                    string str_portion_stpush = entire_stpush_line_contents[1][1..(entire_stpush_line_contents[1].Length-1)];

                    //Handle the allowed escapes appropriately
                    str_portion_stpush = str_portion_stpush.Replace("\\\"", "`");
                    str_portion_stpush = str_portion_stpush.Replace("\"", "");
                    str_portion_stpush = str_portion_stpush.Replace("`", "\"");
                    str_portion_stpush = str_portion_stpush.Replace("\\n", "\n");
                    str_portion_stpush = str_portion_stpush.Replace("\\\\", "\\");
                    
                    //Calculate number of instructions the stpush will add and increment associated line number appropriately
                    var num_instr_to_add = (int)Math.Ceiling(str_portion_stpush.Length / 3.0);
                    lineNumber += num_instr_to_add;
                    alreadyAdd = true;
                    instr[1] = str_portion_stpush;
                }

                _instrLine.Add(instr);

                //Increment line number for the added instruction. Instructions added by stpush are handled seperately and do not get 
                //incremented again here
                if (!alreadyAdd) lineNumber++;  
            }
        }
        sr.Close();

        int prog_counter = 0;
       

        //Loop through all the instruction lines and add a new instance of the associated instruction class to IInstruction list while keeping track
        //of the program counter to handle offsets correctly.
        for (int s = 0; s < _instrLine.Count; s++)
        {
            int pc_increment_value = 1;
            switch (_instrLine[s][0])
            {
                case "exit":
                    _instructions.Add(new Exit(_instrLine[s]));
                    break;
                case "swap":
                    _instructions.Add(new Swap(_instrLine[s]));
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
                case "and":
                    _instructions.Add(new And());
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
                    _instructions.Add(new Call(_instrLine[s], _labels, prog_counter));
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
                    break;
                case "dup":
                    _instructions.Add(new Dup(_instrLine[s]));
                    break;
                case "print":
                case "printo":
                case "printh":
                case "printb":
                    _instructions.Add(new Print(_instrLine[s]));
                    break;
                case "dump":
                    _instructions.Add(new Dump());
                    break;
                case "push":
                    _instructions.Add(new Push(_instrLine[s], 0, _labels));
                    break;
                case "nop":
                    _instructions.Add(new Nop());
                    break;
                //Handle additional stpush instructions and program counter appropriately, taking endian-ness into account
                case "stpush":
                    Stpush p = new Stpush(_instrLine[s][1]);
                    var additional_stpush_ins = p.Encode();
                    pc_increment_value = additional_stpush_ins.Count;
                    for (int i = additional_stpush_ins.Count - 1; i >= 0; i -= 1)
                    {
                        string[] k = new string[2];
                        k[1] = additional_stpush_ins[i].ToString();
                        _instructions.Add(new Push(k, 1, _labels));
                    }
                    break;
                default:
                    pc_increment_value = 0;
                    break;

            }
     
            prog_counter += pc_increment_value;
        }

        //Add Nop to end of _instructions to pad out multiple of 4 instructions appropriately
        for (int i = _instructions.Count; i < (_instructions.Count + 3 & -4); i++) _instructions.Add(new Nop());


        //Throw exception if there are no instructions in the provided file
        if (_instructions.Count == 0){
            try{
                throw new Exception($"Invalid file <{filename}>: No instructions to encode.");
            }
            catch (Exception e){
                Console.WriteLine(e);
            }
        }
        
        //Use BinaryWriter to write the encoded instructions to output file specified from command line
        BinaryWriter br = new BinaryWriter(File.Open(out_filename, FileMode.Create));

        //First 4 bytes will always be deadbeef
        br.Write(0xefbeadde);
        
        //Loop through list of instruction class instances, convert encoded values to binary, and write to file in little endian
        foreach (var v in _instructions)
        {
            //Convert encoded value to binary string and pad to 32 bits
            String e = Convert.ToString(v.Encode(), 2).PadLeft(32, '0');

            //Writing the encoded bytes in little endian
            List<byte> bytes = new List<byte>();
            for (int i = 0; i < 32; i += 8)
            {
                string b = e.Substring(i, 8);
                bytes.Add(Convert.ToByte(b, 2));
            }
            bytes.Reverse();
            br.Write(bytes.ToArray());
        }
        br.Close();
    }
}
