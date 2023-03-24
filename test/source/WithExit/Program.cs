using System;
using System.Reflection;

namespace WithExit
{
    internal class Program
    {
        public static void Main(string[] args)
        {
            Console.WriteLine($"[*] Hello World from `{Assembly.GetExecutingAssembly().FullName}`!");
            Console.WriteLine($"[*] I am running in `{AppDomain.CurrentDomain}`!");
            Console.WriteLine("[*] I very much hope to live past `System.Environment.Exit`!");
            Console.WriteLine();

            Environment.Exit(-1);
            
            Console.WriteLine("[!] I am still alive!");
            Console.WriteLine();
        }
    }
}