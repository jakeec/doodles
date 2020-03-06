using System;

namespace Result
{

    public interface IResult<T, E> where E : Exception
    {
        T unwrap();
    }

    public class Ok<T, E> : IResult<T, E> where E : Exception
    {
        T value;

        public Ok(T value)
        {
            this.value = value;
        }

        public T unwrap()
        {
            return this.value;
        }
    }

    public class Err<T, E> : IResult<T, E> where E : Exception
    {
        E exception;
        public Err(E e)
        {
            this.exception = e;
        }

        public T unwrap()
        {
            throw this.exception;
        }
    }

    public class Result<T>
    {
        public IResult<T, Exception> result { get; private set; }

        public Result(T value)
        {
            this.result = new Ok<T, Exception>(value);
        }

        public Result(Exception e)
        {
            this.result = new Err<T, Exception>(e);
        }

        public T unwrap()
        {
            return this.result.unwrap();
        }
    }
}