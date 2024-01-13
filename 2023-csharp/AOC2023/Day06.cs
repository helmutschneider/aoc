using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text.RegularExpressions;

namespace AOC2023;

public record Race(long Time, long WinnerDistance);

public static class Day06
{
    public static IReadOnlyList<Race> ParsePartOne(string input)
    {
        var lines = input.Trim()
            .Split("\n")
            .Select(line => Regex.Split(line.Trim(), @"\s+"))
            .ToList();
        var races = new List<Race>();

        for (var i = 1; i < lines[0].Length; ++i)
        {
            var time = long.Parse(lines[0][i]);
            var dist = long.Parse(lines[1][i]);
            races.Add(new Race(time, dist));
        }

        return races;
    }

    public static Race ParsePartTwo(string input)
    {
        var lines = input.Trim()
            .Split("\n")
            .Select(line => line.Trim().Split(":")[1].Replace(" ", string.Empty).Trim())
            .ToList();
        var time = long.Parse(lines[0]);
        var dist = long.Parse(lines[1]);
        return new Race(time, dist);
    }

    public static long GetNumberOfWaysToWin(Race race)
    {
        var res = 0L;
        for (var i = 0; i < race.Time; ++i)
        {
            var speed = i;
            var travelTime = race.Time - i;
            var dist = speed * travelTime;

            if (dist > race.WinnerDistance) {
                res += 1;
            }
        }
        return res;
    }

    public static long Part1(string input)
    {
        var parsed = ParsePartOne(input);
        var product = 1L;
        foreach (var race in parsed)
        {
            product *= GetNumberOfWaysToWin(race);
        }
        return product;
    }

    public static long Part2(string input)
    {
        var parsed = ParsePartTwo(input);
        return GetNumberOfWaysToWin(parsed);
    }

    public const string INPUT = @"
Time:        40     92     97     90
Distance:   215   1064   1505   1100
    ";
}
