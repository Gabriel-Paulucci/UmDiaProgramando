using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Running;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Benckmark
{
    class Program
    {
        static void Main(string[] args)
        {
            var summary = BenchmarkRunner.Run(typeof(Program).Assembly);
        }
    }

    [MemoryDiagnoser]
    [SimpleJob]
    public class TesteAdd
    {
        private Pessoa[] pessoasArray;

        [Params(100, 1000, 10000)]
        public int Quantidade;

        [GlobalSetup]
        public void SetupAdd()
        {
            pessoasArray = new Pessoa[Quantidade];

            for (int i = 0; i < Quantidade; i++)
            {
                pessoasArray[i] = new Pessoa(i);
            }
        }

        [Benchmark]
        public void ListAdd()
        {
            List<Pessoa> pessoas = new(Quantidade);

            for (int i = 0; i < Quantidade; i++)
            {
                pessoas.Add(pessoasArray[i]);
            }
        }

        [Benchmark]
        public void SortedListAdd()
        {
            SortedList<int, Pessoa> pessoas = new();

            for (int i = 0; i < Quantidade; i++)
            {
                pessoas.Add(i, pessoasArray[i]);
            }
        }

        [Benchmark]
        public void DictionaryAdd()
        {
            Dictionary<int, Pessoa> pessoas = new();

            for (int i = 0; i < Quantidade; i++)
            {
                pessoas.Add(i, pessoasArray[i]);
            }
        }

        [Benchmark]
        public void ListTupleAdd()
        {
            List<ValueTuple<int, Pessoa>> pessoas = new();

            for (int i = 0; i < Quantidade; i++)
            {
                pessoas.Add(new (i, pessoasArray[i]));
            }
        }
    }

    [MemoryDiagnoser]
    [SimpleJob]
    public class TesteFind
    {
        [Params(100, 1000, 10000)]
        public int Quantidade;

        private List<Pessoa> pessoasList;
        private SortedList<int, Pessoa> pessoasSortedList;
        private Dictionary<int, Pessoa> pessoasDictionary;
        private List<ValueTuple<int, Pessoa>> pessoasListTuple;

        [GlobalSetup]
        public void SetupList()
        {
            var pessoasArray = new Pessoa[Quantidade];

            for (int i = 0; i < Quantidade; i++)
            {
                pessoasArray[i] = new Pessoa(i);
            }

            pessoasList = new();

            for (int i = 0; i < Quantidade; i++)
            {
                pessoasList.Add(pessoasArray[i]);
            }

            pessoasSortedList = new();

            for (int i = 0; i < Quantidade; i++)
            {
                pessoasSortedList.Add(i, pessoasArray[i]);
            }

            pessoasDictionary = new();

            for (int i = 0; i < Quantidade; i++)
            {
                pessoasDictionary.Add(i, pessoasArray[i]);
            }

            pessoasListTuple = new();

            for (int i = 0; i < Quantidade; i++)
            {
                pessoasListTuple.Add(new(i, pessoasArray[i]));
            }
        }

        [Benchmark]
        public void ListFind()
        {
            var pessoa = pessoasList.Find(x => x.Id == Quantidade);
        }

        [Benchmark]
        public void SortedListFind()
        {
            pessoasSortedList.TryGetValue(Quantidade - 1, out var pessoa);
        }

        [Benchmark]
        public void DictionaryFind()
        {
            pessoasDictionary.TryGetValue(Quantidade - 1, out var pessoa);
        }

        [Benchmark]
        public void ListTupleFind()
        {
            var pessoa = pessoasListTuple.Find(x => x.Item1 == Quantidade);
        }
    }

    record Pessoa(int Id);
}
