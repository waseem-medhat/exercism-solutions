defmodule Secrets do
  def secret_add(secret) do
    fn other -> other + secret end
  end

  def secret_subtract(secret) do
    fn other -> other - secret end
  end

  def secret_multiply(secret) do
    fn other -> other * secret end
  end

  def secret_divide(secret) do
    fn other -> div(other, secret) end
  end

  def secret_and(secret) do
    fn other -> Bitwise.band(other, secret) end
  end

  def secret_xor(secret) do
    fn other -> Bitwise.bxor(other, secret) end
  end

  def secret_combine(secret_function1, secret_function2) do
    fn other -> other |> secret_function1.() |> secret_function2.() end
  end
end
