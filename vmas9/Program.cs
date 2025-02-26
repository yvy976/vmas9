using System;
using System.IO;
using System.Collections.Generic;
class Assembler {
    public static void Main(string[] args) {
        Pass1 p = new Pass1(args[0]);


    }   
    
}

public class Pass1 {
    private Dictionary<string, int> _labels;
    private List<string> _instructions;

    private int _label_mem_location;
    private int _instruction_mem_location;

    public Pass1(string filename) {
        _labels = new Dictionary<string, int>{};
        _instructions = new List<string>{};
        _label_mem_location = 0;
        _instruction_mem_location = 0;

        string? line;
        StreamReader sr;
        try {
            sr = new StreamReader(filename);

        } catch (FileNotFoundException e) {
            throw new FileNotFoundException($"{e}");
        }
        while ( (line = sr.ReadLine()) != null){
            line = line.Split("# ")[0].Trim();
            // Console.WriteLine(line);
            if (line.EndsWith(':')) {
                var label = line[0..(line.Length-1)];
                _labels.Add(label, _label_mem_location);
                _label_mem_location += 4;
                
            } else {
                // ITS AN INSTRUCTION
                continue;
            }
        }

    }

    public int MemLocationAtLabel(string label) {
        return _labels[label];
        // return _labels.ContainsKey(label) ? _labels[label] : throw new KeyNotFoundException($"Label '{label}' not found.");
    }

}
