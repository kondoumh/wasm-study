using System;
using Wasmtime;

namespace Tutorial
{
    class Program
    {
        static void Main(string[] args)
        {
            using var engine = new Engine();
            using var module = Module.FromTextFile(engine, "gcd.wat");

            using var host = new Host(engine);
            using dynamic instance = host.Instantiate(module);

            Console.WriteLine($"gcd(27, 6) = {instance.gcd(27, 6)}");
        }
    }
}
