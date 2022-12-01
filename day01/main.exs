defmodule Utils do
  def to_integers(chunks) do
    chunks
      |> String.split("\n")
      |> Enum.map(&String.to_integer/1)
  end
end

max_element =
  File.read!("test.txt")
  |> String.trim()
  |> String.replace("\r", "")
  |> String.split("\n\n")
  |> Enum.map(&Utils.to_integers/1)
  |> Enum.map(&Enum.sum/1)
  |> Enum.max()

IO.inspect(max_element)
