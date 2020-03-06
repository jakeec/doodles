using System;
using Result;

namespace csharp
{
    class Program
    {
        static void Main(string[] args)
        {
            var firstResult = Add(2, 4);
            Console.WriteLine(firstResult);

            // Have to wrap this in a try catch because function might throw an error without my knowledge
            try
            {
                var secondResult = Add(3, 4);
            }
            catch (Exception e)
            {
                Console.WriteLine(e.Message);
            }

            // Same thing with Result return type 
            var thirdResult = MaybeAdd(2, 3);
            var fourthResult = MaybeAdd(3, 2);

            if (thirdResult.result is Ok<int, Exception>)
            {
                Console.WriteLine(thirdResult.unwrap());
            }
            else
            {

            }

            if (fourthResult.result is Err<int, Exception>)
            {
                Console.WriteLine("There was an error!");
            }
            else
            {
                Console.WriteLine(fourthResult.unwrap());
            }
        }

        // This can throw an error but the caller has no idea
        static int Add(int a, int b)
        {
            if (!(a % 2 == 0))
            {
                throw new Exception("a must be even!");
            }
            return a + b;
        }

        static Result<int> MaybeAdd(int a, int b)
        {
            if (!(a % 2 == 0))
            {
                return new Result<int>(new Exception("A should be even"));
            }
            return new Result<int>(a + b);
        }
    }
}
