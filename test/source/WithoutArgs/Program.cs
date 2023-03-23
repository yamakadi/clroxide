using System;
using System.IO;
using System.Reflection;

namespace WithoutArgs
{
    internal static class Program
    {
        public static void Main()
        {
            Console.WriteLine($"[*] Hello World from `{Assembly.GetExecutingAssembly().FullName}`!");
            Console.WriteLine("[*] I have no arguments and live a happy life!");
            Console.WriteLine("[*] Bye!");

            Console.WriteLine();
        }
    }
}