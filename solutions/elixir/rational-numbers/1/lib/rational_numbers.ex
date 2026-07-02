defmodule RationalNumbers do
  @type rational :: {integer, integer}

  @doc """
  Add two rational numbers
  """
  @spec add(r1 :: rational, r2 :: rational) :: rational
  def add({a1, b1}, {a2, b2}),
    do: {a1 * b2 + a2 * b1, b1 * b2} |> reduce()

  @doc """
  Subtract two rational numbers
  """
  @spec subtract(r1 :: rational, r2 :: rational) :: rational
  def subtract({a1, b1}, {a2, b2}),
    do: {a1 * b2 - a2 * b1, b1 * b2} |> reduce()

  @doc """
  Multiply two rational numbers
  """
  @spec multiply(r1 :: rational, r2 :: rational) :: rational
  def multiply({a1, b1}, {a2, b2}),
    do: {a1 * a2, b1 * b2} |> reduce()

  @doc """
  Divide two rational numbers
  """
  @spec divide_by(num :: rational, den :: rational) :: rational
  def divide_by({a1, b1}, {a2, b2}),
    do: {a1 * b2, b1 * a2} |> reduce()

  @doc """
  Absolute value of a rational number
  """
  @spec abs(a :: rational) :: rational
  def abs({a, b}),
    do: {Kernel.abs(a), Kernel.abs(b)} |> reduce()

  @doc """
  Exponentiation of a rational number by an integer
  """
  @spec pow_rational(r :: rational, n :: integer) :: rational
  def pow_rational({a, b}, n) when n >= 0,
    do: {a ** n, b ** n} |> reduce()

  def pow_rational({a, b}, n),
    do: {b ** Kernel.abs(n), a ** Kernel.abs(n)} |> reduce()

  @doc """
  Exponentiation of a real number by a rational number
  """
  @spec pow_real(x :: integer, n :: rational) :: float
  def pow_real(x, {a, b}), do: x ** a ** (1 / b)

  @doc """
  Reduce a rational number to its lowest terms
  """
  @spec reduce(r :: rational) :: rational
  def reduce(r), do: r |> reduce_gcd() |> reduce_neg()

  defp reduce_neg({a, b}) when b < 0, do: {-a, -b}
  defp reduce_neg({a, b}), do: {a, b}

  defp reduce_gcd({a, b}),
    do: {a / Integer.gcd(a, b), b / Integer.gcd(a, b)}
end

