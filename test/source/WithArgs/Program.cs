using System;
using System.IO;
using System.Reflection;
using System.Threading;
using System.Threading.Tasks;

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

            Console.Error.WriteLine("[!] If I had any errors, they would be here, in `stderr`!");

            Console.WriteLine("[*] Bye!");

            Console.WriteLine();
        }
    }
}