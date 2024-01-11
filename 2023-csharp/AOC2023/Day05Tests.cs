using System;
using Xunit;
using AOC2023;
using Xunit.Abstractions;

namespace AOC2023;

public class Day05Tests
{
    readonly ITestOutputHelper Output;

    public Day05Tests(ITestOutputHelper output)
    {
        this.Output = output;
    }

    [Theory]
    [InlineData(79, 82)]
    [InlineData(14, 43)]
    [InlineData(55, 86)]
    [InlineData(13, 35)]
    public void ShouldGetLocation(long seed, long expected)
    {
        var (_, maps) = Day05.Parse(TEST_INPUT, Day05.Part.One);
        var loc = Day05.GetLocation(seed, maps);

        Assert.Equal(expected, loc);
    }

    [Fact]
    public void PartOne()
    {
        var (seeds, maps) = Day05.Parse(Day05.INPUT, Day05.Part.One);
        var lowest = long.MaxValue;

        foreach (var num in seeds.Nums)
        {
            var loc = Day05.GetLocation(num.Start, maps);
            lowest = Math.Min(lowest, loc);
        }

        Output.WriteLine($"DAY05P1: {lowest}");
    }

    [Fact]
    public void PartTwo()
    {
        var (seeds, maps) = Day05.Parse(Day05.INPUT, Day05.Part.Two);
        var lowest = long.MaxValue;

        foreach (var range in seeds.Nums)
        {
            // FIXME: this is dogshit slow. we need a smarter way to do this.
            for (var i = range.Start; i < (range.Start + range.Length); ++i)
            {
                var loc = Day05.GetLocation(i, maps);
                lowest = Math.Min(lowest, loc);
            }
        }

        Output.WriteLine($"DAY05P2: {lowest}");
    }

    const string TEST_INPUT = @"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
    ";
}
