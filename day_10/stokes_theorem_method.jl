#!/usr/bin/env -S julia --color=yes --startup-file=no

using Printf, LinearAlgebra, Random

function ReadMap(fname::AbstractString)
	open(fname, "r") do io
		rows = map(collect, readlines(io))
		permutedims(hcat(rows...))
	end
end

const rotationMap = Dict{Tuple{Char, Char}, Char}(
	('E', '-') => 'E',
	('E', 'J') => 'N',
	('E', '7') => 'S',
	('N', '|') => 'N',
	('N', 'F') => 'E',
	('N', '7') => 'W',
	('W', '-') => 'W',
	('W', 'L') => 'N',
	('W', 'F') => 'S',
	('S', '|') => 'S',
	('S', 'J') => 'W',
	('S', 'L') => 'E'
)
global δMap = Dict{Char, Vector{Int64}}(
	'N' => [-1, 0],
	'E' => [0, +1],
	'S' => [+1, 0],
	'W' => [0, -1]
)

function IndexWithin(index::Vector{Int64}, pmap::Matrix{Char})
	"""Check that the index is within the matrix bounds."""
	all(map(d -> 1 ≤ index[d] ≤ size(pmap, d), 1:length(index)))
end

function GetIndex(pmap::Matrix{Char}, index::Vector{Int64})
	getindex(pmap, CartesianIndex(index...))
end

function StartDirections(start::Vector{Int64}, pmap::Matrix{Char})
	"""Find the possible directions to move from the starting
	point.

	We look at the current direction and value of the surrounding
	characters. The tuple `(dir, char)` will be a key in the
	`rotationMap` if the pipe is oriented correctly to travel
	down."""
	global δMap, rotationMap
	directions = map(collect("NESW")) do dir
		δ = getindex(δMap, dir)
		pos = start + δ
		if IndexWithin(pos, pmap)
			char = GetIndex(pmap, pos)
			if (dir, char) ∈ keys(rotationMap)
				dir
			end
		end
	end
	filter(!isnothing, directions)
end

mutable struct Scout
	"""State of the scout travelling through the pipes."""
	direction::Char
	position::Vector{Int64}
end

function Rotate!(scout::Scout, pmap::Matrix{Char})
	"""After moving to the next segment of pipe, rotate the
	`Scout` to follow it."""
	global rotationMap
	char = GetIndex(pmap, scout.position)
	scout.direction = getindex(rotationMap,
				   (scout.direction, char))
end

function Move!(scout::Scout)
	"""Move in the current direction of the `Scout`."""
	global δMap
	scout.position += getindex(δMap, scout.direction)
end

function Update!(scout::Scout, pmap::Matrix{Char})
	"""Update the `Scout` by moving to the next segment of pipe
	and rotating it to follow."""
	Move!(scout)
	Rotate!(scout, pmap)
end
Update!(pmap::Matrix{Char}) = scout -> Update!(scout, pmap)

function ChartLoop(pmap::Matrix{Char})
	"""Find the number of steps required to be furthest away from
	the starting point.

	This is done by evolving two `Scout`s away from the starting
	position in different directions. Once their positions agree
	we know we're at the furthest position.
	"""
	start = collect(Tuple(findfirst(c -> c == 'S', pmap)))
	directions = StartDirections(start, pmap)
	length(directions) != 2 && error("More than one possible loop!")

	steps = 0
	scouts = map(dir -> Scout(dir, start), directions)
	update! = Update!(pmap)
	map(update!, scouts)
	steps += 1

	positions::Vector{Vector{Vector{Int64}}} = [[], []]
	map(n -> push!(positions[n], scouts[n].position), 1:2)
	while scouts[1].position != scouts[2].position
		map(update!, scouts)
		map(n -> push!(positions[n], scouts[n].position), 1:2)
		steps += 1
	end
	p1, p2 = positions
	steps, vcat([start], p1, reverse(p2[1:end-1]))
end

function Displacements(loop::Vector{Vector{Int64}})
	"""Line elements dℓ for integral."""
	ℓ = length(loop)
	map(n -> loop[mod(n,ℓ)+1]-loop[n], 1:ℓ)
end

function VectorField(loop::Vector{Vector{Int64}}, α::Float64)
	"""A vector field on the loop whose curl is constant and
	points out of the plane.

	We are free to choose any field indexed by α∈[0,1] since it
	doesn't change the curl.
	"""
	map(p ->  α*[-p[2],0] + (1-α)*[0,+p[1]], loop)
end

function InteriorArea(loop::Vector{Vector{Int64}}, α::Float64)
	"""Interior area of the loop.

	By construction, the curl of the vector field is 1 and out of
	the plane. Stokes' theorem allows us to express the area as a
	line integral of the field around the loop."""
	V, dℓ = VectorField(loop, α), Displacements(loop)
	sum(map(n -> dot(V[n], dℓ[n]), 1:length(loop)))
end

function EnclosedTiles(loop::Vector{Vector{Int64}}, α::Float64=0.5)
	"""Tiles interior to the loop.

	To correct for the boundary we use a variant of Pick's
	theorem.
	"""
	ℓ = length(loop)
	A = abs(InteriorArea(loop, α))
	round(Int64, A - ℓ/2 + 1)
end

function Print(pmap::Matrix{Char})
	map(n -> println(string(pmap[n,:]...)), 1:size(pmap, 1))
end

function IndicateLoop!(pmap::Matrix{Char},
		       loop::Vector{Vector{Int64}}, char::Char='*')
	"""Replace all loop positions by a special character."""
	setindex!(pmap,
		  fill(char, length(loop)),
		  map(p -> CartesianIndex(p...), loop))
end

function PartOne(fname::AbstractString)
	"""Furthest loop position for AoC D010 Part One."""
	@printf("PART ONE (%s)\n", fname)
	pmap = ReadMap(fname)
	furthest, positions = ChartLoop(pmap)
	@printf("The furthest loop position is %d steps away.\n",
		furthest)
end

function PartTwo(fname::AbstractString)
	"""Enclosed tiles for AoC D010 Part Two."""
	@printf("PART TWO (%s)\n", fname)
	pmap = ReadMap(fname)
	furthest, loop = ChartLoop(pmap)
	enclosed = EnclosedTiles(loop, rand(Float64))
	@printf("The loop encloses %d tiles.\n", enclosed)
end

function Run(fname::AbstractString)
	PartOne(fname)
	PartTwo(fname)
end

Run("/Users/matthewbarbattini/Desktop/AdventOfCode2023/day_10/input.txt")