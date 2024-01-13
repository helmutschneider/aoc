using System;
using Xunit;

namespace AOC2023;

public class Day06Tests
{
    [Fact]
    public void ShouldParse()
    {
        var x = Day06.ParsePartOne(TEST_INPUT);
        Assert.Equal(3, x.Count);
    }

    [Fact]
    public void PartOneWithTestInput()
    {
        var x = Day06.Part1(TEST_INPUT);
        Console.WriteLine(x);
        Assert.Equal(288, x);
    }

    [Fact]
    public void PartTwoWithTestInput()
    {
        var x = Day06.Part2(TEST_INPUT);
        Console.WriteLine(x);
        Assert.Equal(71503, x);
    }

    [Fact]
    public void ThingPartOne()
    {
        var x = Day06.Part1(Day06.INPUT);
        Console.WriteLine(x);
        Assert.True(true);
    }

    [Fact]
    public void ThingPartTwo()
    {
        var x = Day06.Part2(Day06.INPUT);
        Console.WriteLine(x);
        Assert.True(true);
    }

    const string TEST_INPUT = @"
Time:      7  15   30
Distance:  9  40  200
    ";
}
