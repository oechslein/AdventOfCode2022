defmodule Utils do
  def to_integers(chunks) do
    chunks
      |> String.split("\n")
      |> Enum.map(&String.to_integer/1)
  end
end

list =
  "input.txt"
  |> File.read!()
  |> String.trim()
  |> String.replace("\r", "")
  |> String.split("\n\n")
  |> Enum.map(&Utils.to_integers/1)
  |> Enum.map(&Enum.sum/1)


max_element =
  list
  |> Enum.max()

IO.inspect(max_element)

sum_1_2_3_max_element =
  list
  |> Enum.sort()
  |> Enum.take(-3)
  |> Enum.sum()

IO.inspect(sum_1_2_3_max_element)
