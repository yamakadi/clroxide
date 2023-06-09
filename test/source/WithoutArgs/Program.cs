﻿using System;
using System.IO;
using System.Reflection;

namespace WithoutArgs
{
    internal static class Program
    {
        public static void Main()
        {
            Console.WriteLine($"[*] Hello World from `{Assembly.GetExecutingAssembly().FullName}`!");
            Console.WriteLine($"[*] I am running in `{AppDomain.CurrentDomain}`!");
            Console.WriteLine("[*] I have no arguments and live a happy life!");
            Console.Error.WriteLine("[!] But if I didn't live a happy life, all my error would be here, in `stderr`!");

            Console.WriteLine("[*] Bye!");

            Console.WriteLine();
        }
    }
}