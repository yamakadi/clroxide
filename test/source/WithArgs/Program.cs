using System;
using System.IO;
using System.Reflection;

namespace WithArgs
{
  internal static class Program
  {
        public static void Main(string[] args)
        {
            Console.WriteLine($"[*] Hello World from `{Assembly.GetExecutingAssembly().FullName}`!");
            Console.WriteLine($"[*] I am running in `{AppDomain.CurrentDomain}`!");
            Console.WriteLine($"[*] I was given {args.Length} arguments");
            
            foreach (var s in args)
            {
                Console.WriteLine($"___ ({s.Length}) {s}");
            }

            Console.WriteLine();
        }
    }
}