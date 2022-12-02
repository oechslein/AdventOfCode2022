defmodule Utils do
  def to_integers(chunks) do
    chunks
      |> String.split("\n")
      |> Enum.map(&String.to_integer/1)
  end
  def split_on_spaces(chunks) do
    chunks
      |> String.split(" ")
  end
  def enum_of_str_to_atoms(chunks) do
    chunks
      |> Enum.map(&String.to_atom/1)
  end
end

defmodule Day02 do
  def puzzle1_transform(move_list) do
    Enum.map(move_list, &puzzle1_transform_move/1)
  end

  def puzzle1_transform_move(move) do
    case move do
      :A -> :ROCK
      :B -> :PAPER
      :C -> :SCISSORS
      :X -> :ROCK
      :Y -> :PAPER
      :Z -> :SCISSORS
      _ -> move
    end
  end

  def puzzle2_transform(move_list) do
    [opponent_move_atom, wished_outcome_atom] = move_list
    opponent_move = puzzle1_transform_move(opponent_move_atom)
    player_move =
      case [opponent_move, wished_outcome_atom] do
        [_, :Y] -> opponent_move
        [:ROCK, :Z] -> :PAPER
        [:SCISSORS, :Z] -> :ROCK
        [:PAPER, :Z] -> :SCISSORS
        [:ROCK, :X] -> :SCISSORS
        [:SCISSORS, :X] -> :PAPER
        [:PAPER, :X] -> :ROCK
      end

    [opponent_move, player_move]
    end

    def score_move_list(move_list) do
      score_move = case Enum.at(move_list, 1) do
        :ROCK -> 1
        :PAPER -> 2
        :SCISSORS -> 3
      end
      score_move_list = case move_list do
        [x, x] -> 3
        [:SCISSORS, :ROCK] -> 6
        [:PAPER, :SCISSORS] -> 6
        [:ROCK, :PAPER] -> 6
        _ -> 0
      end
      score_move + score_move_list
    end
  end

list =
  "input.txt"
  |> File.read!()
  |> String.trim()
  |> String.replace("\r", "")
  |> String.split("\n")
  |> Enum.map(&Utils.split_on_spaces/1)
  |> Enum.map(&Utils.enum_of_str_to_atoms/1)

  result_puzzle_1 =
    list
    |> Enum.map(&Day02.puzzle1_transform/1)
    |> Enum.map(&Day02.score_move_list/1)
    |> Enum.sum()

  IO.inspect(result_puzzle_1)

  result_puzzle_2 =
    list
    |> Enum.map(&Day02.puzzle2_transform/1)
    |> Enum.map(&Day02.score_move_list/1)
    |> Enum.sum()

    IO.inspect(result_puzzle_2)
